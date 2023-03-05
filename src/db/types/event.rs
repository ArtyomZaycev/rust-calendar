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
    pub visibility: i8,
    pub plan_id: Option<i32>,
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
    pub visibility: i8,
    pub plan_id: Option<i32>,
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
    pub visibility: Option<i8>,
    pub plan_id: Option<Option<i32>>,
}

impl DbEvent {
    pub fn try_to_api(self, access_level: i32) -> Option<Event> {
        EventVisibility::try_from(self.visibility)
            .ok()
            .and_then(|visibility| {
                let event = Event {
                    id: self.id,
                    user_id: self.user_id,
                    name: self.name,
                    description: self.description,
                    start: self.start,
                    end: self.end,
                    access_level: self.access_level,
                    visibility,
                    plan_id: self.plan_id,
                };
                if access_level >= self.access_level {
                    Some(event)
                } else {
                    match visibility {
                        EventVisibility::HideAll => None,
                        EventVisibility::HideName => Some(Event {
                            name: "".to_owned(),
                            description: None,
                            ..event
                        }),
                        EventVisibility::HideDescription => Some(Event {
                            description: None,
                            ..event
                        }),
                        EventVisibility::Show => Some(event),
                    }
                }
            })
    }
}

impl DbNewEvent {
    pub fn from_api(value: NewEvent) -> Self {
        DbNewEvent {
            user_id: value.user_id,
            name: value.name,
            description: value.description,
            start: value.start,
            end: value.end,
            access_level: value.access_level,
            visibility: value.visibility as i8,
            plan_id: value.plan_id,
        }
    }
}

impl DbUpdateEvent {
    pub fn from_api(value: UpdateEvent) -> Self {
        DbUpdateEvent {
            id: value.id,
            user_id: value.user_id.into(),
            name: value.name.into(),
            description: value.description.into(),
            start: value.start.into(),
            end: value.end.into(),
            access_level: value.access_level.into(),
            visibility: value.visibility.option().map(|v| v as i8),
            plan_id: value.plan_id.into(),
        }
    }
}
