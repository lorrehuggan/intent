use crate::{
    models::app::habit::{
        Habit, HabitCategory, HabitReminder, HabitStatus, HabitStreak, HabitTheme,
    },
    AppState,
};
use chrono::Local;

#[tauri::command]
pub fn get_habit() -> Habit {
    todo!()
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

#[tauri::command]
pub async fn create_habit(state: tauri::State<'_, AppState>, habit: Habit) -> Result<&str, String> {
    let db = &state.db;
    let new_habit = Habit {
        id: None,
        title: habit.title,
        description: habit.description,
        status: HabitStatus::OnGoing,
        streak: habit.streak,
        completions: 1,
        completions_needed: 1,
        icon: habit.icon,
        theme: habit.theme,
        category: habit.category,
        reminder: habit.reminder,
        created: Local::now(),
        updated: None,
    };
    let query = "INSERT INTO habit (title, description, status, streak, completions, completions_needed, icon, theme, category, reminder, created, updated) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)";
    sqlx::query(query)
        .bind(new_habit.title)
        .bind(new_habit.description)
        .bind(new_habit.status)
        .bind(new_habit.streak)
        .bind(new_habit.completions)
        .bind(new_habit.completions_needed)
        .bind(new_habit.icon)
        .bind(new_habit.theme)
        .bind(new_habit.category)
        .bind(new_habit.reminder)
        .bind(new_habit.created)
        .bind(new_habit.updated)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok("ok")
}

#[tauri::command]
pub async fn update_habit(state: tauri::State<'_, AppState>, habit: Habit) -> Result<&str, String> {
    let db = &state.db;

    let query = "UPDATE habit SET title = $1, description = $2, status = $3, streak = $4, completions = $5, completions_needed = $12, icon = $11, theme = $6, category = $7, reminder = $8, created = $9, updated = $10 WHERE id = $13";
    let now = Local::now();

    sqlx::query(query)
        .bind(habit.title)
        .bind(habit.description)
        .bind(habit.status)
        .bind(habit.streak)
        .bind(habit.completions)
        .bind(habit.theme)
        .bind(habit.category)
        .bind(habit.reminder)
        .bind(habit.created)
        .bind(now)
        .bind(habit.icon)
        .bind(habit.completions_needed)
        .bind(habit.id.unwrap())
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update habit {}", e))?;

    Ok("ok")
}

#[tauri::command]
pub async fn delete_habit(state: tauri::State<'_, AppState>, id: u32) -> Result<&str, String> {
    let db = &state.db;

    let query1 = "DELETE FROM habit_commit WHERE habit_id = $1";
    let query2 = "DELETE FROM habit WHERE id = $1";

    //TODO: Add transaction

    sqlx::query(query1)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete commits {}", e))?;

    sqlx::query(query2)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete habit {}", e))?;

    Ok("ok")
}
