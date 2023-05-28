use serde::Serialize;

#[derive(Serialize)]
pub enum Cadance {
   Daily,
   Weekly,
   Monthly,
}

#[derive(Serialize)]
pub struct Habit {
    pub name: String,
    pub desciription: String,
    pub cadance: Cadance,
    pub reps: i32,
}