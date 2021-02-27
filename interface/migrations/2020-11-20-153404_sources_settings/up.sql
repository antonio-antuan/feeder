CREATE TABLE sources_user_settings (
    id serial primary key,
    user_id int not null constraint sources_user_settings_user_id references users,
    source_id int not null constraint sources_user_settings_source_id references sources,
    unique(user_id, source_id)
);

ALTER TABLE records_meta RENAME TO records_user_settings;