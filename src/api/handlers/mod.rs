pub mod ai;
pub mod approval;
pub mod audit;
pub mod auth;
pub mod crud;
pub mod health;

pub use ai::*;
pub use approval::*;
pub use audit::*;
pub use auth::*;
pub use crud::*;
pub use health::*;

use crate::ai::AiTuner;
use crate::audit::{ApprovalService, AuditService};
use crate::auth::AuthService;
use crate::core::events::EventBus;
use crate::core::hooks::ModelHook;
use crate::core::resilience::{CircuitBreaker, SystemWatchdog};
use crate::db::SqliteDatabase;
use crate::model::SchemaRegistry;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

/// Shared application state for all HTTP route handlers
#[derive(Clone)]
pub struct AppState {
    pub db: SqliteDatabase,
    pub pool: Pool<Sqlite>,
    pub registry: SchemaRegistry,
    pub auth_service: Arc<AuthService>,
    pub audit_service: Arc<AuditService>,
    pub approval_service: Arc<ApprovalService>,
    pub event_bus: Arc<EventBus>,
    pub hooks: Arc<Vec<Box<dyn ModelHook>>>,
    pub watchdog: Arc<SystemWatchdog>,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub ai_tuner: Arc<AiTuner>,
}
