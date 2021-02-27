CREATE TABLE users (
    id serial primary key,
    last_read_date timestamp not null
);

CREATE TABLE records_meta (
    id serial primary key,
    user_id int not null constraint records_meta_user_id references users,
    record_id int not null constraint records_meta_record_id references records,
    starred bool not null default false,
    unique(user_id, record_id)
);