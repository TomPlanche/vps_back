use anyhow::Context;
use axum::{
    Json,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use std::collections::HashMap;
use tracing::info;

use crate::{
    data_response,
    entities::{brew_downloads, prelude::*},
    error::{ApiError, ApiResult},
};

/// Returns the GitHub org and repo for a known project, or `None` if the project is unknown.
fn github_org_repo(project: &str) -> Option<(&'static str, &'static str)> {
    match project {
        "rona" => Some(("rona-rs", "rona")),
        "clean-dev-dirs" => Some(("clean-dev-dirs", "clean-dev-dirs")),
        _ => None,
    }
}

/// Parses a Homebrew bottle filename into `(version, platform)`.
///
/// Expected format: `{project}-{version}.{platform}.bottle.tar.gz`
/// Example: `rona-2.17.7.arm64_sequoia.bottle.tar.gz`
fn parse_brew_filename(project: &str, filename: &str) -> Option<(String, String)> {
    let base = filename.strip_suffix(".bottle.tar.gz")?;
    let dot_pos = base.rfind('.')?;
    let platform = base[dot_pos + 1..].to_string();
    let name_version = &base[..dot_pos];
    let prefix = format!("{project}-");
    let version = name_version.strip_prefix(&prefix)?.to_string();
    Some((version, platform))
}

/// Handles GET requests to track a Homebrew bottle download and redirect to the real asset.
///
/// Homebrew sets this server as `root_url` in the bottle block. When a user runs
/// `brew install <formula>`, Homebrew fetches:
///   `{root_url}/{filename}`
/// which hits this endpoint. The server records the download and issues a 302 redirect
/// to the actual GitHub release asset.
///
/// # Path parameters
/// * `project` - The formula/project name (e.g. `rona`, `clean-dev-dirs`)
/// * `filename` - The bottle filename (e.g. `rona-2.17.7.arm64_sequoia.bottle.tar.gz`)
///
/// # Returns
/// A 302 redirect to the GitHub release asset URL.
///
/// # Errors
/// * 404 if the project is not recognised
/// * 400 if the filename cannot be parsed
/// * 500 on database or header-value errors
pub async fn track_brew_download(
    State(db): State<DatabaseConnection>,
    Path((project, filename)): Path<(String, String)>,
) -> ApiResult<Response> {
    info!("GET `/brew/track/{project}/{filename}` endpoint called");

    let (org, repo) = github_org_repo(&project)
        .ok_or_else(|| ApiError::not_found(format!("Unknown project: {project}")))?;

    let (version, platform) = parse_brew_filename(&project, &filename)
        .ok_or_else(|| ApiError::validation(format!("Could not parse filename: {filename}")))?;

    // Upsert: increment count if row exists, insert with count=1 otherwise.
    let existing = BrewDownloads::find()
        .filter(brew_downloads::Column::Project.eq(&project))
        .filter(brew_downloads::Column::Version.eq(&version))
        .filter(brew_downloads::Column::Platform.eq(&platform))
        .one(&db)
        .await
        .context("Failed to query brew download record")?;

    if let Some(model) = existing {
        let mut active: brew_downloads::ActiveModel = model.into();
        active.count = Set(active.count.unwrap() + 1);
        active
            .update(&db)
            .await
            .context("Failed to update brew download count")?;
    } else {
        let new_record = brew_downloads::ActiveModel {
            project: Set(project.clone()),
            version: Set(version.clone()),
            platform: Set(platform),
            count: Set(1),
            ..Default::default()
        };
        new_record
            .insert(&db)
            .await
            .context("Failed to insert brew download record")?;
    }

    let redirect_url =
        format!("https://github.com/{org}/{repo}/releases/download/v{version}/{filename}");

    let location =
        header::HeaderValue::from_str(&redirect_url).context("Failed to build Location header")?;

    Ok((StatusCode::FOUND, [(header::LOCATION, location)]).into_response())
}

/// Aggregated per-project download stats, keyed by version then summed at the top level.
struct ProjectStats {
    total: i64,
    versions: HashMap<String, i64>,
}

/// Handles GET requests for global Homebrew download statistics.
///
/// # Returns
/// JSON object keyed by project name, containing `total_downloads`, `total_installs`,
/// and per-version `downloads`/`installs` counts.
///
/// # Errors
/// Returns 500 on database failure.
pub async fn get_brew_stats(
    State(db): State<DatabaseConnection>,
) -> ApiResult<Json<serde_json::Value>> {
    info!("GET `/brew/stats` endpoint called");

    let rows = BrewDownloads::find()
        .all(&db)
        .await
        .context("Failed to fetch brew downloads from database")?;

    let mut stats: HashMap<String, ProjectStats> = HashMap::new();

    for row in rows {
        let entry = stats.entry(row.project.clone()).or_insert_with(|| ProjectStats {
            total: 0,
            versions: HashMap::new(),
        });
        let count = i64::from(row.count);
        entry.total += count;
        *entry.versions.entry(row.version.clone()).or_insert(0) += count;
    }

    let mut result = serde_json::Map::new();

    for (project_name, project_stats) in stats {
        let mut obj = serde_json::Map::new();
        obj.insert("total_downloads".to_string(), json!(project_stats.total));
        obj.insert("total_installs".to_string(), json!(project_stats.total));
        for (version, count) in project_stats.versions {
            obj.insert(
                version,
                json!({
                    "downloads": count,
                    "installs": count,
                }),
            );
        }
        result.insert(project_name, serde_json::Value::Object(obj));
    }

    Ok(data_response(serde_json::Value::Object(result)))
}
