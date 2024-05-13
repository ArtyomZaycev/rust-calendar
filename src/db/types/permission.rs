use calendar_lib::api::permissions::types::{Permissions, TablePermissions};
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbPermission {
    pub id: i32,

    pub access_level: i32,

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
    pub access_level: i32,

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
            allow_share: true,
        }
    }
}
