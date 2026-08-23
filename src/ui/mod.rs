pub mod angular_sdk;
pub mod assets;
pub mod react_sdk;
pub mod vue_sdk;

use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use std::sync::Arc;

#[derive(Clone)]
pub struct UiConfig {
    pub site_name: String,
    pub base_url: String,
}

pub async fn admin_ui_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Html<String> {
    Html(assets::admin_react_spa_html(&config.site_name))
}

pub async fn react_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = react_sdk::generate_react_sdk(&config.base_url);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/typescript; charset=utf-8")],
        sdk_ts,
    )
        .into_response()
}

pub async fn vue_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = vue_sdk::generate_vue_sdk(&config.base_url);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/typescript; charset=utf-8")],
        sdk_ts,
    )
        .into_response()
}

pub async fn angular_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = angular_sdk::generate_angular_sdk(&config.base_url);
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/typescript; charset=utf-8")],
        sdk_ts,
    )
        .into_response()
}
