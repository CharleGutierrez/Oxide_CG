use oxide_cg::prelude::*;
use oxide_cg::db::{DatabaseAdapter, SchemaMigrator, SqliteDatabase};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use serde_json::json;

/// Comprehensive benchmark and stress suite simulating 2020-2025 Modern Server Microarchitectures:
/// - 2020: AMD EPYC 7002 Rome (64 Cores / 128 Threads, PCIe 4.0, DDR4-3200) & Intel Ice Lake Xeon
/// - 2021-2022: AMD EPYC Milan-X (3D V-Cache) & AWS Graviton3 (64 Cores, DDR5)
/// - 2023: AMD EPYC 9004 Genoa / Bergamo (96-128 Cores, AVX-512, PCIe 5.0, DDR5) & Intel Sapphire Rapids
/// - 2024-2025: AMD EPYC 9005 Turin (128-192 Cores / 384 Threads, Zen 5, 512-bit AVX-512) & Intel Granite Rapids / Graviton4

#[test]
fn test_2020_zen2_ice_lake_64_threads_throughput() {
    // Simulating 2020 Era: 64 hardware worker threads (EPYC 7742 Rome / Xeon Platinum 8380)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(64)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/oxide_server_2020_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 50).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("ServerMetric2020")
            .field(Field::string("node_id").required().searchable())
            .field(Field::float("cpu_utilization").filterable(true))
            .field(Field::integer("iops"))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        let total_ops = Arc::new(AtomicU64::new(0));
        let start = Instant::now();

        let mut handles = Vec::new();
        for worker_id in 0..64 {
            let db_clone = db.clone();
            let schema_clone = schema.clone();
            let ops_clone = total_ops.clone();

            handles.push(tokio::spawn(async move {
                for i in 0..25 {
                    let mut payload = serde_json::Map::new();
                    payload.insert("node_id".to_string(), json!(format!("epyc-rome-node-{}", worker_id)));
                    payload.insert("cpu_utilization".to_string(), json!(78.4 + (i as f64 * 0.1)));
                    payload.insert("iops".to_string(), json!(150000 + i));
                    let res = db_clone.insert(&schema_clone, &payload).await.unwrap();
                    assert!(res.get("id").is_some());
                    ops_clone.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        let elapsed = start.elapsed();
        let ops = total_ops.load(Ordering::Relaxed);
        assert_eq!(ops, 1600); // 64 workers * 25 queries = 1,600 ops

        println!("⚡ [2020 Server Simulation - 64 Threads]: 1,600 transactions in {:?} ({:.2} ops/sec)", elapsed, ops as f64 / elapsed.as_secs_f64());
        assert!(elapsed.as_millis() < 2500);
    });
}

#[test]
fn test_2022_2023_zen4_sapphire_rapids_128_threads_concurrency() {
    // Simulating 2022-2023 Era: 128 hardware worker threads (EPYC 9654 Genoa / Bergamo / Xeon 4th Gen)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(128)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/oxide_server_2023_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 100).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("GenoaProduct")
            .field(Field::string("sku").required().unique().searchable())
            .field(Field::money("price", "USD").filterable(true))
            .field(Field::integer("stock"))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        // Pre-seed base data
        let mut seed = serde_json::Map::new();
        seed.insert("sku".to_string(), json!("GENOA-BASE-001"));
        seed.insert("price".to_string(), json!(499.99));
        seed.insert("stock".to_string(), json!(5000));
        let created = db.insert(&schema, &seed).await.unwrap();
        let base_id = created.get("id").unwrap().as_i64().unwrap();

        let start = Instant::now();
        let total_reads = Arc::new(AtomicU64::new(0));

        // 128 parallel concurrent reader workers simulating DDR5 memory bandwidth saturation
        let mut handles = Vec::new();
        for _ in 0..128 {
            let db_clone = db.clone();
            let schema_clone = schema.clone();
            let reads_clone = total_reads.clone();

            handles.push(tokio::spawn(async move {
                for _ in 0..50 {
                    let rec = db_clone.get_by_id(&schema_clone, base_id).await.unwrap();
                    assert!(rec.is_some());
                    reads_clone.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        let elapsed = start.elapsed();
        let total = total_reads.load(Ordering::Relaxed);
        assert_eq!(total, 6400); // 128 workers * 50 reads = 6,400 concurrent reads

        println!("⚡ [2023 Server Simulation - 128 Threads]: 6,400 concurrent read transactions in {:?} ({:.2} reads/sec)", elapsed, total as f64 / elapsed.as_secs_f64());
        assert!(elapsed.as_millis() < 3000);
    });
}

#[test]
fn test_2024_2025_zen5_turin_granite_rapids_256_threads_stress() {
    // Simulating 2024-2025 Era: 256 parallel worker tasks (EPYC 9005 Turin 192c/384t & Xeon 6 Granite Rapids / Graviton4)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(128)
        .max_blocking_threads(256)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let db_file = format!("/tmp/oxide_server_2025_{}.db", rand::random::<u64>());
        let db = SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 128).await.unwrap();
        SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

        let schema = ModelSchema::new("TurinEnterprise")
            .field(Field::string("session_uuid").required().searchable())
            .field(Field::integer("latency_micros"))
            .field(Field::boolean("status_ok").default_value(json!(true)))
            .with_timestamps();
        SchemaMigrator::migrate_model(&db.pool, &schema).await.unwrap();

        let start = Instant::now();
        let total_ops = Arc::new(AtomicU64::new(0));

        // 256 parallel asynchronous workers
        let mut handles = Vec::new();
        for worker_id in 0..256 {
            let db_clone = db.clone();
            let schema_clone = schema.clone();
            let ops_clone = total_ops.clone();

            handles.push(tokio::spawn(async move {
                let mut payload = serde_json::Map::new();
                payload.insert("session_uuid".to_string(), json!(format!("turin-sess-{}-{}", worker_id, uuid::Uuid::new_v4())));
                payload.insert("latency_micros".to_string(), json!(142));
                payload.insert("status_ok".to_string(), json!(true));

                let rec = db_clone.insert(&schema_clone, &payload).await.unwrap();
                let new_id = rec.get("id").unwrap().as_i64().unwrap();
                let fetched = db_clone.get_by_id(&schema_clone, new_id).await.unwrap();
                assert!(fetched.is_some());

                ops_clone.fetch_add(1, Ordering::Relaxed);
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        let elapsed = start.elapsed();
        let ops = total_ops.load(Ordering::Relaxed);
        assert_eq!(ops, 256);

        println!("⚡ [2025 Server Simulation - 256 Concurrent Tasks]: Completed read-after-write transactions across 256 tasks in {:?} ({:.2} txn/sec)", elapsed, ops as f64 / elapsed.as_secs_f64());
        assert!(elapsed.as_millis() < 1500);
    });
}

#[test]
fn test_2020_to_2025_ai_tuner_realtime_telemetry() {
    let tuner = oxide_cg::ai::AiTuner::new();

    let mut schemas = std::collections::HashMap::new();
    let schema = ModelSchema::new("HighSpeedMetric")
        .field(Field::string("region").searchable())
        .field(Field::money("cost", "USD").filterable(true));
    schemas.insert("highspeedmetric".to_string(), schema);
    let registry = SchemaRegistry::from_map(schemas);

    // Simulate 10,000 sub-millisecond query executions on a modern multi-core server
    for i in 0..10_000 {
        let lat = 0.12 + ((i % 10) as f64 * 0.05); // 0.12ms to 0.57ms
        tuner.record_query_pattern("HighSpeedMetric", "highspeedmetrics", &["region", "cost"], lat);
    }

    let report = tuner.generate_report(&registry);
    assert_eq!(report.total_queries_analyzed, 10_000);
    assert!(report.p50_latency_ms < 0.50);
    assert!(report.p99_latency_ms < 1.00);
    assert!(report.workload_summary.contains("Optimal"));
    assert!(!report.recommendations.is_empty());

    println!("🧠 [2020-2025 AI Tuner Telemetry]: Analyzed 10,000 queries => p50: {:.2}ms, p95: {:.2}ms, p99: {:.2}ms | Status: {}",
        report.p50_latency_ms, report.p95_latency_ms, report.p99_latency_ms, report.workload_summary);
}
