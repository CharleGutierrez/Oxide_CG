use crate::ai::decision::AiDecisionEngine;
use crate::api::handlers::AppState;
use crate::auth::extractor::OptionalAuthUser;
use crate::core::error::OxideError;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApplyIndexPayload {
    pub table: String,
    pub column: String,
}

#[derive(Debug, Deserialize)]
pub struct AssessRiskPayload {
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: String,
}

pub async fn ai_report_handler(State(state): State<AppState>) -> impl IntoResponse {
    let report = state.ai_tuner.generate_report(&state.registry);
    Json(json!({ "success": true, "report": report }))
}

pub async fn apply_index_handler(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, OxideError> {
    let reviewer = match user {
        Some(ref u) if u.role.is_admin() => u,
        _ => return Err(OxideError::Forbidden("Requires Admin role to apply database indexes".to_string())),
    };

    let table = params
        .get("table")
        .ok_or_else(|| OxideError::Validation("Parameter 'table' is required".to_string()))?;
    let column = params
        .get("column")
        .ok_or_else(|| OxideError::Validation("Parameter 'column' is required".to_string()))?;

    let result = state.ai_tuner.apply_index(&state.pool, table, column).await?;

    Ok(Json(json!({
        "success": true,
        "message": result,
        "applied_by": reviewer.username
    })))
}

pub async fn assess_risk_handler(
    Json(payload): Json<AssessRiskPayload>,
) -> impl IntoResponse {
    let assessment = AiDecisionEngine::assess_approval_risk(
        &payload.field_name,
        payload.old_value.as_deref(),
        &payload.new_value,
    );

    Json(json!({ "success": true, "assessment": assessment }))
}
