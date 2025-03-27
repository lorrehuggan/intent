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
    title: String,
    description: String,
    status: HabitStatus,
    streak: Streak,
    completions: u16,
    theme: Theme,
    category: Category,
    reminder: Option<Reminder>,
    created: DateTime<Utc>,
    updated: Option<DateTime<Utc>>,
}

#[tauri::command]
pub fn get_habit() -> Habit {
    Habit {
        title: "test".to_string(),
        description: "test".to_string(),
        status: HabitStatus::OnGoing,
        streak: Streak::Monthly,
        completions: 1,
        theme: Theme::Mint,
        category: Category::Health,
        reminder: Some(Reminder::Mon),
        created: Utc::now(),
        updated: None,
    }
}
