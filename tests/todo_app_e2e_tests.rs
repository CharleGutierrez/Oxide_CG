use oxide_cg::prelude::*;
use oxide_cg::db::{DatabaseAdapter, SchemaMigrator, SqliteDatabase};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use serde_json::json;

/// Comprehensive End-to-End Functional Test Suite for the Multi-Framework Todo Application
#[tokio::test]
async fn test_todo_app_complete_lifecycle_and_showcase() {
    let db_file = format!("/tmp/oxide_todo_e2e_{}.db", rand::random::<u64>());
    let db_url = format!("sqlite://{}?mode=rwc", db_file);

    // 1. Declare Todo Model Schema
    let todo_schema = ModelSchema::new("Todo")
        .category("Productivity")
        .field(Field::string("title").required().searchable())
        .field(Field::string("category").searchable().filterable(true))
        .field(Field::r#enum("priority", vec!["Low", "Medium", "High", "Critical"]).filterable(true))
        .field(Field::boolean("is_completed").default_value(json!(false)).filterable(true))
        .field(Field::progress_bar("progress", 100.0, "#10b981").filterable(true))
        .with_timestamps();

    // 2. Initialize Database & Seed Tasks
    let db = SqliteDatabase::connect(&db_url, 5).await.unwrap();
    SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();
    SchemaMigrator::migrate_model(&db.pool, &todo_schema).await.unwrap();

    let mut t1 = serde_json::Map::new();
    t1.insert("title".to_string(), json!("Build sub-millisecond Rust backend with Oxide_CG"));
    t1.insert("category".to_string(), json!("Rust Core"));
    t1.insert("priority".to_string(), json!("Critical"));
    t1.insert("progress".to_string(), json!(100));
    t1.insert("is_completed".to_string(), json!(true));
    let _task1 = db.insert(&todo_schema, &t1).await.unwrap();

    let mut t2 = serde_json::Map::new();
    t2.insert("title".to_string(), json!("Connect React 18 Hooks"));
    t2.insert("category".to_string(), json!("React 18"));
    t2.insert("priority".to_string(), json!("High"));
    t2.insert("progress".to_string(), json!(50));
    t2.insert("is_completed".to_string(), json!(false));
    let _task2 = db.insert(&todo_schema, &t2).await.unwrap();

    // 3. Build Full Application Router
    let auth_service = std::sync::Arc::new(oxide_cg::auth::AuthService::new(db.pool.clone()));
    auth_service.ensure_admin_user().await.unwrap();
    let audit_service = std::sync::Arc::new(oxide_cg::audit::AuditService::new(db.pool.clone()));
    let approval_service = std::sync::Arc::new(oxide_cg::audit::ApprovalService::new(db.pool.clone()));
    let event_bus = std::sync::Arc::new(oxide_cg::core::events::EventBus::default());
    let mut map = std::collections::HashMap::new();
    map.insert("todo".to_string(), todo_schema.clone());
    let registry = SchemaRegistry::from_map(map);

    let watchdog = std::sync::Arc::new(oxide_cg::core::resilience::SystemWatchdog::default());
    let circuit_breaker = std::sync::Arc::new(oxide_cg::core::resilience::CircuitBreaker::new("test", 5, 10));
    let ai_tuner = std::sync::Arc::new(oxide_cg::ai::AiTuner::default());

    let app_state = oxide_cg::api::handlers::AppState {
        pool: db.pool.clone(),
        db: db.clone(),
        registry,
        auth_service,
        audit_service,
        approval_service,
        event_bus,
        hooks: std::sync::Arc::new(Vec::new()),
        watchdog,
        circuit_breaker,
        ai_tuner,
    };

    let ui_config = std::sync::Arc::new(oxide_cg::ui::UiConfig {
        site_name: "Oxide_CG Task Hub".to_string(),
        base_url: "http://localhost:8080".to_string(),
    });

    let api_router = oxide_cg::api::build_api_router(app_state);
    let ui_router = axum::Router::new()
        .route("/", axum::routing::get(oxide_cg::ui::admin_ui_handler))
        .route("/todos", axum::routing::get(oxide_cg::ui::todo_showcase_handler))
        .route("/showcase", axum::routing::get(oxide_cg::ui::todo_showcase_handler))
        .route("/api/sdk/react.ts", axum::routing::get(oxide_cg::ui::react_sdk_handler))
        .route("/api/sdk/vue.ts", axum::routing::get(oxide_cg::ui::vue_sdk_handler))
        .route("/api/sdk/angular.ts", axum::routing::get(oxide_cg::ui::angular_sdk_handler))
        .with_state(ui_config);

    let app = axum::Router::new()
        .merge(api_router)
        .merge(ui_router)
        .layer(oxide_cg::core::resilience::panic_recovery_layer());

    // ---------------------------------------------------------------------------------
    // TEST 1: Verify HTML Delivery of Showcase Page (/todos and /showcase)
    // ---------------------------------------------------------------------------------
    let req = Request::builder().uri("/todos").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let html_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(html_str.contains("React 18/19 Todo App"));
    assert!(html_str.contains("Vue 3 Composition API Todo App"));
    assert!(html_str.contains("Angular 17/18 Signals Todo App"));

    // ---------------------------------------------------------------------------------
    // TEST 2: Query All Todos (GET /api/d/todo)
    // ---------------------------------------------------------------------------------
    let req = Request::builder().uri("/api/d/todo?$limit=100&$order=-created_at").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let json_res: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(json_res["success"].as_bool().unwrap());
    assert_eq!(json_res["total"].as_i64().unwrap(), 2);
    assert_eq!(json_res["data"].as_array().unwrap().len(), 2);

    // ---------------------------------------------------------------------------------
    // TEST 3: Query Filter by is_completed=true (Active vs Completed)
    // ---------------------------------------------------------------------------------
    let req = Request::builder().uri("/api/d/todo?is_completed=true").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let json_res: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json_res["total"].as_i64().unwrap(), 1);
    assert_eq!(json_res["data"][0]["title"].as_str().unwrap(), "Build sub-millisecond Rust backend with Oxide_CG");

    // ---------------------------------------------------------------------------------
    // TEST 4: Create a New Todo Task (POST /api/d/todo)
    // ---------------------------------------------------------------------------------
    let create_payload = json!({
        "title": "Deploy Oxide_CG on Bare Metal Cluster",
        "category": "DevOps",
        "priority": "Critical",
        "progress": 0,
        "is_completed": false
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/d/todo")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(create_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let json_res: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let new_task_id = json_res["data"]["id"].as_i64().unwrap();
    assert_eq!(json_res["data"]["title"].as_str().unwrap(), "Deploy Oxide_CG on Bare Metal Cluster");

    // ---------------------------------------------------------------------------------
    // TEST 5: Update & Toggle Task Status (PUT /api/d/todo/:id)
    // ---------------------------------------------------------------------------------
    let update_payload = json!({
        "is_completed": true,
        "progress": 100
    });
    let req = Request::builder()
        .method("PUT")
        .uri(format!("/api/d/todo/{}", new_task_id))
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(update_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let json_res: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json_res["data"]["is_completed"].as_bool().unwrap(), true);
    assert_eq!(json_res["data"]["progress"].as_f64().unwrap(), 100.0);

    // ---------------------------------------------------------------------------------
    // TEST 6: Delete Task (DELETE /api/d/todo/:id)
    // ---------------------------------------------------------------------------------
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/api/d/todo/{}", new_task_id))
        .body(axum::body::Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Verify task is deleted
    let req = Request::builder().uri(format!("/api/d/todo/{}", new_task_id)).body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    // ---------------------------------------------------------------------------------
    // TEST 7: Verify Frontend SDK Endpoints (/api/sdk/react.ts, vue.ts, angular.ts)
    // ---------------------------------------------------------------------------------
    let req = Request::builder().uri("/api/sdk/react.ts").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let req = Request::builder().uri("/api/sdk/vue.ts").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let req = Request::builder().uri("/api/sdk/angular.ts").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
