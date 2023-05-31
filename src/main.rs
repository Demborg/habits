use std::path::PathBuf;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::routing::SpaRouter;

use shared::{Cadance, Habit, Completions, HabitWithCompletions};
use sqlx::{PgPool, Row};

fn date_pattern(cadance: &Cadance) -> String {
    match cadance {
        Cadance::Daily => "YYYY-MM-DD".to_string(),
        Cadance::Weekly => "YYYY-IW".to_string(),
        Cadance::Monthly => "YYYY-MM".to_string(),
    }
}

async fn get_completions(pool: &PgPool, habit: &Habit) -> Completions {
    let rows = sqlx::query("SELECT TO_CHAR(completion_timestamp, $2) as day, COUNT(completion_timestamp) as count FROM habit_completions INNER JOIN habits ON habits.id = habit_completions.habit_id WHERE name = $1 GROUP BY TO_CHAR(completion_timestamp, $2) ORDER BY MAX(completion_timestamp)")
        .bind(&habit.name)
        .bind(date_pattern(&habit.cadance))
        .fetch_all(pool)
        .await
        .expect("Expect to be able to get completions");

    let completions = rows
        .into_iter()
        .map(|row| (row.get("day"), row.get("count")));
    completions.collect()
}

async fn habits(State(pool): State<PgPool>) -> Response {
    let rows = sqlx::query("SELECT name, description, cadence, reps FROM habits")
        .fetch_all(&pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)
        .unwrap();

    let habits = rows.into_iter().map(|row| Habit {
        name: row.get("name"),
        desciription: row.get("description"),
        cadance: Cadance::from(row.get("cadence")).unwrap(),
        reps: row.get("reps"),
    });

    let mut result: Vec<HabitWithCompletions> = vec![];

    for habit in habits {
        let completions = get_completions(&pool, &habit).await;
        result.push(HabitWithCompletions {habit: habit, completions: completions});
    }

    return Json(result).into_response();
}

async fn create_habit(State(pool): State<PgPool>, Json(habit): Json<Habit>) -> () {
    sqlx::query("INSERT INTO habits (name, description, cadence, reps) VALUES ($1, $2, $3, $4)")
        .bind(habit.name)
        .bind(habit.desciription)
        .bind(habit.cadance.to_string())
        .bind(habit.reps)
        .execute(&pool)
        .await
        .expect("Expected to be able to create habit");
}

async fn complete_habit(State(pool): State<PgPool>, Path(name): Path<String>) -> () {
    sqlx::query("INSERT INTO habit_completions (habit_id) SELECT id FROM habits WHERE name = $1")
        .bind(name)
        .execute(&pool)
        .await
        .expect("Expected to be able to complete habit");
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "./dist")] frontend: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    let router = Router::new()
        .route("/habits", get(habits))
        .route("/habit", post(create_habit))
        .route("/complete/:name", get(complete_habit))
        .merge(SpaRouter::new("/", frontend).index_file("index.html"))
        .with_state(pool);
    Ok(router.into())
}
