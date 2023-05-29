use std::path::PathBuf;

use axum::{routing::{get, post}, Router, Json, response::{Response, IntoResponse}, extract::State};
use axum_extra::routing::SpaRouter;

use shared::{Habit, Cadance};
use sqlx::{PgPool, Row};

async fn habits(State(pool): State<PgPool>) -> Response {
    let rows = sqlx::query(
        "SELECT name, description, cadence, reps FROM habits",
    )
    .fetch_all(&pool)
    .await
    .map_err(shuttle_runtime::CustomError::new).unwrap();

    let habits = rows.into_iter().map(|row| {
        Habit {name: row.get("name"), desciription: row.get("description"), cadance: Cadance::from(row.get("cadence")).unwrap(), reps: row.get("reps")}
    });

    let habits: Vec<Habit> = habits.collect();
    return Json(habits).into_response();
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder="./dist")] frontend: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&pool).await.map_err(shuttle_runtime::CustomError::new)?;

    let router = Router::new()
        .route("/habits", get(habits))
        .route("/habit", post(create_habit))
        .merge(SpaRouter::new("/", frontend).index_file("index.html"))
        .with_state(pool);
    Ok(router.into())
}

async fn create_habit(State(pool): State<PgPool>, Json(habit): Json<Habit>) -> (){
    sqlx::query("INSERT INTO habits (name, description, cadence, reps) VALUES ($1, $2, $3, $4)")
        .bind(habit.name)
        .bind(habit.desciription)
        .bind(habit.cadance.to_string())
        .bind(habit.reps)
        .execute(&pool)
        .await;
}
