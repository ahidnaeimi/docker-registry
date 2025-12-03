use axum::{Json};
use serde_json::json;

pub async fn get_catalog() -> Json<serde_json::Value> {
    Json(json!({
        "repositories": []
    }))
}
