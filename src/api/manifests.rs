use axum::{
    extract::Path,
    Json,
};
use serde_json::json;

pub async fn get_manifest(
    Path((_name, _reference)): Path<(String, String)>
) -> Json<serde_json::Value> {
    Json(json!({
        "status": "get manifest (not implemented)"
    }))
}

pub async fn put_manifest(
    Path((_name, _reference)): Path<(String, String)>
) -> Json<serde_json::Value> {
    Json(json!({
        "status": "put manifest (not implemented)"
    }))
}

pub async fn delete_manifest(
    Path((_name, _reference)): Path<(String, String)>
) -> Json<serde_json::Value> {
    Json(json!({
        "status": "delete manifest (not implemented)"
    }))
}
