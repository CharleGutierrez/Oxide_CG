use oxide_cg::prelude::*;
use oxide_cg::db::{DatabaseAdapter, SchemaMigrator, SqliteDatabase};
use oxide_cg::auth::{AuthService, Crypto};
use oxide_cg::api::filter::QueryOptions;
use oxide_cg::ai::decision::{AiDecisionEngine, RiskLevel};
use std::collections::HashMap;
use serde_json::json;

/// Comprehensive Security Penetration Test Suite for Oxide_CG
/// Covering SQLi, Timing Attacks, Session Replay, Privilege Escalation, and DoS Boundary Attacks.

#[tokio::test]
async fn test_security_sqli_filter_parameterization() {
    let db_file = format!("/tmp/oxide_sec_sqli_{}.db", rand::random::<u64>());
    let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await.unwrap();
    SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

    let schema = ModelSchema::new("SecureItem")
        .field(Field::string("name").required().searchable())
        .field(Field::string("secret_data"))
        .with_timestamps();
    SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

    // Insert 1 private record
    let mut payload = serde_json::Map::new();
    payload.insert("name".to_string(), json!("Top Secret"));
    payload.insert("secret_data".to_string(), json!("CONFIDENTIAL_API_KEY_12345"));
    db.insert(&schema, &payload).await.unwrap();

    // SQL Injection Attack 1: SQL Tautology in contains filter
    // Attack payload: ' OR '1'='1 --
    let mut params = HashMap::new();
    params.insert("name__contains".to_string(), "' OR '1'='1 --".to_string());

    let q_opts = QueryOptions::parse(&params);
    let (select_sql, select_params, _, _) = q_opts.build_sql(&schema);

    // Verify query is strictly parameterized with `?` and not string concatenated
    assert!(select_sql.contains("\"name\" LIKE ?"));
    assert_eq!(select_params[0], json!("%' OR '1'='1 --%"));

    let mut query = sqlx::query(&select_sql);
    for p in select_params {
        query = SqliteDatabase::bind_json_value(query, &p);
    }
    let rows = query.fetch_all(&db.pool).await.unwrap();
    // SQLi is neutralized: returns 0 rows because literal string was not found
    assert_eq!(rows.len(), 0);
}

#[test]
fn test_security_sqli_order_by_whitelist_hardening() {
    let schema = ModelSchema::new("UserRecord")
        .field(Field::string("username").required())
        .field(Field::money("balance", "USD"));

    // SQLi Attack in $order: Injected SQL subquery & DROP TABLE attempt
    let mut malicious_params = HashMap::new();
    malicious_params.insert("$order".to_string(), "-username\"; DROP TABLE users; --".to_string());

    let opts = QueryOptions::parse(&malicious_params);
    let (select_sql, _, _, _) = opts.build_sql(&schema);

    // Malicious field rejected by schema whitelist: falls back safely to `"id" DESC`
    assert!(select_sql.contains("ORDER BY \"id\" DESC"));
    assert!(!select_sql.contains("DROP TABLE"));
}

#[tokio::test]
async fn test_security_expired_session_replay_attack() {
    let db_file = format!("/tmp/oxide_sec_auth_{}.db", rand::random::<u64>());
    let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await.unwrap();
    SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

    let auth = AuthService::new(db.pool.clone());
    auth.ensure_admin_user().await.unwrap();

    // 1. Create an expired session in database (expired 1 hour ago)
    let expired_token = Crypto::random_token(32);
    let expired_time = (chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO _oxide_sessions (token, user_id, ip_address, user_agent, expires_at)
        VALUES (?, 1, '127.0.0.1', 'HackerBot', ?)
        "#
    )
    .bind(&expired_token)
    .bind(&expired_time)
    .execute(&db.pool)
    .await
    .unwrap();

    // 2. Attacker attempts to replay expired token
    let user_opt = auth.validate_session(&expired_token).await.unwrap();

    // Replay attack blocked: Returns None and automatically purges expired token from DB
    assert!(user_opt.is_none());

    // Verify token was purged
    let session_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM _oxide_sessions WHERE token = ?")
        .bind(&expired_token)
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(session_count.0, 0);
}

#[test]
fn test_security_constant_time_password_verification() {
    let password = "SuperSecretSecurePassword!2026";
    let hash = Crypto::hash_password(password);

    // Valid verification
    assert!(Crypto::verify_password(password, &hash));

    // Tampered hash with altered signature (timing attack defense)
    let tampered_hash = format!("{}a", &hash[..hash.len() - 1]);
    assert!(!Crypto::verify_password(password, &tampered_hash));

    // Empty and garbage hashes
    assert!(!Crypto::verify_password(password, "invalid_hash_string"));
    assert!(!Crypto::verify_password(password, "$s2$garbage"));
}

#[test]
fn test_security_privilege_escalation_detection() {
    // Junior editor attempting to elevate their own role to Admin
    let assessment = AiDecisionEngine::assess_approval_risk("role", Some("Editor"), "Admin");

    assert_eq!(assessment.risk_level, RiskLevel::Critical);
    assert!(assessment.reasoning[0].contains("Elevated privilege assignment"));
    assert!(assessment.recommendation.contains("Critical Risk"));

    // Attempting to grant Superuser access
    let assessment2 = AiDecisionEngine::assess_approval_risk("user_role", Some("Viewer"), "superuser");
    assert_eq!(assessment2.risk_level, RiskLevel::Critical);
}

#[test]
fn test_security_dos_query_limit_clamping() {
    let mut params = HashMap::new();
    params.insert("$limit".to_string(), "9999999999".to_string()); // Overflow attempt
    params.insert("$offset".to_string(), "-50".to_string());         // Negative offset attempt

    let opts = QueryOptions::parse(&params);

    // Enforces maximum bounds: Limit clamped to 1000, Offset clamped to 0
    assert_eq!(opts.limit, 1000);
    assert_eq!(opts.offset, 0);
}
