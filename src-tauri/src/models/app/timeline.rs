use chrono::{DateTime, Datelike, Duration, Local, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub struct YearTimeline {
    nodes: HashMap<String, Vec<DateTime<Local>>>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub struct MonthTimeline {
    nodes: Vec<DateTime<Local>>,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/types/bindings.ts")]
pub struct WeekTimeline {
    nodes: Vec<DateTime<Local>>,
}

impl YearTimeline {
    pub fn default() -> Self {
        let mut days_map: HashMap<String, Vec<DateTime<Local>>> = HashMap::new();

        let days_of_week = [
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
        ];

        for day in days_of_week {
            days_map.insert(day.to_string(), Vec::new());
        }

        let today = Local::now();

        for i in 0..52 {
            let current_week_start = today - Duration::weeks(i);

            let week_start = current_week_start
                - Duration::days(current_week_start.weekday().num_days_from_sunday() as i64);

            for (offset, day) in days_of_week.iter().enumerate() {
                let current_date = week_start + Duration::days(offset as i64);

                days_map
                    .get_mut(&day.to_string())
                    .unwrap()
                    .push(current_date);
            }
        }

        YearTimeline { nodes: days_map }
    }
}
