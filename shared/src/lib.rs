use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Cadance {
   Daily,
   Weekly,
   Monthly,
}

impl std::fmt::Display for Cadance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match * self {
            Cadance::Daily => "daily",
            Cadance::Weekly => "weekly",
            Cadance::Monthly => "monthly",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Habit {
    pub name: String,
    pub desciription: String,
    pub cadance: Cadance,
    pub reps: i32,
}