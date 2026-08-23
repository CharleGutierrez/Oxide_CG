use crate::ai::AiTuner;
use crate::api::build_api_router;
use crate::api::handlers::AppState;
use crate::audit::{ApprovalService, AuditService};
use crate::auth::AuthService;
use crate::core::config::OxideConfig;
use crate::core::error::OxideError;
use crate::core::events::EventBus;
use crate::core::hooks::ModelHook;
use crate::core::resilience::{panic_recovery_layer, CircuitBreaker, SystemWatchdog};
use crate::db::{DatabaseType, SchemaMigrator, SqliteDatabase};
use crate::model::{ModelSchema, SchemaRegistry};
use crate::ui::{admin_ui_handler, angular_sdk_handler, react_sdk_handler, todo_showcase_handler, vue_sdk_handler, UiConfig};
use axum::{routing::get, Router};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Core Application Engine and Server Builder for Oxide_CG
pub struct OxideCGApp {
    pub config: OxideConfig,
    pub schemas: HashMap<String, ModelSchema>,
    pub hooks: Vec<Box<dyn ModelHook>>,
}

/// Backward compatibility alias
pub type OxideApp = OxideCGApp;

impl Default for OxideCGApp {
    fn default() -> Self {
        Self::new()
    }
}

impl OxideCGApp {
    pub fn new() -> Self {
        Self {
            config: OxideConfig::default(),
            schemas: HashMap::new(),
            hooks: Vec::new(),
        }
    }

    pub fn site_name(mut self, name: impl Into<String>) -> Self {
        self.config.site_name = name.into();
        self
    }

    pub fn bind(mut self, addr: impl Into<String>) -> Self {
        self.config.bind_address = addr.into();
        self
    }

    pub fn database(mut self, url: impl Into<String>) -> Self {
        self.config.database_url = url.into();
        self
    }

    pub fn max_connections(mut self, count: u32) -> Self {
        self.config.max_db_connections = count;
        self
    }

    pub fn register(mut self, schema: ModelSchema) -> Self {
        let key = schema.name.to_lowercase();
        self.schemas.insert(key, schema);
        self
    }

    pub fn hook<H: ModelHook + 'static>(mut self, hook: H) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }

    /// Build and run the server
    pub async fn run(self) -> Result<(), OxideError> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "oxide_cg=info,tower_http=info".into()),
            )
            .try_init();

        // 1. Initialize High-Performance SQLite Pool
        let db = SqliteDatabase::connect(&self.config.database_url, self.config.max_db_connections).await?;

        // 2. Auto-Migrate Schemas
        SchemaMigrator::migrate_system_tables(&db.pool).await?;
        for schema in self.schemas.values() {
            SchemaMigrator::migrate_model(&db.pool, schema).await?;
        }

        // 3. Initialize Services
        let auth_service = Arc::new(AuthService::new(db.pool.clone()));
        auth_service.ensure_admin_user().await?;

        let audit_service = Arc::new(AuditService::new(db.pool.clone()));
        let approval_service = Arc::new(ApprovalService::new(db.pool.clone()));
        let event_bus = Arc::new(EventBus::default());
        let registry = SchemaRegistry::from_map(self.schemas);

        // Self-Healing Watchdog, Circuit Breaker & AI Tuner
        let watchdog = Arc::new(SystemWatchdog::default());
        watchdog.start(db.pool.clone());
        let circuit_breaker = Arc::new(CircuitBreaker::new("global_db_breaker", 5, 10));
        let ai_tuner = Arc::new(AiTuner::default());

        let app_state = AppState {
            pool: db.pool.clone(),
            db,
            registry,
            auth_service,
            audit_service,
            approval_service,
            event_bus,
            hooks: Arc::new(self.hooks),
            watchdog,
            circuit_breaker,
            ai_tuner,
        };

        let ui_config = Arc::new(UiConfig {
            site_name: self.config.site_name.clone(),
            base_url: format!("http://{}", self.config.bind_address),
        });

        // 4. Construct Sub-Routers
        let api_router = build_api_router(app_state);

        let ui_router = Router::new()
            .route("/", get(admin_ui_handler))
            .route("/admin", get(admin_ui_handler))
            .route("/todos", get(todo_showcase_handler))
            .route("/showcase", get(todo_showcase_handler))
            .route("/api/sdk/react.ts", get(react_sdk_handler))
            .route("/api/sdk/vue.ts", get(vue_sdk_handler))
            .route("/api/sdk/angular.ts", get(angular_sdk_handler))
            .with_state(ui_config);

        let app = Router::new()
            .merge(api_router)
            .merge(ui_router)
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .layer(panic_recovery_layer());

        // 5. Banner output
        let db_type = DatabaseType::from_url(&self.config.database_url);
        let addr: SocketAddr = self.config.bind_address.parse().map_err(|e: std::net::AddrParseError| {
            OxideError::Internal(format!("Invalid bind address: {}", e))
        })?;

        println!(
            r#"
   ____            _       __          ________
  / __ \_  _______(_)___  / /   /\    / ____/ /_
 / / / / |/_/ ___/ / __ \/ _ \ / /_  / /   / __ \
/ /_/ />  </ /__/ / /_/ / // // __ \/ /___/ /_/ /
\____/_/|_|\___/_/\__,_/\___//_/  /_/\____/\_,___/

 ⚡ Oxide_CG v0.1.0 (React ⚛️  • Vue 🟢 • Angular 🅰️  • AI Tuner 🧠)
  Server Listening:   http://{}
 ⚛️  React Admin SPA:    http://{}
  Database Driver:    {}
  AI Tuner Engine:    Online (Index Advisor & Risk Scorer)
  OpenAPI Docs:       http://{}/swagger
 📦 SDKs (React/Vue/NG): http://{}/api/sdk/react.ts
 ️  Default Admin:      admin / admin
"#,
            addr, addr, db_type.name(), addr, addr
        );

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}
