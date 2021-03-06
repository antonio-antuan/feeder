table! {
    files (id) {
        id -> Int4,
        record_id -> Int4,
        kind -> Text,
        local_path -> Nullable<Text>,
        remote_path -> Text,
        remote_id -> Nullable<Text>,
        file_name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        meta -> Nullable<Text>,
    }
}

table! {
    record_tags (id) {
        id -> Int4,
        tag -> Text,
        user_id -> Int4,
        record_id -> Int4,
    }
}

table! {
    records (id) {
        id -> Int4,
        title -> Nullable<Text>,
        source_record_id -> Text,
        source_id -> Int4,
        content -> Text,
        date -> Timestamp,
        image -> Nullable<Text>,
        external_link -> Text,
    }
}

table! {
    records_user_settings (id) {
        id -> Int4,
        user_id -> Int4,
        record_id -> Int4,
        starred -> Bool,
    }
}

table! {
    sources (id) {
        id -> Int4,
        name -> Text,
        origin -> Text,
        kind -> Text,
        image -> Nullable<Text>,
        last_scrape_time -> Timestamp,
        external_link -> Text,
    }
}

table! {
    sources_user_settings (id) {
        id -> Int4,
        user_id -> Int4,
        source_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        last_read_date -> Timestamp,
        token -> Nullable<Text>,
        login -> Text,
        password -> Text,
    }
}

joinable!(files -> records (record_id));
joinable!(record_tags -> records (record_id));
joinable!(record_tags -> users (user_id));
joinable!(records -> sources (source_id));
joinable!(records_user_settings -> records (record_id));
joinable!(records_user_settings -> users (user_id));
joinable!(sources_user_settings -> sources (source_id));
joinable!(sources_user_settings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    files,
    record_tags,
    records,
    records_user_settings,
    sources,
    sources_user_settings,
    users,
);
