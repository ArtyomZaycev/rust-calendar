use calendar_lib::api::event_templates::types::*;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbEventTemplate {
    pub id: i32,
    pub user_id: i32,
    pub access_level: i32,
    pub name: String,
    pub event_name: String,
    pub event_description: Option<String>,
    pub duration: i32,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::event_templates)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewEventTemplate {
    pub user_id: i32,
    pub access_level: i32,
    pub name: String,
    pub event_name: String,
    pub event_description: Option<String>,
    pub duration: i32,
}

#[derive(diesel::AsChangeset)]
#[diesel(table_name = crate::db::schema::event_templates)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbUpdateEventTemplate {
    pub id: i32,
    pub user_id: Option<i32>,
    pub access_level: Option<i32>,
    pub name: Option<String>,
    pub event_name: Option<String>,
    pub event_description: Option<Option<String>>,
    pub duration: Option<i32>,
}

impl DbEventTemplate {
    pub fn to_api(self) -> EventTemplate {
        EventTemplate {
            id: self.id,
            user_id: self.user_id,
            name: self.name,
            event_name: self.event_name,
            event_description: self.event_description,
            duration: std::time::Duration::from_secs(self.duration as u64 * 60),
            access_level: self.access_level,
        }
    }
}

impl DbNewEventTemplate {
    pub fn from_api(value: NewEventTemplate) -> Self {
        DbNewEventTemplate {
            user_id: value.user_id,
            name: value.name,
            event_name: value.event_name,
            event_description: value.event_description,
            duration: (value.duration.as_secs() / 60) as i32,
            access_level: value.access_level,
        }
    }
}

impl DbUpdateEventTemplate {
    pub fn from_api(value: UpdateEventTemplate) -> Self {
        DbUpdateEventTemplate {
            id: value.id,
            user_id: None,
            name: value.name.option(),
            event_name: value.event_name.option(),
            event_description: value.event_description.option(),
            duration: value.duration.option().map(|d| (d.as_secs() / 60) as i32),
            access_level: value.access_level.option(),
        }
    }
}
