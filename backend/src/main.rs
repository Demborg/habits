use std::path::PathBuf;

use axum::{routing::get, Router};
use axum_extra::routing::SpaRouter;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder="dist")] frontend: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .merge(SpaRouter::new("/", frontend).index_file("index.html"));


    Ok(router.into())
}