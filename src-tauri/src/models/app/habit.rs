use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type}; // 👈 This line is the fix
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub enum HabitStatus {
    Completed,
    OnGoing,
    Archived,
}

#[derive(Serialize, Deserialize, TS, Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub enum HabitStreak {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Serialize, Deserialize, TS, Type)]
#[sqlx(type_name = "TEXT")]
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

#[derive(Serialize, Deserialize, TS, Type)]
#[sqlx(type_name = "TEXT")]
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

#[derive(Serialize, Deserialize, TS, Type)]
#[sqlx(type_name = "TEXT")]
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

#[derive(Serialize, Deserialize, TS, FromRow)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub struct Habit {
    pub id: Option<u32>,
    pub title: String,
    pub description: String,
    pub status: HabitStatus,
    pub streak: HabitStreak,
    pub completions: u8,
    pub theme: HabitTheme,
    pub category: HabitCategory,
    pub reminder: Option<HabitReminder>,
    pub created: DateTime<Local>,
    pub updated: Option<DateTime<Local>>,
}
