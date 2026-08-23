use crate::core::error::OxideError;
use crate::model::{FieldType, ModelSchema};
use sqlx::{Pool, Sqlite};
use tracing::info;

pub struct SchemaMigrator;

impl SchemaMigrator {
    /// Initialize internal system tables for Oxide_CG
    pub async fn migrate_system_tables(pool: &Pool<Sqlite>) -> Result<(), OxideError> {
        // Users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS _oxide_users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'Admin',
                is_active INTEGER NOT NULL DEFAULT 1,
                otp_secret TEXT,
                otp_enabled INTEGER NOT NULL DEFAULT 0,
                last_login TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            "#
        )
        .execute(pool)
        .await?;

        // Sessions table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS _oxide_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token TEXT UNIQUE NOT NULL,
                user_id INTEGER NOT NULL,
                ip_address TEXT,
                user_agent TEXT,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY(user_id) REFERENCES _oxide_users(id) ON DELETE CASCADE
            );
            "#
        )
        .execute(pool)
        .await?;

        // Audit Logs table (Tracks all mutations with JSON diffs & snapshots)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS _oxide_audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                model_name TEXT NOT NULL,
                record_id INTEGER NOT NULL,
                action TEXT NOT NULL,
                user_id INTEGER,
                username TEXT,
                changes_json TEXT NOT NULL,
                snapshot_json TEXT NOT NULL,
                ip_address TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            "#
        )
        .execute(pool)
        .await?;

        // Approvals table (Tracks sensitive field change requests)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS _oxide_approvals (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                model_name TEXT NOT NULL,
                record_id INTEGER NOT NULL,
                field_name TEXT NOT NULL,
                old_value TEXT,
                new_value TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'Pending',
                requested_by_id INTEGER,
                requested_by_username TEXT,
                reviewed_by_id INTEGER,
                reviewed_by_username TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            "#
        )
        .execute(pool)
        .await?;

        info!("System tables verified and auto-migrated");
        Ok(())
    }

    /// Auto-migrate user domain model to database
    pub async fn migrate_model(pool: &Pool<Sqlite>, schema: &ModelSchema) -> Result<(), OxideError> {
        let mut col_defs = Vec::new();

        for field in &schema.fields {
            if field.name == "id" {
                col_defs.push("id INTEGER PRIMARY KEY AUTOINCREMENT".to_string());
                continue;
            }

            let sql_type = match &field.field_type {
                FieldType::Integer | FieldType::ForeignKey { .. } => "INTEGER",
                FieldType::Float | FieldType::Money { .. } | FieldType::ProgressBar { .. } => "REAL",
                FieldType::Boolean => "BOOLEAN",
                FieldType::DateTime => "TEXT",
                FieldType::Json => "TEXT",
                _ => "TEXT",
            };

            let mut def = format!("\"{}\" {}", field.name, sql_type);

            if field.required {
                def.push_str(" NOT NULL");
            }

            if field.unique {
                def.push_str(" UNIQUE");
            }

            if let Some(ref default) = field.default_value {
                match default {
                    serde_json::Value::Bool(b) => def.push_str(&format!(" DEFAULT {}", if *b { 1 } else { 0 })),
                    serde_json::Value::Number(n) => def.push_str(&format!(" DEFAULT {}", n)),
                    serde_json::Value::String(s) => def.push_str(&format!(" DEFAULT '{}'", s.replace('\'', "''"))),
                    _ => {}
                }
            } else if field.name == "created_at" || field.name == "updated_at" {
                def.push_str(" DEFAULT (datetime('now'))");
            }

            col_defs.push(def);
        }

        let ddl = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" (\n    {}\n);",
            schema.table_name,
            col_defs.join(",\n    ")
        );

        sqlx::query(&ddl).execute(pool).await?;

        // Create indexes on searchable and filterable fields
        for field in &schema.fields {
            if field.name != "id" && (field.searchable || field.unique) {
                let idx_name = format!("idx_{}_{}", schema.table_name, field.name);
                let idx_sql = format!(
                    "CREATE INDEX IF NOT EXISTS \"{}\" ON \"{}\" (\"{}\");",
                    idx_name, schema.table_name, field.name
                );
                let _ = sqlx::query(&idx_sql).execute(pool).await;
            }
        }

        info!("Model table '{}' ready", schema.table_name);
        Ok(())
    }
}
