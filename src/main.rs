use std::path::PathBuf;

use axum::{routing::get, Router, Json, response::{Response, IntoResponse}};
use axum_extra::routing::SpaRouter;

use shared::{self, Habit};

async fn habits() -> Response {
    let habits = vec![
         Habit {name: "run".to_string(), desciription: "".to_string(), cadance: shared::Cadance::Weekly, reps: 1},
         Habit {name: "iron".to_string(), desciription: "eat iron suplements".to_string(), cadance: shared::Cadance::Daily, reps: 1},
    ];
    return Json(habits).into_response();
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder="frontend/dist")] frontend: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/habits", get(habits))
        .merge(SpaRouter::new("/", frontend).index_file("index.html"));
    Ok(router.into())
}