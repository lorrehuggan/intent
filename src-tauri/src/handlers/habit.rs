use serde::Serialize;

#[derive(Serialize)]
pub enum HabitStatus {
    Completed,
    OnGoing,
    Archived,
}

#[derive(Serialize)]
pub struct Habit {
    title: String,
    description: String,
    status: HabitStatus,
}

#[tauri::command]
pub fn get_habit() -> Habit {
    let habit = Habit {
        title: "test".to_string(),
        description: "test".to_string(),
        status: HabitStatus::Archived,
    };

    habit
}
