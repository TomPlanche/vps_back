//! Pagination utilities for API endpoints

use serde::{Deserialize, Serialize};

/// Query parameters for pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: u32,

    /// Number of items per page
    #[serde(default = "default_limit")]
    pub limit: u32,
}

const fn default_page() -> u32 {
    1
}

const fn default_limit() -> u32 {
    20
}

/// Maximum number of items per page
const MAX_LIMIT: u32 = 100;

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
        }
    }
}

impl PaginationParams {
    /// Calculate the offset for database queries (0-indexed)
    #[must_use]
    pub fn offset(&self) -> u64 {
        u64::from((self.page.saturating_sub(1)) * self.limit)
    }

    /// Get the limit as u64 for database queries
    #[must_use]
    pub fn limit_u64(&self) -> u64 {
        u64::from(self.limit)
    }

    /// Validate pagination parameters
    pub const fn validate(&mut self) {
        // Ensure page is at least 1
        if self.page == 0 {
            self.page = 1;
        }

        // Cap limit to a reasonable maximum
        if self.limit == 0 {
            self.limit = default_limit();
        } else if self.limit > MAX_LIMIT {
            self.limit = MAX_LIMIT;
        }
    }
}
