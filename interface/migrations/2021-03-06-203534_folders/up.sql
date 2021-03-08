CREATE TABLE user_folders (
    id serial primary key,
    name text not null,
    user_id int not null constraint user_folders_user_id references users,
    parent_folder int null constraint user_folders_parent_folder references user_folders,
    unique(name, user_id)
);

CREATE TABLE user_source_to_folder (
    id serial primary key,
    user_source_id int not null constraint user_source_to_folder_user_sources references sources_user_settings,
    folder_id int not null constraint user_source_to_folder_folder_id references user_folders
);