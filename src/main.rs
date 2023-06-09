use std::path::PathBuf;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::{get, post, delete},
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

async fn get_history(pool: &PgPool, habit: &Habit) -> Completions {
    let rows = sqlx::query(r#"
        SELECT
            TO_CHAR(completion_timestamp, $2) as day,
            COUNT(completion_timestamp) as count
        FROM habit_completions 
        WHERE habit_id = $1
        GROUP BY TO_CHAR(completion_timestamp, $2)
        ORDER BY MAX(completion_timestamp)"#)
        .bind(&habit.id.unwrap())
        .bind(date_pattern(&habit.cadance))
        .fetch_all(pool)
        .await
        .expect("Expect to be able to get completions");

    let completions = rows
        .into_iter()
        .map(|row| (row.get("day"), row.get("count")));
    completions.collect()
}

async fn get_completed(pool: &PgPool, habit: &Habit) -> i64 {
    sqlx::query(r#"
        SELECT
            COUNT(habit_completions.id) as count
        FROM habit_completions
        WHERE
            habit_id = $1
            AND TO_CHAR(completion_timestamp, $2) = TO_CHAR(NOW(), $2)"#)
        .bind(&habit.id.unwrap())
        .bind(date_pattern(&habit.cadance))
        .fetch_one(pool)
        .await
        .expect("Expect to be able to get completions")
        .get("count")
}

async fn habits(State(pool): State<PgPool>) -> Response {
    let rows = sqlx::query("SELECT id, name, description, cadence, reps, anti_habit FROM habits")
        .fetch_all(&pool)
        .await
        .map_err(shuttle_runtime::CustomError::new)
        .unwrap();

    let habits = rows.into_iter().map(|row| Habit {
        id: row.get("id"),
        name: row.get("name"),
        desciription: row.get("description"),
        cadance: Cadance::from(row.get("cadence")).unwrap(),
        reps: row.get("reps"),
        anti_habit: row.get("anti_habit"),
    });

    let mut result: Vec<HabitWithCompletions> = vec![];

    for habit in habits {
        let completions = get_history(&pool, &habit).await;
        let current = get_completed(&pool, &habit).await;
        result.push(HabitWithCompletions {habit: habit, completed: current, history: completions});
    }

    return Json(result).into_response();
}

async fn create_habit(State(pool): State<PgPool>, Json(habit): Json<Habit>) -> () {
    sqlx::query("INSERT INTO habits (name, description, cadence, reps, anti_habit) VALUES ($1, $2, $3, $4, $5)")
        .bind(habit.name)
        .bind(habit.desciription)
        .bind(habit.cadance.to_string())
        .bind(habit.reps)
        .bind(habit.anti_habit)
        .execute(&pool)
        .await
        .expect("Expected to be able to create habit");
}

async fn complete_habit(State(pool): State<PgPool>, Path(id): Path<i64>) -> () {
    sqlx::query("INSERT INTO habit_completions (habit_id) VALUES ($1)")
        .bind(id)
        .execute(&pool)
        .await
        .expect("Expected to be able to complete habit");
}

async fn delete_habit(State(pool): State<PgPool>, Path(id): Path<i64>) -> () {
    sqlx::query("DELETE FROM habit_completions WHERE habit_id = $1")
        .bind(id.clone())
        .execute(&pool)
        .await
        .expect("Expected to be able to complete habit");
    sqlx::query("DELETE FROM habits WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .expect("Expected to be able to delete habit");
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
        .route("/habit/:id", delete(delete_habit))
        .route("/complete/:id", get(complete_habit))
        .merge(SpaRouter::new("/", frontend).index_file("index.html"))
        .with_state(pool);
    Ok(router.into())
}
