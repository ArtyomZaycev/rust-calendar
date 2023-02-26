use calendar_lib::api::events::types::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Clone, Serialize, Deserialize)]
pub struct DbEvent {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub access_level: i32,
    pub schedule_id: Option<i32>,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::events)]
pub struct DbNewEvent {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub access_level: i32,
    pub schedule_id: Option<i32>,
}

#[derive(diesel::AsChangeset)]
#[diesel(table_name = crate::db::schema::events)]
#[derive(Clone, Serialize, Deserialize)]
pub struct DbUpdateEvent {
    pub id: i32,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub access_level: Option<i32>,
    pub schedule_id: Option<Option<i32>>,
}

impl From<Event> for DbEvent {
    fn from(value: Event) -> Self {
        DbEvent {
            id: value.id,
            user_id: value.user_id,
            name: value.name,
            description: value.description,
            start: value.start,
            end: value.end,
            access_level: value.access_level,
            schedule_id: value.schedule_id,
        }
    }
}
impl From<DbEvent> for Event {
    fn from(value: DbEvent) -> Self {
        Event {
            id: value.id,
            user_id: value.user_id,
            name: value.name,
            description: value.description,
            start: value.start,
            end: value.end,
            access_level: value.access_level,
            schedule_id: value.schedule_id,
        }
    }
}

impl From<(i32, NewEvent)> for DbNewEvent {
    fn from(value: (i32, NewEvent)) -> Self {
        let (user_id, value) = value;
        DbNewEvent {
            user_id: user_id,
            name: value.name,
            description: value.description,
            start: value.start,
            end: value.end,
            access_level: value.access_level,
            schedule_id: value.schedule_id,
        }
    }
}
impl From<DbNewEvent> for NewEvent {
    fn from(value: DbNewEvent) -> Self {
        NewEvent {
            name: value.name,
            description: value.description,
            start: value.start,
            end: value.end,
            access_level: value.access_level,
            schedule_id: value.schedule_id,
        }
    }
}

impl From<UpdateEvent> for DbUpdateEvent {
    fn from(value: UpdateEvent) -> Self {
        DbUpdateEvent {
            id: value.id,
            user_id: value.user_id,
            name: value.name,
            description: value.description,
            start: value.start,
            end: value.end,
            access_level: value.access_level,
            schedule_id: value.schedule_id,
        }
    }
}
impl From<DbUpdateEvent> for UpdateEvent {
    fn from(value: DbUpdateEvent) -> Self {
        UpdateEvent {
            id: value.id,
            user_id: value.user_id,
            name: value.name,
            description: value.description,
            start: value.start,
            end: value.end,
            access_level: value.access_level,
            schedule_id: value.schedule_id,
        }
    }
}
