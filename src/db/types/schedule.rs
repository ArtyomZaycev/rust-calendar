use std::num::{NonZeroU16, NonZeroU32};

use calendar_lib::api::schedules::types::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Clone, Serialize, Deserialize)]
pub struct DbSchedule {
    pub id: i32,
    pub user_id: i32,
    pub start: NaiveDateTime,
    pub weekday_filter: i32,
    pub day_period: Option<i32>,
    pub time_period: Option<i32>,
    pub event_duration: i32,
    pub deleted: bool,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::schedules)]
pub struct DbNewSchedule {
    pub user_id: i32,
    pub start: NaiveDateTime,
    pub weekday_filter: i32,
    pub day_period: Option<i32>,
    pub time_period: Option<i32>,
    pub event_duration: i32,
    pub deleted: bool,
}

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
}

impl From<Schedule> for DbSchedule {
    fn from(value: Schedule) -> Self {
        DbSchedule {
            id: value.id,
            user_id: value.user_id,
            start: value.start,
            weekday_filter: value.weekday_filter.0 as i32,
            day_period: value.day_period.map(|v| v.get() as i32),
            time_period: value.time_period.map(|v| v.get() as i32),
            event_duration: value.event_duration as i32,
            deleted: false,
        }
    }
}
impl From<DbSchedule> for Schedule {
    fn from(value: DbSchedule) -> Self {
        Schedule {
            id: value.id,
            user_id: value.user_id,
            start: value.start,
            weekday_filter: WeekdayMask(value.weekday_filter as u8),
            day_period: value.day_period.and_then(|v| NonZeroU32::new(v as u32)),
            time_period: value.time_period.and_then(|v| NonZeroU16::new(v as u16)),
            event_duration: value.event_duration as u16,
        }
    }
}

impl From<NewSchedule> for DbNewSchedule {
    fn from(value: NewSchedule) -> Self {
        DbNewSchedule {
            user_id: value.user_id,
            start: value.start,
            weekday_filter: value.weekday_filter.0 as i32,
            day_period: value.day_period.map(|v| v.get() as i32),
            time_period: value.time_period.map(|v| v.get() as i32),
            event_duration: value.event_duration as i32,
            deleted: false,
        }
    }
}
impl From<DbNewSchedule> for NewSchedule {
    fn from(value: DbNewSchedule) -> Self {
        NewSchedule {
            user_id: value.user_id,
            start: value.start,
            weekday_filter: WeekdayMask(value.weekday_filter as u8),
            day_period: value.day_period.and_then(|v| NonZeroU32::new(v as u32)),
            time_period: value.time_period.and_then(|v| NonZeroU16::new(v as u16)),
            event_duration: value.event_duration as u16,
        }
    }
}

impl From<UpdateSchedule> for DbUpdateSchedule {
    fn from(value: UpdateSchedule) -> Self {
        DbUpdateSchedule {
            id: value.id,
            user_id: value.user_id,
            start: value.start,
            weekday_filter: value.weekday_filter.map(|v| v.0 as i32),
            day_period: value.day_period.map(|v| v.map(|v| v.get() as i32)),
            time_period: value.time_period.map(|v| v.map(|v| v.get() as i32)),
            event_duration: value.event_duration.map(|v| v as i32),
            deleted: None,
        }
    }
}
impl From<DbUpdateSchedule> for UpdateSchedule {
    fn from(value: DbUpdateSchedule) -> Self {
        UpdateSchedule {
            id: value.id,
            user_id: value.user_id,
            start: value.start,
            weekday_filter: value.weekday_filter.map(|v| WeekdayMask(v as u8)),
            day_period: value
                .day_period
                .map(|v| v.and_then(|v| NonZeroU32::new(v as u32))),
            time_period: value
                .time_period
                .map(|v| v.and_then(|v| NonZeroU16::new(v as u16))),
            event_duration: value.event_duration.map(|v| v as u16),
        }
    }
}
