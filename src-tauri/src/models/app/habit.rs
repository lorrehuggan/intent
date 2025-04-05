use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub enum HabitStatus {
    Completed,
    OnGoing,
    Archived,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub enum HabitStreak {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub enum HabitTheme {
    Red,
    Green,
    Blue,
    Rose,
    Mint,
    Sky,
    Amber,
    Indigo,
    Neutral,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub enum HabitCategory {
    Health,
    Fitness,
    Nutrition,
    Leisure,
    Productivity,
    Finace,
    PersonalGrowth,
    Social,
    Creative,
    Study,
    Home,
    Work,
    School,
    Morning,
    Afternoon,
    Evening,
    Night,
    Other,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub enum HabitReminder {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub struct Habit {
    pub title: String,
    pub description: String,
    pub status: HabitStatus,
    pub streak: HabitStreak,
    pub completions: u16,
    pub theme: HabitTheme,
    pub category: HabitCategory,
    pub reminder: Option<HabitReminder>,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
}
