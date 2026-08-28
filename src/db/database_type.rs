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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_url_postgres() {
        assert_eq!(DatabaseType::from_url("postgres://user:pass@localhost/db"), DatabaseType::Postgres);
        assert_eq!(DatabaseType::from_url("POSTGRES://user:pass@localhost/db"), DatabaseType::Postgres);
        assert_eq!(DatabaseType::from_url("postgresql://user:pass@localhost/db"), DatabaseType::Postgres);
        assert_eq!(DatabaseType::from_url("POSTGRESQL://user:pass@localhost/db"), DatabaseType::Postgres);
    }

    #[test]
    fn test_from_url_mysql() {
        assert_eq!(DatabaseType::from_url("mysql://user:pass@localhost/db"), DatabaseType::MySql);
        assert_eq!(DatabaseType::from_url("MYSQL://user:pass@localhost/db"), DatabaseType::MySql);
        assert_eq!(DatabaseType::from_url("mariadb://user:pass@localhost/db"), DatabaseType::MySql);
        assert_eq!(DatabaseType::from_url("MARIADB://user:pass@localhost/db"), DatabaseType::MySql);
    }

    #[test]
    fn test_from_url_sqlite() {
        assert_eq!(DatabaseType::from_url("sqlite://app.db"), DatabaseType::Sqlite);
        assert_eq!(DatabaseType::from_url("SQLITE://app.db"), DatabaseType::Sqlite);
        assert_eq!(DatabaseType::from_url("sqlite::memory:"), DatabaseType::Sqlite);
        // Fallback case
        assert_eq!(DatabaseType::from_url("unknown://user:pass@localhost/db"), DatabaseType::Sqlite);
        assert_eq!(DatabaseType::from_url("just_a_string"), DatabaseType::Sqlite);
    }
}
