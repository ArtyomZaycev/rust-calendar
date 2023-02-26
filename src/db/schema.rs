// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Varchar,
        description -> Nullable<Varchar>,
        start -> Timestamp,
        end -> Timestamp,
        access_level -> Integer,
        schedule_id -> Nullable<Integer>,
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
        start -> Timestamp,
        weekday_filter -> Integer,
        day_period -> Nullable<Integer>,
        time_period -> Nullable<Integer>,
        event_duration -> Integer,
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

diesel::joinable!(events -> schedules (schedule_id));
diesel::joinable!(events -> users (user_id));
diesel::joinable!(passwords -> users (user_id));
diesel::joinable!(schedules -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    events, passwords, roles, schedules, sessions, user_roles, users,
);
