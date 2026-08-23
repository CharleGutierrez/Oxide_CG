use crate::core::error::OxideError;
use crate::db::adapter::DatabaseAdapter;
use crate::model::ModelSchema;
use async_trait::async_trait;
use serde_json::{Map, Value};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Column, Pool, Row, Sqlite, TypeInfo, ValueRef,
};
use std::str::FromStr;
use tracing::info;

/// High-performance SQLite database engine with WAL mode and PRAGMA tuning.
#[derive(Clone)]
pub struct SqliteDatabase {
    pub pool: Pool<Sqlite>,
}

impl SqliteDatabase {
    /// Connect to SQLite and apply performance PRAGMAs
    pub async fn connect(url: &str, max_connections: u32) -> Result<Self, OxideError> {
        let opts = SqliteConnectOptions::from_str(url)?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(std::time::Duration::from_secs(5));

        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect_with(opts)
            .await?;

        // Apply additional runtime performance tuning
        let _ = sqlx::query("PRAGMA cache_size = -64000;").execute(&pool).await;
        let _ = sqlx::query("PRAGMA foreign_keys = ON;").execute(&pool).await;

        info!("⚡ SQLite connection pool established in WAL mode (max_conns: {})", max_connections);
        Ok(Self { pool })
    }

    /// Convert a SqliteRow into dynamic JSON object with strict type preservation
    pub fn row_to_json(row: &sqlx::sqlite::SqliteRow) -> Value {
        let mut map = Map::new();
        for col in row.columns() {
            let col_name = col.name();
            let val_ref = row.try_get_raw(col_name);

            let json_val = match val_ref {
                Ok(raw) if raw.is_null() => Value::Null,
                Ok(_) => {
                    let type_name = col.type_info().name();
                    match type_name {
                        "INTEGER" | "INT" | "BIGINT" | "TINYINT" => {
                            if let Ok(v) = row.try_get::<i64, _>(col_name) {
                                Value::Number(v.into())
                            } else if let Ok(v) = row.try_get::<bool, _>(col_name) {
                                Value::Bool(v)
                            } else {
                                Value::Null
                            }
                        }
                        "REAL" | "FLOAT" | "DOUBLE" | "NUMERIC" => {
                            if let Ok(v) = row.try_get::<f64, _>(col_name) {
                                serde_json::Number::from_f64(v)
                                    .map(Value::Number)
                                    .unwrap_or(Value::Null)
                            } else {
                                Value::Null
                            }
                        }
                        "BOOLEAN" | "BOOL" => {
                            if let Ok(v) = row.try_get::<bool, _>(col_name) {
                                Value::Bool(v)
                            } else if let Ok(v) = row.try_get::<i64, _>(col_name) {
                                Value::Bool(v != 0)
                            } else {
                                Value::Null
                            }
                        }
                        _ => {
                            if let Ok(v) = row.try_get::<String, _>(col_name) {
                                if (v.starts_with('{') && v.ends_with('}'))
                                    || (v.starts_with('[') && v.ends_with(']'))
                                {
                                    serde_json::from_str(&v).unwrap_or(Value::String(v))
                                } else {
                                    Value::String(v)
                                }
                            } else {
                                Value::Null
                            }
                        }
                    }
                }
                Err(_) => Value::Null,
            };
            map.insert(col_name.to_string(), json_val);
        }
        Value::Object(map)
    }

    /// Parameter binding helper
    pub fn bind_json_value<'a>(
        query: sqlx::query::Query<'a, Sqlite, sqlx::sqlite::SqliteArguments<'a>>,
        val: &Value,
    ) -> sqlx::query::Query<'a, Sqlite, sqlx::sqlite::SqliteArguments<'a>> {
        match val {
            Value::Null => query.bind(Option::<String>::None),
            Value::Bool(b) => query.bind(if *b { 1i64 } else { 0i64 }),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    query.bind(i)
                } else if let Some(f) = n.as_f64() {
                    query.bind(f)
                } else {
                    query.bind(n.to_string())
                }
            }
            Value::String(s) => query.bind(s.clone()),
            Value::Array(_) | Value::Object(_) => query.bind(val.to_string()),
        }
    }
}

#[async_trait]
impl DatabaseAdapter for SqliteDatabase {
    async fn get_by_id(&self, schema: &ModelSchema, id: i64) -> Result<Option<Value>, OxideError> {
        let sql = format!("SELECT * FROM \"{}\" WHERE id = ? LIMIT 1", schema.table_name);
        let row_opt = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row_opt.as_ref().map(Self::row_to_json))
    }

    async fn insert(
        &self,
        schema: &ModelSchema,
        payload: &Map<String, Value>,
    ) -> Result<Value, OxideError> {
        let mut cols = Vec::new();
        let mut placeholders = Vec::new();
        let mut values_to_bind = Vec::new();

        for field in &schema.fields {
            if field.name == "id" || field.name == "created_at" || field.name == "updated_at" {
                continue;
            }

            if let Some(val) = payload.get(&field.name) {
                cols.push(format!("\"{}\"", field.name));
                placeholders.push("?");
                values_to_bind.push(val.clone());
            }
        }

        let sql = if cols.is_empty() {
            format!("INSERT INTO \"{}\" DEFAULT VALUES", schema.table_name)
        } else {
            format!(
                "INSERT INTO \"{}\" ({}) VALUES ({})",
                schema.table_name,
                cols.join(", "),
                placeholders.join(", ")
            )
        };

        let mut query = sqlx::query(&sql);
        for val in values_to_bind {
            query = Self::bind_json_value(query, &val);
        }

        let res = query.execute(&self.pool).await?;
        let new_id = res.last_insert_rowid();

        let created = self.get_by_id(schema, new_id).await?.unwrap_or(Value::Null);
        Ok(created)
    }

    async fn update(
        &self,
        schema: &ModelSchema,
        id: i64,
        payload: &Map<String, Value>,
    ) -> Result<Option<Value>, OxideError> {
        let mut set_clauses = Vec::new();
        let mut values_to_bind = Vec::new();

        for field in &schema.fields {
            if field.name == "id" || field.name == "created_at" {
                continue;
            }
            if field.name == "updated_at" {
                set_clauses.push("\"updated_at\" = datetime('now')".to_string());
                continue;
            }

            if let Some(val) = payload.get(&field.name) {
                set_clauses.push(format!("\"{}\" = ?", field.name));
                values_to_bind.push(val.clone());
            }
        }

        if set_clauses.is_empty() {
            return self.get_by_id(schema, id).await;
        }

        let sql = format!(
            "UPDATE \"{}\" SET {} WHERE id = ?",
            schema.table_name,
            set_clauses.join(", ")
        );

        let mut query = sqlx::query(&sql);
        for val in values_to_bind {
            query = Self::bind_json_value(query, &val);
        }
        query = query.bind(id);

        let res = query.execute(&self.pool).await?;
        if res.rows_affected() == 0 {
            return Ok(None);
        }

        self.get_by_id(schema, id).await
    }

    async fn delete(&self, schema: &ModelSchema, id: i64) -> Result<bool, OxideError> {
        let sql = format!("DELETE FROM \"{}\" WHERE id = ?", schema.table_name);
        let res = sqlx::query(&sql).bind(id).execute(&self.pool).await?;
        Ok(res.rows_affected() > 0)
    }

    async fn execute_raw(&self, sql: &str) -> Result<(), OxideError> {
        sqlx::query(sql).execute(&self.pool).await?;
        Ok(())
    }
}
