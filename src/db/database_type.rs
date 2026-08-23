use serde::{Deserialize, Serialize};

/// Supported database types for Oxide_CG from lightweight embedded to enterprise-grade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseType {
    Sqlite,
    Postgres,
    MySql,
}

impl DatabaseType {
    /// Detect database type from connection URI
    pub fn from_url(url: &str) -> Self {
        let lower = url.to_lowercase();
        if lower.starts_with("postgres://") || lower.starts_with("postgresql://") {
            DatabaseType::Postgres
        } else if lower.starts_with("mysql://") || lower.starts_with("mariadb://") {
            DatabaseType::MySql
        } else {
            DatabaseType::Sqlite
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Sqlite => "SQLite (WAL Mode)",
            Self::Postgres => "PostgreSQL (Enterprise)",
            Self::MySql => "MySQL / MariaDB (Enterprise)",
        }
    }

    pub fn is_postgres(&self) -> bool {
        matches!(self, Self::Postgres)
    }

    pub fn is_mysql(&self) -> bool {
        matches!(self, Self::MySql)
    }

    pub fn is_sqlite(&self) -> bool {
        matches!(self, Self::Sqlite)
    }
}
