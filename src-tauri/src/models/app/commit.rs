use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, FromRow)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub struct Commit {
    pub id: Option<u32>,
    pub habit_id: u32,
    pub note: Option<String>,
    pub image: Option<String>,
    pub created: DateTime<Local>,
    pub updated: Option<DateTime<Local>>,
    pub completion: u8,
    pub completed: bool,
    pub completion_date: DateTime<Local>,
}

impl Commit {
    pub fn new(habit_id: u32, completed: bool) -> Self {
        Self {
            id: None,
            habit_id,
            note: None,
            image: None,
            created: Local::now(),
            updated: None,
            completion: 1,
            completed,
            completion_date: Local::now(),
        }
    }
}
