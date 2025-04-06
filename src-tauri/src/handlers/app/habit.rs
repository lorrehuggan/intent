use crate::{
    models::app::habit::{
        Habit, HabitCategory, HabitReminder, HabitStatus, HabitStreak, HabitTheme,
    },
    AppState,
};
use chrono::Local;

#[tauri::command]
pub fn get_habit() -> Habit {
    Habit {
        id: Some(2),
        title: "test".to_string(),
        description: "test".to_string(),
        status: HabitStatus::OnGoing,
        streak: HabitStreak::Monthly,
        completions: 1,
        theme: HabitTheme::Mint,
        category: HabitCategory::Health,
        reminder: Some(HabitReminder::Mon),
        created: Local::now(),
        updated: None,
    }
}

#[tauri::command]
pub async fn get_all_habits(state: tauri::State<'_, AppState>) -> Result<Vec<Habit>, String> {
    let db = &state.db;

    let habits: Vec<Habit> = sqlx::query_as("SELECT * FROM habit ORDER BY created DESC")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get habits {}", e))?;

    let habits = habits
        .into_iter()
        .filter(|habit| habit.status != HabitStatus::Archived)
        .collect();

    Ok(habits)
}
