use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, FromRow)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub struct UserSettings {
    pub id: u8,
    pub theme: String,
    pub highlight_current_day: bool,
    pub show_category_filter: bool,
    pub default_timeline: String,
}
