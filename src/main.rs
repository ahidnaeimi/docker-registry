use axum::{Router, serve};
use axum::routing::{get, post, put, patch, delete};
use tokio::net::TcpListener;

mod api;
mod storage;
mod config;
mod errors;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v2/_catalog", get(api::catalog::get_catalog))

        // BLOBS
        .route("/v2/:name/blobs/:digest", get(api::blobs::get_blob))
        .route("/v2/:name/blobs/uploads", post(api::blobs::start_upload))
        .route("/v2/:name/blobs/uploads/:uuid", patch(api::blobs::patch_upload))
        .route("/v2/:name/blobs/uploads/:uuid", put(api::blobs::finish_upload))

        // MANIFESTS
        .route("/v2/:name/manifests/:reference", get(api::manifests::get_manifest))
        .route("/v2/:name/manifests/:reference", put(api::manifests::put_manifest))
        .route("/v2/:name/manifests/:reference", delete(api::manifests::delete_manifest));

    println!("Docker registry listening on 0.0.0.0:5000");
    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();

    serve(listener, app).await.unwrap();
}
