use serde::{Deserialize, Serialize};

/// Global configuration for the Oxide_CG application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxideConfig {
    pub site_name: String,
    pub bind_address: String,
    pub database_url: String,
    pub max_db_connections: u32,
    pub session_duration_days: i64,
    pub enable_cors: bool,
    pub enable_gzip: bool,
}

impl Default for OxideConfig {
    fn default() -> Self {
        Self {
            site_name: "Oxide_CG".to_string(),
            bind_address: "0.0.0.0:8080".to_string(),
            database_url: "sqlite://oxide_cg.db?mode=rwc".to_string(),
            max_db_connections: 25,
            session_duration_days: 7,
            enable_cors: true,
            enable_gzip: true,
        }
    }
}

impl OxideConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn site_name(mut self, name: impl Into<String>) -> Self {
        self.site_name = name.into();
        self
    }

    pub fn bind(mut self, addr: impl Into<String>) -> Self {
        self.bind_address = addr.into();
        self
    }

    pub fn database(mut self, url: impl Into<String>) -> Self {
        self.database_url = url.into();
        self
    }

    pub fn max_db_connections(mut self, count: u32) -> Self {
        self.max_db_connections = count;
        self
    }
}
