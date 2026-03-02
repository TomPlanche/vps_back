use anyhow::Context;
use axum::{Extension, Json, extract::State};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use serde_json::{Value, json};
use std::collections::HashMap;
use tracing::info;

use crate::{
    data_response,
    entities::{prelude::*, sources},
    error::ApiResult,
};

pub async fn get_github_stats(
    Extension(static_dir): Extension<String>,
) -> Json<Value> {
    info!("GET `/stats/github` endpoint called");

    let path = format!("{static_dir}/github-stats.json");

    let data = tokio::fs::read_to_string(&path)
        .await
        .map_or(Value::Null, |contents| {
            serde_json::from_str(&contents).unwrap_or(Value::Null)
        });

    data_response(data)
}

pub async fn get_brew_stats(
    State(db): State<DatabaseConnection>,
) -> ApiResult<Json<Value>> {
    info!("GET `/stats/brew` endpoint called");

    let rows = BrewDownloads::find()
        .all(&db)
        .await
        .context("Failed to fetch brew downloads")?;

    let mut stats: HashMap<String, (i64, HashMap<String, i64>)> = HashMap::new();

    for row in rows {
        let entry = stats.entry(row.project).or_default();
        let count = i64::from(row.count);
        entry.0 += count;
        *entry.1.entry(row.version).or_insert(0) += count;
    }

    let mut result = serde_json::Map::new();
    for (project, (total, versions)) in stats {
        let mut obj = serde_json::Map::new();
        obj.insert("total_downloads".to_string(), json!(total));
        for (version, count) in versions {
            obj.insert(version, json!({ "downloads": count }));
        }
        result.insert(project, Value::Object(obj));
    }

    Ok(data_response(Value::Object(result)))
}

pub async fn get_source_stats(
    State(db): State<DatabaseConnection>,
) -> ApiResult<Json<Value>> {
    info!("GET `/stats/sources` endpoint called");

    let rows = Sources::find()
        .order_by_asc(sources::Column::Name)
        .all(&db)
        .await
        .context("Failed to fetch sources")?;

    let mut map = serde_json::Map::new();
    for row in rows {
        map.insert(row.name, json!(row.count));
    }

    Ok(data_response(Value::Object(map)))
}
