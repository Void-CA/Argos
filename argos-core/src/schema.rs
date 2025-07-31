// @generated automatically by Diesel CLI.

diesel::table! {
    log_sessions (id) {
        id -> Nullable<Text>,
        process_pid -> Integer,
        started_at -> Nullable<Timestamp>,
        duration_secs -> Nullable<Integer>,
        iterations -> Nullable<Integer>,
    }
}

diesel::table! {
    processes (pid) {
        pid -> Nullable<Integer>,
        name -> Text,
        state -> Text,
        memory_mb -> Nullable<Float>,
        start_time -> Nullable<Integer>,
        parent_pid -> Nullable<Integer>,
    }
}

diesel::table! {
    samples (id) {
        id -> Nullable<Integer>,
        log_id -> Text,
        timestamp -> Float,
        cpu_usage -> Float,
        memory -> Integer,
    }
}

diesel::joinable!(log_sessions -> processes (process_pid));
diesel::joinable!(samples -> log_sessions (log_id));

diesel::allow_tables_to_appear_in_same_query!(
    log_sessions,
    processes,
    samples,
);
