CREATE TABLE source_tags (
                             id serial primary key,
                             tag text not null,
                             user_id int not null constraint source_tags_user_id references users,
                             source_id int not null constraint source_tags_record_id references sources,
                             unique(tag, user_id, source_id)
);
