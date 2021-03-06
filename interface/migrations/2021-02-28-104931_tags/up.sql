CREATE TABLE record_tags (
    id serial primary key,
    tag text not null,
    user_id int not null constraint record_tags_user_id references users,
    record_id int not null constraint record_tags_record_id references records,
    unique(tag, user_id, record_id)
);