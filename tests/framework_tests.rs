use oxide_cg::prelude::*;
use oxide_cg::db::{DatabaseAdapter, SchemaMigrator, SqliteDatabase};
use oxide_cg::auth::Crypto;
use oxide_cg::audit::{ApprovalService, AuditService};
use oxide_cg::api::filter::QueryOptions;
use oxide_cg::api::openapi::OpenApiGenerator;
use oxide_cg::core::resilience::{BreakerState, CircuitBreaker, SystemWatchdog};
use oxide_cg::ui::angular_sdk::generate_angular_sdk;
use oxide_cg::ui::react_sdk::generate_react_sdk;
use oxide_cg::ui::vue_sdk::generate_vue_sdk;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use serde_json::json;
use async_trait::async_trait;

#[tokio::test]
async fn test_crypto_password_hashing() {
    let password = "SuperSecretPassword123!";
    let hash = Crypto::hash_password(password);

    assert!(hash.starts_with("$s2$"));
    assert!(Crypto::verify_password(password, &hash));
    assert!(!Crypto::verify_password("wrong_password", &hash));
}

#[tokio::test]
async fn test_database_crud_and_audit() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_file = format!("/tmp/oxide_cg_test_{}.db", rand::random::<u64>());
    let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await?;

    // Migrate system tables
    SchemaMigrator::migrate_system_tables(&db.pool).await?;

    // Create test model
    let product_schema = ModelSchema::new("Product")
        .field(Field::string("name").required().searchable())
        .field(Field::money("price", "USD").required())
        .field(Field::integer("stock"))
        .field(Field::boolean("in_stock").default_value(json!(true)))
        .with_timestamps();

    SchemaMigrator::migrate_model(&db.pool, &product_schema).await?;

    let audit = AuditService::new(db.pool.clone());

    // 1. Insert product
    let mut payload = serde_json::Map::new();
    payload.insert("name".to_string(), json!("Rust Mechanical Keyboard"));
    payload.insert("price".to_string(), json!(149.99));
    payload.insert("stock".to_string(), json!(50));
    payload.insert("in_stock".to_string(), json!(true));

    let created = db.insert(&product_schema, &payload).await?;
    let id = created.get("id").unwrap().as_i64().unwrap();
    assert_eq!(id, 1);
    assert_eq!(created.get("name").unwrap().as_str().unwrap(), "Rust Mechanical Keyboard");

    // Log audit
    audit.log_action("Product", id, "CREATE", Some(1), Some("admin"), &json!(payload), &created, None).await?;

    // 2. Query product
    let fetched = db.get_by_id(&product_schema, id).await?.unwrap();
    assert_eq!(fetched.get("price").unwrap().as_f64().unwrap(), 149.99);

    // 3. Update product
    let mut update_payload = serde_json::Map::new();
    update_payload.insert("price".to_string(), json!(129.99));
    let updated = db.update(&product_schema, id, &update_payload).await?.unwrap();
    assert_eq!(updated.get("price").unwrap().as_f64().unwrap(), 129.99);

    let update_log_id = audit.log_action("Product", id, "UPDATE", Some(1), Some("admin"), &json!(update_payload), &fetched, None).await?;

    // 4. Test Rollback: Revert update back to initial snapshot (149.99)
    let rolled_back = audit.rollback(update_log_id, &product_schema, Some(1), Some("admin")).await?;
    assert!(rolled_back);

    let reverted = db.get_by_id(&product_schema, id).await?.unwrap();
    assert_eq!(reverted.get("price").unwrap().as_f64().unwrap(), 149.99);

    // 5. Delete and Rollback Restore
    let before_delete_snapshot = db.get_by_id(&product_schema, id).await?.unwrap();
    let deleted = db.delete(&product_schema, id).await?;
    assert!(deleted);
    assert!(db.get_by_id(&product_schema, id).await?.is_none());

    let delete_log_id = audit.log_action("Product", id, "DELETE", Some(1), Some("admin"), &json!(null), &before_delete_snapshot, None).await?;
    let restored = audit.rollback(delete_log_id, &product_schema, Some(1), Some("admin")).await?;
    assert!(restored);

    let restored_record = db.get_by_id(&product_schema, id).await?.unwrap();
    assert_eq!(restored_record.get("name").unwrap().as_str().unwrap(), "Rust Mechanical Keyboard");

    Ok(())
}

#[tokio::test]
async fn test_approval_workflow() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_file = format!("/tmp/oxide_cg_test_{}.db", rand::random::<u64>());
    let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await?;

    SchemaMigrator::migrate_system_tables(&db.pool).await?;

    let schema = ModelSchema::new("Employee")
        .field(Field::string("name").required())
        .field(Field::money("salary", "USD").requires_approval());

    SchemaMigrator::migrate_model(&db.pool, &schema).await?;

    let approvals = ApprovalService::new(db.pool.clone());

    let mut payload = serde_json::Map::new();
    payload.insert("name".to_string(), json!("Alice"));
    payload.insert("salary".to_string(), json!(75000.0));
    let emp = db.insert(&schema, &payload).await?;
    let emp_id = emp.get("id").unwrap().as_i64().unwrap();

    // Junior editor attempts to raise salary to 120,000 -> creates pending approval
    let app_id = approvals.create_approval(
        "Employee",
        emp_id,
        "salary",
        Some("75000.0"),
        "120000.0",
        Some(2),
        Some("junior_editor"),
    ).await?;

    let pending = approvals.list_pending().await?;
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].new_value, "120000.0");

    // Manager approves
    let approved = approvals.approve(app_id, 1, "manager_admin", &schema.table_name).await?;
    assert!(approved);

    let updated_emp = db.get_by_id(&schema, emp_id).await?.unwrap();
    assert_eq!(updated_emp.get("salary").unwrap().as_f64().unwrap(), 120000.0);

    Ok(())
}

#[test]
fn test_query_filter_builder() {
    let schema = ModelSchema::new("Article")
        .field(Field::string("title").searchable())
        .field(Field::string("status").filterable(true))
        .field(Field::integer("views").filterable(true));

    let mut params = HashMap::new();
    params.insert("$limit".to_string(), "10".to_string());
    params.insert("$offset".to_string(), "20".to_string());
    params.insert("$order".to_string(), "-views".to_string());
    params.insert("$search".to_string(), "rust".to_string());
    params.insert("status".to_string(), "published".to_string());
    params.insert("views__gte".to_string(), "100".to_string());

    let opts = QueryOptions::parse(&params);
    assert_eq!(opts.limit, 10);
    assert_eq!(opts.offset, 20);

    let (select_sql, select_params, count_sql, count_params) = opts.build_sql(&schema);
    assert!(select_sql.contains("\"title\" LIKE ?"));
    assert!(select_sql.contains("\"status\" = ?"));
    assert!(select_sql.contains("\"views\" >= ?"));
    assert!(select_sql.contains("ORDER BY \"views\" DESC LIMIT ? OFFSET ?"));
    assert!(count_sql.starts_with("SELECT COUNT(*) as total FROM \"articles\""));
    assert_eq!(count_params.len(), 4);
    assert_eq!(select_params.len(), 6);
}

#[test]
fn test_openapi_and_react_sdk_generation() {
    let mut schemas = HashMap::new();
    let schema = ModelSchema::new("Customer")
        .category("CRM")
        .field(Field::string("name").required())
        .field(Field::email("email").required())
        .field(Field::boolean("active"));

    schemas.insert("customer".to_string(), schema);
    let registry = SchemaRegistry::from_map(schemas);

    let spec = OpenApiGenerator::generate_spec(&registry);
    assert_eq!(spec.get("openapi").unwrap().as_str().unwrap(), "3.1.0");
    assert!(spec.get("paths").unwrap().get("/api/d/customer").is_some());
    assert!(spec.get("components").unwrap().get("schemas").unwrap().get("Customer").is_some());

    // Test React SDK generation
    let react_sdk = generate_react_sdk("http://localhost:8080");
    assert!(react_sdk.contains("export class OxideClient"));
    assert!(react_sdk.contains("export function useOxideQuery"));
    assert!(react_sdk.contains("export function useOxideMutation"));
    assert!(react_sdk.contains("export const OxideProvider"));

    // Test Vue 3 SDK generation
    let vue_sdk = generate_vue_sdk("http://localhost:8080");
    assert!(vue_sdk.contains("export function useOxideVueQuery"));
    assert!(vue_sdk.contains("export function useOxideVueMutation"));
    assert!(vue_sdk.contains("export function provideOxide"));

    // Test Angular 17+ SDK generation
    let ng_sdk = generate_angular_sdk("http://localhost:8080");
    assert!(ng_sdk.contains("@Injectable"));
    assert!(ng_sdk.contains("export class OxideService"));
    assert!(ng_sdk.contains("createSignalQuery"));
}

struct TestAuditHook {
    triggered: Arc<AtomicBool>,
}

#[async_trait]
impl ModelHook for TestAuditHook {
    async fn before_create(&self, _model: &str, _data: &mut serde_json::Value) -> Result<(), OxideError> {
        self.triggered.store(true, Ordering::SeqCst);
        Ok(())
    }
}

#[tokio::test]
async fn test_lifecycle_hook() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let triggered = Arc::new(AtomicBool::new(false));
    let hook = TestAuditHook { triggered: triggered.clone() };

    let mut data = serde_json::json!({ "title": "Test" });
    hook.before_create("Item", &mut data).await?;

    assert!(triggered.load(Ordering::SeqCst));
    Ok(())
}

#[tokio::test]
async fn test_circuit_breaker_self_healing() {
    let breaker = CircuitBreaker::new("test_breaker", 3, 1); // 3 failures, 1s cooldown

    assert_eq!(breaker.state(), BreakerState::Closed);
    assert!(breaker.allow_execution());

    // 2 failures -> still closed
    breaker.record_failure();
    breaker.record_failure();
    assert_eq!(breaker.state(), BreakerState::Closed);

    // 3rd failure -> trips to OPEN
    breaker.record_failure();
    assert_eq!(breaker.state(), BreakerState::Open);
    assert!(!breaker.allow_execution());

    // Wait for cooldown (1s) -> transitions to HALF_OPEN (Self-Healing probe)
    tokio::time::sleep(Duration::from_millis(1100)).await;
    assert!(breaker.allow_execution());
    assert_eq!(breaker.state(), BreakerState::HalfOpen);

    // Successful probe -> auto-heals back to CLOSED
    breaker.record_success();
    assert_eq!(breaker.state(), BreakerState::Closed);
    assert!(breaker.allow_execution());
}

#[tokio::test]
async fn test_system_watchdog() {
    let watchdog = SystemWatchdog::new(Duration::from_millis(50));
    assert!(watchdog.is_healthy());
    assert_eq!(watchdog.auto_heal_count(), 0);

    let status = watchdog.status_json();
    assert_eq!(status["status"], "HEALTHY");
}

#[tokio::test]
async fn test_ai_tuner_and_risk_scoring() {
    // 1. Test AI Tuner & Index Advisor
    let mut schemas = HashMap::new();
    let schema = ModelSchema::new("Product")
        .field(Field::string("category").filterable(true))
        .field(Field::money("price", "USD").filterable(true));
    schemas.insert("product".to_string(), schema);
    let registry = SchemaRegistry::from_map(schemas);

    let tuner = AiTuner::new();
    tuner.record_query_pattern("Product", "products", &["category"], 25.0);

    let report = tuner.generate_report(&registry);
    assert_eq!(report.total_queries_analyzed, 1);
    assert!(!report.recommendations.is_empty());
    assert!(report.recommendations[0].ddl.contains("CREATE INDEX"));

    // 2. Test AI Decision Engine Risk Scoring
    // Low risk: minor price bump (100 -> 110)
    let low_risk = AiDecisionEngine::assess_approval_risk("price", Some("100.0"), "110.0");
    assert_eq!(low_risk.risk_level, RiskLevel::Low);

    // High risk: massive salary increase (50,000 -> 250,000)
    let high_risk = AiDecisionEngine::assess_approval_risk("salary", Some("50000.0"), "250000.0");
    assert_eq!(high_risk.risk_level, RiskLevel::High);
    assert!(high_risk.reasoning[0].contains("deviation"));

    // Critical risk: granting superadmin
    let crit_risk = AiDecisionEngine::assess_approval_risk("role", Some("Editor"), "Admin");
    assert_eq!(crit_risk.risk_level, RiskLevel::Critical);
}

#[test]
fn test_multi_db_dialects() {
    let schema = ModelSchema::new("Order")
        .field(Field::string("order_id").required())
        .field(Field::money("total", "USD"))
        .field(Field::boolean("is_paid").default_value(json!(false)))
        .field(Field::json("metadata"))
        .with_timestamps();

    // SQLite DDL
    let sqlite_ddl = SqlDialect::create_table_ddl(DatabaseType::Sqlite, &schema);
    assert!(sqlite_ddl.contains("INTEGER PRIMARY KEY AUTOINCREMENT"));
    assert!(sqlite_ddl.contains("DEFAULT (datetime('now'))"));

    // PostgreSQL DDL
    let pg_ddl = SqlDialect::create_table_ddl(DatabaseType::Postgres, &schema);
    assert!(pg_ddl.contains("BIGSERIAL PRIMARY KEY"));
    assert!(pg_ddl.contains("JSONB"));
    assert!(pg_ddl.contains("DEFAULT (NOW())"));

    // MySQL DDL
    let mysql_ddl = SqlDialect::create_table_ddl(DatabaseType::MySql, &schema);
    assert!(mysql_ddl.contains("BIGINT AUTO_INCREMENT PRIMARY KEY"));
    assert!(mysql_ddl.contains("`orders`"));
    assert!(mysql_ddl.contains("DEFAULT (NOW())"));
}

#[tokio::test]
async fn test_panic_recovery_http_isolation() {
    use axum::{routing::get, Router, http::{Request, StatusCode}};
    use tower::ServiceExt;
    use oxide_cg::core::resilience::panic_recovery_layer;

    // Build router with panic recovery layer
    let app = Router::new()
        .route("/panic", get(|| async {
            panic!("Simulated critical fault in request handler!");
            #[allow(unreachable_code)]
            "unreachable"
        }))
        .route("/healthy", get(|| async { "all good" }))
        .layer(panic_recovery_layer());

    // 1. Send request to panic route -> should return 500 without crashing
    let req = Request::builder().uri("/panic").body(axum::body::Body::empty()).unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

    // 2. Send request to healthy route -> server is completely intact and functional!
    let req2 = Request::builder().uri("/healthy").body(axum::body::Body::empty()).unwrap();
    let res2 = app.oneshot(req2).await.unwrap();
    assert_eq!(res2.status(), StatusCode::OK);
}
