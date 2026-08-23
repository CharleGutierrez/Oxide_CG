pub mod filter;
pub mod handlers;
pub mod openapi;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::*;

pub use handlers::AppState;
pub use openapi::{openapi_json_handler, swagger_handler};

/// Build the API router with all sub-modules mounted cleanly
pub fn build_api_router(state: AppState) -> Router {
    Router::new()
        // Auth endpoints
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/logout", post(logout_handler))
        .route("/api/auth/me", get(me_handler))
        // Schema & dAPI endpoints
        .route("/api/d/schema", get(schema_handler))
        .route("/api/d/:model", get(list_records_handler).post(create_record_handler))
        .route(
            "/api/d/:model/:id",
            get(get_record_handler)
                .put(update_record_handler)
                .delete(delete_record_handler),
        )
        // Time travel rollback
        .route("/api/d/rollback/:log_id", post(rollback_handler))
        .route("/api/d/audit-logs", get(list_audit_logs_handler))
        // Approval workflow
        .route("/api/d/approvals", get(list_approvals_handler))
        .route("/api/d/approvals/:id/approve", post(approve_handler))
        .route("/api/d/approvals/:id/reject", post(reject_handler))
        // Health & Self-Healing probes
        .route("/health", get(health_check_handler))
        .route("/health/live", get(liveness_probe_handler))
        .route("/health/ready", get(readiness_probe_handler))
        // AI Tuner & Decision Engine endpoints
        .route("/api/ai/report", get(ai_report_handler))
        .route("/api/ai/indexes/apply", post(apply_index_handler))
        .route("/api/ai/assess-risk", post(assess_risk_handler))
        // OpenAPI / Swagger endpoints
        .route("/api/openapi.json", get(openapi_json_handler))
        .route("/swagger", get(swagger_handler))
        .with_state(state)
}
