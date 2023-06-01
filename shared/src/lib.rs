use chrono::{TimeZone, Datelike, Local, Duration};
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

pub type Completions = Vec<(String, i64)>;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HabitWithCompletions {
    pub habit: Habit,
    pub completed: i64,
    pub history: Completions
}

impl Cadance {
    pub fn from(text: &str) -> Option<Cadance> {
        match text {
            "daily" => Some(Cadance::Daily),
            "weekly" => Some(Cadance::Weekly),
            "monthly" => Some(Cadance::Monthly),
            _ => None,

        }
    }

    pub fn remaining_percenteage(&self) -> f64 {
        let now = Local::now();
        match self {
            Cadance::Daily => {
                let start= Local.with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0).unwrap();
                1.0 - (start - now).num_minutes() as f64 / (24 * 60) as f64
            },
            Cadance::Weekly => {
                let num_days_since_mon = now.weekday().num_days_from_monday();
                let start= Local.with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0).unwrap() - Duration::days(num_days_since_mon as i64);
                1.0 - (start - now).num_minutes() as f64 / (7 * 24 * 60) as f64
            },
            Cadance::Monthly => {
                let start= Local.with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0).unwrap();
                1.0 - (start - now).num_minutes() as f64 / (31 * 7 * 24 * 60) as f64
            },
        }
        
    }
}

impl HabitWithCompletions {
    pub fn urgency(&self) -> f64 {
        let remaning_work = (self.habit.reps as i64 - self.completed) as f64 / self.habit.reps as f64;
        remaning_work / self.habit.cadance.remaining_percenteage()
    }   
}