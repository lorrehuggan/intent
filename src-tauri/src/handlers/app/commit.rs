use crate::{models::app::commit::Commit, AppState};

#[tauri::command]
pub async fn create_commit(
    state: tauri::State<'_, AppState>,
    habit_id: u32,
    completed: bool,
) -> Result<&str, String> {
    let db = &state.db;

    let commit = Commit::new(habit_id, completed);

    let query = "
        INSERT INTO habit_commit (habit_id, created, completion, completed, completion_date)
        VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(habit_id)
        .bind(commit.created)
        .bind(1)
        .bind(completed)
        .bind(commit.completion_date)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok("ok")
}

#[tauri::command]
pub async fn delete_commit(
    state: tauri::State<'_, AppState>,
    commit_id: u32,
) -> Result<&str, String> {
    let db = &state.db;

    let query = "DELETE FROM habit_commit WHERE id = $1";

    sqlx::query(query)
        .bind(commit_id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete commit {}", e))?;

    Ok("ok")
}

#[tauri::command]
pub async fn get_habit_commits(
    state: tauri::State<'_, AppState>,
    habit_id: u32,
) -> Result<Vec<Commit>, String> {
    let db = &state.db;

    let query = "SELECT * FROM habit_commit WHERE habit_id = $1";

    let commits = sqlx::query_as::<_, Commit>(query)
        .bind(habit_id)
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get commits {}", e))?;

    Ok(commits)
}
