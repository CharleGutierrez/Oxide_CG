use crate::core::error::OxideError;
use crate::model::ModelSchema;
use async_trait::async_trait;
use serde_json::{Map, Value};

/// Extensible database adapter interface for Oxide_CG (SQLite, PostgreSQL, MySQL)
#[async_trait]
pub trait DatabaseAdapter: Send + Sync {
    async fn get_by_id(&self, schema: &ModelSchema, id: i64) -> Result<Option<Value>, OxideError>;
    async fn insert(&self, schema: &ModelSchema, payload: &Map<String, Value>) -> Result<Value, OxideError>;
    async fn update(&self, schema: &ModelSchema, id: i64, payload: &Map<String, Value>) -> Result<Option<Value>, OxideError>;
    async fn delete(&self, schema: &ModelSchema, id: i64) -> Result<bool, OxideError>;
    async fn execute_raw(&self, sql: &str) -> Result<(), OxideError>;
}
