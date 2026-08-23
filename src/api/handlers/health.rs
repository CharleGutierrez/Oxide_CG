use crate::api::handlers::AppState;
use crate::core::resilience::total_panic_recoveries;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

pub async fn health_check_handler(State(state): State<AppState>) -> impl IntoResponse {
    let is_db_healthy = state.watchdog.is_healthy();
    let status_code = if is_db_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        Json(json!({
            "status": if is_db_healthy { "HEALTHY" } else { "DEGRADED" },
            "uptime_seconds": state.watchdog.uptime_secs(),
            "self_healing": {
                "watchdog": state.watchdog.status_json(),
                "circuit_breaker": state.circuit_breaker.status_json(),
                "panic_recoveries_count": total_panic_recoveries(),
            },
            "system": {
                "models_count": state.registry.len(),
                "database_driver": "sqlite_wal",
            }
        })),
    )
}

pub async fn liveness_probe_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "alive": true })))
}

pub async fn readiness_probe_handler(State(state): State<AppState>) -> impl IntoResponse {
    if state.watchdog.is_healthy() {
        (StatusCode::OK, Json(json!({ "ready": true })))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "ready": false, "reason": "Database connection recovering" })))
    }
}
