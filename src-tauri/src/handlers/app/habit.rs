use crate::models::app::habit::{
    Habit, HabitCategory, HabitReminder, HabitStatus, HabitStreak, HabitTheme,
};
use chrono::Utc;

#[tauri::command]
pub fn get_habit() -> Habit {
    Habit {
        title: "test".to_string(),
        description: "test".to_string(),
        status: HabitStatus::OnGoing,
        streak: HabitStreak::Monthly,
        completions: 1,
        theme: HabitTheme::Mint,
        category: HabitCategory::Health,
        reminder: Some(HabitReminder::Mon),
        created: Utc::now(),
        updated: None,
    }
}
