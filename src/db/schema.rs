// @generated automatically by Diesel CLI.

diesel::table! {
    access_levels (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 40]
        name -> Varchar,
        level -> Integer,
    }
}

diesel::table! {
    event_plans (id) {
        id -> Integer,
        schedule_id -> Integer,
        weekday -> Tinyint,
        time -> Smallint,
    }
}

diesel::table! {
    event_templates (id) {
        id -> Integer,
        user_id -> Integer,
        access_level -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        event_name -> Varchar,
        #[max_length = 255]
        event_description -> Nullable<Varchar>,
        duration -> Integer,
    }
}

diesel::table! {
    events (id) {
        id -> Integer,
        user_id -> Integer,
        access_level -> Integer,
        visibility -> Tinyint,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        start -> Timestamp,
        end -> Timestamp,
        plan_id -> Nullable<Integer>,
    }
}

diesel::table! {
    granted_permissions (id) {
        id -> Integer,
        giver_user_id -> Integer,
        receiver_user_id -> Integer,
        permissions_id -> Integer,
    }
}

diesel::table! {
    permissions (id) {
        id -> Integer,
        access_level -> Integer,
        access_levels_create -> Bool,
        access_levels_read -> Bool,
        access_levels_update -> Bool,
        access_levels_delete -> Bool,
        events_create -> Bool,
        events_read -> Bool,
        events_update -> Bool,
        events_delete -> Bool,
        event_templates_create -> Bool,
        event_templates_read -> Bool,
        event_templates_update -> Bool,
        event_templates_delete -> Bool,
        schedules_create -> Bool,
        schedules_read -> Bool,
        schedules_update -> Bool,
        schedules_delete -> Bool,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        #[max_length = 40]
        name -> Varchar,
    }
}

diesel::table! {
    schedules (id) {
        id -> Integer,
        user_id -> Integer,
        access_level -> Integer,
        template_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        first_day -> Date,
        last_day -> Nullable<Date>,
        deleted -> Bool,
    }
}

diesel::table! {
    sessions (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 64]
        key -> Binary,
        start -> Timestamp,
        end -> Timestamp,
        valid -> Bool,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Integer,
        user_id -> Integer,
        role_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
    }
}

diesel::joinable!(access_levels -> users (user_id));
diesel::joinable!(event_plans -> schedules (schedule_id));
diesel::joinable!(event_templates -> users (user_id));
diesel::joinable!(events -> event_plans (plan_id));
diesel::joinable!(events -> users (user_id));
diesel::joinable!(granted_permissions -> permissions (permissions_id));
diesel::joinable!(schedules -> event_templates (template_id));
diesel::joinable!(schedules -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_levels,
    event_plans,
    event_templates,
    events,
    granted_permissions,
    permissions,
    roles,
    schedules,
    sessions,
    user_roles,
    users,
);
