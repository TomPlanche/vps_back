use anyhow::{Context, Result};
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT},
};

use super::models::{CommitStats, GhCommit, GhRepo, LastCommit, RepoStats};

pub fn build_client(token: &str) -> Result<Client> {
    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}"))
            .context("Failed to build Authorization header")?,
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("vps-back"),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28"),
    );

    Client::builder()
        .default_headers(headers)
        .build()
        .context("Failed to build reqwest client")
}

pub async fn fetch_stats(client: &Client) -> Result<Vec<RepoStats>> {
    let repos: Vec<GhRepo> = client
        .get("https://api.github.com/user/repos")
        .query(&[("affiliation", "owner,collaborator"), ("per_page", "6"), ("sort", "updated")])
        .send()
        .await
        .context("Failed to fetch repos")?
        .error_for_status()
        .context("GitHub API returned error status for repos")?
        .json()
        .await
        .context("Failed to deserialize repos")?;

    let mut stats = Vec::with_capacity(repos.len());

    for repo in repos {
        let owner = repo.owner.login.clone();
        let name = repo.name.clone();

        let commit_stats = fetch_commit_stats(client, &owner, &name)
            .await
            .with_context(|| format!("Failed to fetch commits for {owner}/{name}"))?;

        stats.push(RepoStats {
            name: repo.name,
            link: repo.html_url,
            stars: repo.stargazers_count,
            owner,
            description: repo.description,
            commits: commit_stats,
        });
    }

    Ok(stats)
}

async fn fetch_commit_stats(client: &Client, owner: &str, repo: &str) -> Result<CommitStats> {
    let response = client
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/commits"
        ))
        .query(&[("per_page", "1")])
        .send()
        .await
        .context("Failed to fetch commits")?
        .error_for_status()
        .context("GitHub API returned error status for commits")?;

    let link_header = response
        .headers()
        .get("link")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let total = parse_total_from_link(link_header.as_deref());

    let commits: Vec<GhCommit> = response
        .json()
        .await
        .context("Failed to deserialize commits")?;

    let first = commits
        .into_iter()
        .next()
        .context("No commits returned for repo")?;

    let hash = first.sha[..7.min(first.sha.len())].to_string();

    Ok(CommitStats {
        total,
        last: LastCommit {
            hash,
            link: first.html_url,
            date: first.commit.author.date,
        },
    })
}

fn parse_total_from_link(header: Option<&str>) -> u64 {
    let Some(header) = header else {
        return 1;
    };

    // The Link header looks like:
    // <url?page=2>; rel="next", <url?page=42>; rel="last"
    for part in header.split(',') {
        let part = part.trim();
        if !part.contains(r#"rel="last""#) {
            continue;
        }

        // Extract URL portion between < and >
        let Some(url_part) = part.split(';').next() else {
            continue;
        };
        let url_part = url_part.trim();

        let url = url_part.trim_start_matches('<').trim_end_matches('>');

        // Extract page= query param
        for param in url.split('&') {
            if let Some(Ok(n)) = param.strip_prefix("page=").map(str::parse::<u64>) {
                return n;
            }
        }
    }

    1
}
