// @generated automatically by Diesel CLI.

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
        name -> Varchar,
        event_name -> Varchar,
        event_description -> Nullable<Varchar>,
        duration -> Integer,
        access_level -> Integer,
    }
}

diesel::table! {
    events (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Varchar,
        description -> Nullable<Varchar>,
        start -> Timestamp,
        end -> Timestamp,
        access_level -> Integer,
        visibility -> Tinyint,
        plan_id -> Nullable<Integer>,
    }
}

diesel::table! {
    passwords (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Varchar,
        password -> Varchar,
        access_level -> Integer,
        edit_right -> Bool,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    schedules (id) {
        id -> Integer,
        user_id -> Integer,
        template_id -> Integer,
        name -> Varchar,
        description -> Nullable<Varchar>,
        first_day -> Date,
        last_day -> Nullable<Date>,
        access_level -> Integer,
        deleted -> Bool,
    }
}

diesel::table! {
    sessions (id) {
        id -> Integer,
        user_id -> Integer,
        key -> Binary,
        access_level -> Integer,
        edit_right -> Bool,
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
        granted -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
    }
}

diesel::joinable!(event_plans -> schedules (schedule_id));
diesel::joinable!(event_templates -> users (user_id));
diesel::joinable!(events -> event_plans (plan_id));
diesel::joinable!(events -> users (user_id));
diesel::joinable!(passwords -> users (user_id));
diesel::joinable!(schedules -> event_templates (template_id));
diesel::joinable!(schedules -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    event_plans,
    event_templates,
    events,
    passwords,
    roles,
    schedules,
    sessions,
    user_roles,
    users,
);
