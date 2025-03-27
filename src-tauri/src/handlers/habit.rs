use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub enum HabitStatus {
    Completed,
    OnGoing,
    Archived,
}

#[derive(Serialize)]
pub enum Streak {
    Daily,
    ByWeekly,
    Monthly,
    ByMonthly,
}

#[derive(Serialize)]
pub struct Habit {
    title: String,
    description: String,
    status: HabitStatus,
    created: DateTime<Utc>,
    updated: Option<DateTime<Utc>>,
    streak: Streak,
}

#[tauri::command]
pub fn get_habit() -> Habit {
    Habit {
        title: "test".to_string(),
        description: "test".to_string(),
        status: HabitStatus::Archived,
        streak: Streak::Monthly,
        created: Utc::now(),
        updated: None,
    }
}
