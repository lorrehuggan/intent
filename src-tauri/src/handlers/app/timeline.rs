use crate::models::app::timeline::YearTimeline;

#[tauri::command]
pub fn create_year_timeline() -> YearTimeline {
    YearTimeline::default()
}
