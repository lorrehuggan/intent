use crate::{models::user::settings::UserSettings, AppState};

#[tauri::command]
pub async fn get_user_settings(state: tauri::State<'_, AppState>) -> Result<UserSettings, String> {
    let db = &state.db;
    let query = "SELECT * FROM user_settings";

    let settings: UserSettings = sqlx::query_as(query)
        .fetch_one(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(settings)
}

#[tauri::command]
pub async fn set_theme(state: tauri::State<'_, AppState>, theme: String) -> Result<&str, String> {
    let db = &state.db;
    let query = "UPDATE user_settings SET theme = $1 WHERE id = 1";
    sqlx::query(query)
        .bind(theme)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok("ok")
}

#[tauri::command]
pub async fn set_highlight_current_day(
    state: tauri::State<'_, AppState>,
    highlight_current_day: bool,
) -> Result<&str, String> {
    let db = &state.db;
    let query = "UPDATE user_settings SET highlight_current_day = $1 WHERE id = 1";
    sqlx::query(query)
        .bind(highlight_current_day)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok("ok")
}

#[tauri::command]
pub async fn set_user_settings(
    state: tauri::State<'_, AppState>,
    payload: UserSettings,
) -> Result<&str, String> {
    let db = &state.db;

    let query = "UPDATE user_settings SET theme = $1, highlight_current_day = $2, show_category_filter = $3, default_timeline = $4 WHERE id = 1";

    sqlx::query(query)
        .bind(payload.theme)
        .bind(payload.highlight_current_day)
        .bind(payload.show_category_filter)
        .bind(payload.default_timeline)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update user settings {}", e))?;

    Ok("ok")
}
