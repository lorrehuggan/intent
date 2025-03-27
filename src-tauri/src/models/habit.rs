use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HabitStatus {
    Completed,
    OnGoing,
    Archived,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Streak {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Category {
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Reminder {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    pub title: String,
    pub description: String,
    pub status: HabitStatus,
    pub streak: Streak,
    pub completions: u16,
    pub theme: Theme,
    pub category: Category,
    pub reminder: Option<Reminder>,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
}
