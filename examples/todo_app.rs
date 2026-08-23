use oxide_cg::db::DatabaseAdapter;
use oxide_cg::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Declare the Todo Model Schema
    let todo_schema = ModelSchema::new("Todo")
        .category("Productivity")
        .icon("check-square")
        .description("Multi-Frontend Task Items")
        .field(Field::string("title").required().searchable())
        .field(Field::string("category").searchable().filterable(true))
        .field(Field::r#enum("priority", vec!["Low", "Medium", "High", "Critical"]).filterable(true))
        .field(Field::boolean("is_completed").default_value(serde_json::json!(false)).filterable(true))
        .field(Field::progress_bar("progress", 100.0, "#10b981").filterable(true))
        .field(Field::html("description"))
        .with_timestamps();

    // 2. Pre-seed initial tasks if database is newly initialized
    let db_url = "sqlite://todo_app.db?mode=rwc";
    let db = oxide_cg::db::SqliteDatabase::connect(db_url, 5).await?;
    oxide_cg::db::SchemaMigrator::migrate_system_tables(&db.pool).await?;
    oxide_cg::db::SchemaMigrator::migrate_model(&db.pool, &todo_schema).await?;

    let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM \"todos\"")
        .fetch_one(&db.pool)
        .await
        .unwrap_or((0,));

    if count_row.0 == 0 {
        let initial_tasks = vec![
            ("Build sub-millisecond Rust backend with Oxide_CG", "Rust Core", "Critical", 100, true),
            ("Connect React 18 Hooks & Provider", "React 18", "High", 100, true),
            ("Connect Vue 3 Composition API Composables", "Vue 3", "High", 90, false),
            ("Connect Angular 17+ Signals & Standalone Components", "Angular 17+", "Medium", 80, false),
            ("Run AI Tuner Index Advisor & Workload Telemetry", "AI Optimization", "High", 75, false),
        ];

        for (title, cat, prio, prog, comp) in initial_tasks {
            let mut payload = serde_json::Map::new();
            payload.insert("title".to_string(), serde_json::json!(title));
            payload.insert("category".to_string(), serde_json::json!(cat));
            payload.insert("priority".to_string(), serde_json::json!(prio));
            payload.insert("progress".to_string(), serde_json::json!(prog));
            payload.insert("is_completed".to_string(), serde_json::json!(comp));
            let _ = db.insert(&todo_schema, &payload).await;
        }
    }

    println!("\n🚀 Launching Oxide_CG Multi-Framework Todo Application...");

    // 3. Start Oxide_CG Server
    OxideCGApp::new()
        .site_name("Oxide_CG Task Hub")
        .bind("0.0.0.0:8080")
        .database(db_url)
        .register(todo_schema)
        .run()
        .await?;

    Ok(())
}
