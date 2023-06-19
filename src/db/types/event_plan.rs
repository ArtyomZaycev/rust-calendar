use calendar_lib::api::schedules::types::*;
use chrono::NaiveTime;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbEventPlan {
    pub id: i32,
    pub schedule_id: i32,
    pub weekday: i8,
    pub time: i16,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::event_plans)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewEventPlan {
    pub schedule_id: i32,
    pub weekday: i8,
    pub time: i16,
}

impl DbEventPlan {
    pub fn to_api(self) -> EventPlan {
        EventPlan {
            id: self.id,
            weekday: chrono::Weekday::from_i8(self.weekday).unwrap(),
            time: NaiveTime::from_num_seconds_from_midnight_opt(self.time as u32 * 60, 0).unwrap(),
        }
    }
}

impl DbNewEventPlan {
    pub fn from_api(value: NewEventPlan, schedule_id: i32) -> Self {
        DbNewEventPlan {
            schedule_id,
            weekday: value.weekday.num_days_from_monday() as i8,
            time: value
                .time
                .signed_duration_since(NaiveTime::default())
                .num_minutes() as i16,
        }
    }
}
