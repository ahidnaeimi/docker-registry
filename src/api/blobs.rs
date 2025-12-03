use axum::{
    extract::{Path},
    Json,
};
use serde_json::json;

pub async fn get_blob(
    Path((_name, _digest)): Path<(String, String)>
) -> Json<serde_json::Value> {
    Json(json!({
        "status": "get blob (not implemented)"
    }))
}

pub async fn start_upload(
    Path(_name): Path<String>
) -> Json<serde_json::Value> {
    Json(json!({
        "status": "start upload (not implemented)",
        "upload_id": "fake-uuid"
    }))
}

pub async fn patch_upload(
    Path((_name, _uuid)): Path<(String, String)>
) -> Json<serde_json::Value> {
    Json(json!({
        "status": "patch upload (not implemented)"
    }))
}

pub async fn finish_upload(
    Path((_name, _uuid)): Path<(String, String)>
) -> Json<serde_json::Value> {
    Json(json!({
        "status": "finish upload (not implemented)"
    }))
}
