//! # Oxide_CG
//!
//! Ultra-fast, zero-overhead Rust web framework and auto-generated Admin/REST engine
//! with multi-database support (SQLite, PostgreSQL, MySQL), integrated React ecosystem,
//! and AI Tuner processing and decision engine.

pub mod ai;
pub mod api;
pub mod app;
pub mod audit;
pub mod auth;
pub mod core;
pub mod db;
pub mod model;
pub mod prelude;
pub mod ui;

pub use ai::{AiDecisionEngine, AiTuner, RiskAssessment, RiskLevel};
pub use app::{OxideApp, OxideCGApp};
pub use core::config::OxideConfig;
pub use core::error::OxideError;
pub use core::events::{EventBus, SystemEvent};
pub use core::hooks::ModelHook;
pub use db::{DatabaseAdapter, DatabaseType, SqlDialect};
pub use model::{Field, FieldType, ModelSchema, SchemaRegistry};
