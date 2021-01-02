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

joinable!(files -> records (record_id));
joinable!(records -> sources (source_id));

allow_tables_to_appear_in_same_query!(files, records, sources,);
