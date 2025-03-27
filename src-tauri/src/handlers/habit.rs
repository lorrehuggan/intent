use crate::models::habit::{Category, Habit, HabitStatus, Reminder, Streak, Theme};
use chrono::Utc;

#[tauri::command]
pub fn get_habit() -> Habit {
    Habit {
        title: "test".to_string(),
        description: "test".to_string(),
        status: HabitStatus::OnGoing,
        streak: Streak::Monthly,
        completions: 1,
        theme: Theme::Mint,
        category: Category::Health,
        reminder: Some(Reminder::Mon),
        created: Utc::now(),
        updated: None,
    }
}
