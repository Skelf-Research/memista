//! Integration tests for Memista HTTP API
//!
//! These tests verify the HTTP API endpoints work correctly.
//! Run with: cargo test --test integration

use memista::{AppState, ChunkData, Config, InsertChunkRequest, SearchRequest, create_app};
use actix_web::{test, App};
use async_sqlite::{PoolBuilder, JournalMode};
use std::sync::Arc;

/// Create test application state
async fn create_test_state() -> Arc<AppState> {
    let db_pool = PoolBuilder::new()
        .path(":memory:")
        .journal_mode(JournalMode::Wal)
        .open()
        .await
        .expect("Failed to create test database pool");

    Arc::new(AppState { db_pool })
}

#[actix_web::test]
async fn test_health_check() {
    let app_state = create_test_state().await;
    let app = test::init_service(create_app(app_state)).await;

    let req = test::TestRequest::get()
        .uri("/openapi.json")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_insert_and_search() {
    let app_state = create_test_state().await;
    let app = test::init_service(create_app(app_state)).await;

    // Insert chunks
    let insert_payload = InsertChunkRequest {
        database_id: "test_db".to_string(),
        chunks: vec![
            ChunkData {
                embedding: vec![0.1, 0.2],
                text: "Test chunk one".to_string(),
                metadata: "{\"test\": true}".to_string(),
            },
            ChunkData {
                embedding: vec![0.3, 0.4],
                text: "Test chunk two".to_string(),
                metadata: "{\"test\": true}".to_string(),
            },
        ],
    };

    let req = test::TestRequest::post()
        .uri("/v1/insert")
        .set_json(&insert_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Search for chunks
    let search_payload = SearchRequest {
        database_id: "test_db".to_string(),
        embeddings: vec![vec![0.1, 0.2]],
        num_results: 5,
    };

    let req = test::TestRequest::post()
        .uri("/v1/search")
        .set_json(&search_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_drop_database() {
    let app_state = create_test_state().await;
    let app = test::init_service(create_app(app_state)).await;

    // First insert some data
    let insert_payload = InsertChunkRequest {
        database_id: "drop_test_db".to_string(),
        chunks: vec![ChunkData {
            embedding: vec![0.1, 0.2],
            text: "Test chunk".to_string(),
            metadata: "{}".to_string(),
        }],
    };

    let req = test::TestRequest::post()
        .uri("/v1/insert")
        .set_json(&insert_payload)
        .to_request();

    test::call_service(&app, req).await;

    // Drop the database
    let drop_payload = serde_json::json!({
        "database_id": "drop_test_db"
    });

    let req = test::TestRequest::delete()
        .uri("/v1/drop")
        .set_json(&drop_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_invalid_request() {
    let app_state = create_test_state().await;
    let app = test::init_service(create_app(app_state)).await;

    // Send invalid JSON
    let req = test::TestRequest::post()
        .uri("/v1/insert")
        .set_payload("invalid json")
        .insert_header(("Content-Type", "application/json"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
