use calendar_lib::api::schedules::types::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Clone, Serialize, Deserialize)]
pub struct DbSchedule {
    pub id: i32,
    pub user_id: i32,
    pub template_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub first_day: NaiveDate,
    pub last_day: Option<NaiveDate>,
    pub access_level: i32,
    pub deleted: bool,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::schedules)]
pub struct DbNewSchedule {
    pub user_id: i32,
    pub template_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub first_day: NaiveDate,
    pub last_day: Option<NaiveDate>,
    pub access_level: i32,
    pub deleted: bool,
}
/*
#[derive(diesel::AsChangeset)]
#[diesel(table_name = crate::db::schema::schedules)]
#[derive(Clone, Serialize, Deserialize)]
pub struct DbUpdateSchedule {
    pub id: i32,
    pub user_id: Option<i32>,
    pub start: Option<NaiveDateTime>,
    pub weekday_filter: Option<i32>,
    pub day_period: Option<Option<i32>>,
    pub time_period: Option<Option<i32>>,
    pub event_duration: Option<i32>,
    pub deleted: Option<bool>,
} */

impl DbSchedule {
    pub fn to_api(self, events: Vec<EventPlan>) -> Schedule {
        Schedule {
            id: self.id,
            user_id: self.user_id,
            template_id: self.template_id,
            name: self.name,
            description: self.description,
            first_day: self.first_day,
            last_day: self.last_day,
            access_level: self.access_level,
            events,
        }
    }
}

impl DbNewSchedule {
    pub fn from_api(value: NewSchedule) -> Self {
        DbNewSchedule {
            user_id: value.user_id,
            template_id: value.template_id,
            name: value.name,
            description: value.description,
            first_day: value.first_day,
            last_day: value.last_day,
            access_level: value.access_level,
            deleted: false,
        }
    }
}
