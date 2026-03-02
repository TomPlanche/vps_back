use serde::{Deserialize, Serialize};

// Output models (serialized to the JSON file)

#[derive(Debug, Serialize)]
pub struct GithubStatsFile<'a> {
    pub last_updated_at: String,
    pub repos: &'a [RepoStats],
}

#[derive(Debug, Serialize)]
pub struct RepoStats {
    pub name: String,
    pub link: String,
    pub stars: u64,
    pub owner: String,
    pub description: Option<String>,
    pub commits: CommitStats,
}

#[derive(Debug, Serialize)]
pub struct CommitStats {
    pub total: u64,
    pub last: LastCommit,
}

#[derive(Debug, Serialize)]
pub struct LastCommit {
    pub hash: String,
    pub link: String,
    pub date: String,
}

// GitHub API response models (deserialized from GitHub responses)

#[derive(Debug, Deserialize)]
pub struct GhRepo {
    pub name: String,
    pub html_url: String,
    pub stargazers_count: u64,
    pub description: Option<String>,
    pub owner: GhOwner,
}

#[derive(Debug, Deserialize)]
pub struct GhOwner {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct GhCommit {
    pub sha: String,
    pub html_url: String,
    pub commit: GhCommitInner,
}

#[derive(Debug, Deserialize)]
pub struct GhCommitInner {
    pub author: GhAuthor,
}

#[derive(Debug, Deserialize)]
pub struct GhAuthor {
    pub date: String,
}
