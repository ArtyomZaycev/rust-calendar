use calendar_lib::api::{
    permissions::types::{Permissions, TablePermissions},
    utils::TableId,
};
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbPermission {
    pub id: i32,

    pub user_id: i32,
    pub access_level: i32,

    pub allow_share: bool,

    pub access_levels_create: bool,
    pub access_levels_read: bool,
    pub access_levels_update: bool,
    pub access_levels_delete: bool,

    pub events_create: bool,
    pub events_read: bool,
    pub events_update: bool,
    pub events_delete: bool,

    pub event_templates_create: bool,
    pub event_templates_read: bool,
    pub event_templates_update: bool,
    pub event_templates_delete: bool,

    pub schedules_create: bool,
    pub schedules_read: bool,
    pub schedules_update: bool,
    pub schedules_delete: bool,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::permissions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewPermission {
    pub user_id: i32,
    pub access_level: i32,

    pub allow_share: bool,

    pub access_levels_create: bool,
    pub access_levels_read: bool,
    pub access_levels_update: bool,
    pub access_levels_delete: bool,

    pub events_create: bool,
    pub events_read: bool,
    pub events_update: bool,
    pub events_delete: bool,

    pub event_templates_create: bool,
    pub event_templates_read: bool,
    pub event_templates_update: bool,
    pub event_templates_delete: bool,

    pub schedules_create: bool,
    pub schedules_read: bool,
    pub schedules_update: bool,
    pub schedules_delete: bool,
}

#[derive(diesel::AsChangeset)]
#[diesel(table_name = crate::db::schema::permissions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbUpdatePermission {
    pub id: i32,

    pub access_level: Option<i32>,
    pub allow_share: Option<bool>,

    pub access_levels_create: Option<bool>,
    pub access_levels_read: Option<bool>,
    pub access_levels_update: Option<bool>,
    pub access_levels_delete: Option<bool>,

    pub events_create: Option<bool>,
    pub events_read: Option<bool>,
    pub events_update: Option<bool>,
    pub events_delete: Option<bool>,

    pub event_templates_create: Option<bool>,
    pub event_templates_read: Option<bool>,
    pub event_templates_update: Option<bool>,
    pub event_templates_delete: Option<bool>,

    pub schedules_create: Option<bool>,
    pub schedules_read: Option<bool>,
    pub schedules_update: Option<bool>,
    pub schedules_delete: Option<bool>,
}

impl DbPermission {
    pub fn to_api(self) -> Permissions {
        Permissions {
            access_level: self.access_level,
            access_levels: TablePermissions {
                view: self.access_levels_read,
                edit: self.access_levels_update,
                create: self.access_levels_create,
                delete: self.access_levels_delete,
            },
            events: TablePermissions {
                view: self.events_read,
                edit: self.events_update,
                create: self.events_create,
                delete: self.events_delete,
            },
            event_templates: TablePermissions {
                view: self.event_templates_read,
                edit: self.event_templates_update,
                create: self.event_templates_create,
                delete: self.event_templates_delete,
            },
            schedules: TablePermissions {
                view: self.schedules_read,
                edit: self.schedules_update,
                create: self.schedules_create,
                delete: self.schedules_delete,
            },
            allow_share: self.allow_share,
        }
    }
}

impl DbNewPermission {
    pub fn from_api(user_id: i32, value: Permissions) -> Self {
        Self {
            user_id,
            access_level: value.access_level,

            allow_share: value.allow_share,

            access_levels_create: value.access_levels.create,
            access_levels_read: value.access_levels.view,
            access_levels_update: value.access_levels.edit,
            access_levels_delete: value.access_levels.delete,

            events_create: value.events.create,
            events_read: value.events.view,
            events_update: value.events.edit,
            events_delete: value.events.delete,

            event_templates_create: value.event_templates.create,
            event_templates_read: value.event_templates.view,
            event_templates_update: value.event_templates.edit,
            event_templates_delete: value.event_templates.delete,

            schedules_create: value.schedules.create,
            schedules_read: value.schedules.view,
            schedules_update: value.schedules.edit,
            schedules_delete: value.schedules.delete,
        }
    }
}

impl DbUpdatePermission {
    pub fn from_api(id: TableId, value: Permissions) -> Self {
        Self {
            id,
            access_level: Some(value.access_level),
            allow_share: Some(value.allow_share),

            access_levels_create: Some(value.access_levels.create),
            access_levels_read: Some(value.access_levels.view),
            access_levels_update: Some(value.access_levels.edit),
            access_levels_delete: Some(value.access_levels.delete),

            events_create: Some(value.events.create),
            events_read: Some(value.events.view),
            events_update: Some(value.events.edit),
            events_delete: Some(value.events.delete),

            event_templates_create: Some(value.event_templates.create),
            event_templates_read: Some(value.event_templates.view),
            event_templates_update: Some(value.event_templates.edit),
            event_templates_delete: Some(value.event_templates.delete),

            schedules_create: Some(value.schedules.create),
            schedules_read: Some(value.schedules.view),
            schedules_update: Some(value.schedules.edit),
            schedules_delete: Some(value.schedules.delete),
        }
    }
}
