use crate::core::error::OxideError;
use async_trait::async_trait;
use serde_json::Value;

/// Trait for intercepting and customizing model lifecycle events.
#[async_trait]
pub trait ModelHook: Send + Sync {
    async fn before_create(&self, _model: &str, _data: &mut Value) -> Result<(), OxideError> {
        Ok(())
    }

    async fn after_create(&self, _model: &str, _record: &Value) -> Result<(), OxideError> {
        Ok(())
    }

    async fn before_update(&self, _model: &str, _id: i64, _data: &mut Value) -> Result<(), OxideError> {
        Ok(())
    }

    async fn after_update(&self, _model: &str, _id: i64, _record: &Value) -> Result<(), OxideError> {
        Ok(())
    }

    async fn before_delete(&self, _model: &str, _id: i64) -> Result<(), OxideError> {
        Ok(())
    }

    async fn after_delete(&self, _model: &str, _id: i64, _snapshot: &Value) -> Result<(), OxideError> {
        Ok(())
    }
}
