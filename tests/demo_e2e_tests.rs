use oxide_cg::prelude::*;
use oxide_cg::db::{SchemaMigrator, SqliteDatabase};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use serde_json::json;

/// Comprehensive End-to-End Test Suite for the Enterprise E-Commerce & CRM Demo (demo.rs)
#[tokio::test]
async fn test_ecommerce_demo_full_lifecycle() {
    let db_file = format!("/tmp/oxide_demo_e2e_{}.db", rand::random::<u64>());
    let db_url = format!("sqlite://{}?mode=rwc", db_file);

    // 1. Declare All 4 Domain Models from demo.rs
    let category_schema = ModelSchema::new("Category")
        .category("E-Commerce")
        .field(Field::string("name").required().unique().searchable())
        .field(Field::string("slug").unique().searchable())
        .field(Field::string("description"))
        .field(Field::boolean("is_active").default_value(json!(true)))
        .with_timestamps();

    let product_schema = ModelSchema::new("Product")
        .category("E-Commerce")
        .field(Field::string("title").required().searchable())
        .field(Field::string("sku").required().unique().searchable())
        .field(Field::money("price", "USD").required().filterable(true))
        .field(Field::float("discount_percent").requires_approval())
        .field(Field::progress_bar("stock_quantity", 500.0, "#22c55e").filterable(true))
        .field(Field::html("description"))
        .field(Field::r#enum("status", vec!["Draft", "Published", "Archived"]))
        .field(Field::foreign_key("category_id", "Category"))
        .field(Field::boolean("is_featured").default_value(json!(false)))
        .with_timestamps();

    let order_schema = ModelSchema::new("Order")
        .category("Sales & CRM")
        .field(Field::string("order_number").required().unique().searchable())
        .field(Field::email("customer_email").required().searchable())
        .field(Field::money("total_amount", "USD").required().filterable(true))
        .field(Field::r#enum("payment_status", vec!["Pending", "Paid", "Refunded", "Failed"]))
        .field(Field::r#enum("fulfillment_status", vec!["Unfulfilled", "Processing", "Shipped", "Delivered"]))
        .field(Field::string("shipping_address"))
        .with_timestamps();

    let ticket_schema = ModelSchema::new("Ticket")
        .category("Support")
        .field(Field::string("subject").required().searchable())
        .field(Field::email("user_email").required().searchable())
        .field(Field::r#enum("priority", vec!["Low", "Medium", "High", "Critical"]))
        .field(Field::r#enum("status", vec!["Open", "In_Progress", "Resolved", "Closed"]))
        .field(Field::markdown("message").required())
        .with_timestamps();

    // 2. Initialize Database & Migrate All Models
    let db = SqliteDatabase::connect(&db_url, 5).await.unwrap();
    SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();
    SchemaMigrator::migrate_model(&db.pool, &category_schema).await.unwrap();
    SchemaMigrator::migrate_model(&db.pool, &product_schema).await.unwrap();
    SchemaMigrator::migrate_model(&db.pool, &order_schema).await.unwrap();
    SchemaMigrator::migrate_model(&db.pool, &ticket_schema).await.unwrap();

    // 3. Build Full App Router
    let auth_service = std::sync::Arc::new(oxide_cg::auth::AuthService::new(db.pool.clone()));
    auth_service.ensure_admin_user().await.unwrap();
    let audit_service = std::sync::Arc::new(oxide_cg::audit::AuditService::new(db.pool.clone()));
    let approval_service = std::sync::Arc::new(oxide_cg::audit::ApprovalService::new(db.pool.clone()));
    let event_bus = std::sync::Arc::new(oxide_cg::core::events::EventBus::default());

    let mut map = std::collections::HashMap::new();
    map.insert("category".to_string(), category_schema.clone());
    map.insert("product".to_string(), product_schema.clone());
    map.insert("order".to_string(), order_schema.clone());
    map.insert("ticket".to_string(), ticket_schema.clone());
    let registry = SchemaRegistry::from_map(map);

    let watchdog = std::sync::Arc::new(oxide_cg::core::resilience::SystemWatchdog::default());
    let circuit_breaker = std::sync::Arc::new(oxide_cg::core::resilience::CircuitBreaker::new("demo_breaker", 5, 10));
    let ai_tuner = std::sync::Arc::new(oxide_cg::ai::AiTuner::default());

    let app_state = oxide_cg::api::handlers::AppState {
        pool: db.pool.clone(),
        db: db.clone(),
        registry: registry.clone(),
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
        site_name: "AcroStore Admin".to_string(),
        base_url: "http://localhost:8080".to_string(),
    });

    let api_router = oxide_cg::api::build_api_router(app_state);
    let ui_router = axum::Router::new()
        .route("/", axum::routing::get(oxide_cg::ui::admin_ui_handler))
        .route("/admin", axum::routing::get(oxide_cg::ui::admin_ui_handler))
        .with_state(ui_config);

    let app = axum::Router::new()
        .merge(api_router)
        .merge(ui_router)
        .layer(oxide_cg::core::resilience::panic_recovery_layer());

    // ---------------------------------------------------------------------------------
    // TEST 1: Create Category (POST /api/d/category)
    // ---------------------------------------------------------------------------------
    let cat_payload = json!({
        "name": "Mechanical Keyboards",
        "slug": "mechanical-keyboards",
        "description": "High-end ergonomic keyboards",
        "is_active": true
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/d/category")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(cat_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let cat_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let cat_id = cat_json["data"]["id"].as_i64().unwrap();

    // ---------------------------------------------------------------------------------
    // TEST 2: Create Product with Category FK (POST /api/d/product)
    // ---------------------------------------------------------------------------------
    let prod_payload = json!({
        "title": "Oxide Titan Pro Keyboard",
        "sku": "KB-OXIDE-PRO",
        "price": 189.99,
        "discount_percent": 0.0,
        "stock_quantity": 250,
        "description": "Premium CNC aluminum keyboard with hot-swap switches.",
        "status": "Published",
        "category_id": cat_id,
        "is_featured": true
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/d/product")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(prod_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let prod_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let prod_id = prod_json["data"]["id"].as_i64().unwrap();

    // ---------------------------------------------------------------------------------
    // TEST 3: Create Order (POST /api/d/order)
    // ---------------------------------------------------------------------------------
    let order_payload = json!({
        "order_number": "ORD-2026-90412",
        "customer_email": "buyer@example.com",
        "total_amount": 189.99,
        "payment_status": "Paid",
        "fulfillment_status": "Processing",
        "shipping_address": "100 Tech Blvd, Silicon Valley, CA"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/d/order")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(order_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    // ---------------------------------------------------------------------------------
    // TEST 4: Create Support Ticket (POST /api/d/ticket)
    // ---------------------------------------------------------------------------------
    let ticket_payload = json!({
        "subject": "Inquiry about firmware upgrade",
        "user_email": "buyer@example.com",
        "priority": "High",
        "status": "Open",
        "message": "Can I flash QMK firmware on this keyboard model?"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/d/ticket")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(ticket_payload.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    // ---------------------------------------------------------------------------------
    // TEST 5: Sensitive Field Approval on Product Discount (PUT /api/d/product/:id)
    // ---------------------------------------------------------------------------------
    let discount_update = json!({
        "discount_percent": 35.0
    });
    let req = Request::builder()
        .method("PUT")
        .uri(format!("/api/d/product/{}", prod_id))
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(discount_update.to_string()))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let update_res: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    // Sensitive discount field was quarantined for approval
    assert!(update_res["pending_approval_fields"].is_array());

    // ---------------------------------------------------------------------------------
    // TEST 6: Query Filter across Products (price__gte=100 & status=Published)
    // ---------------------------------------------------------------------------------
    let req = Request::builder().uri("/api/d/product?price__gte=100&status=Published").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let list_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(list_json["total"].as_i64().unwrap(), 1);
    assert_eq!(list_json["data"][0]["sku"].as_str().unwrap(), "KB-OXIDE-PRO");

    // ---------------------------------------------------------------------------------
    // TEST 7: Verify Admin Panel HTML delivery (/admin)
    // ---------------------------------------------------------------------------------
    let req = Request::builder().uri("/admin").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
