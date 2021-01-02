CREATE TABLE sources (
  id serial primary key,
  name text not null,
  origin text not null,
  kind text not null
);


CREATE TABLE records (
  id serial primary key,
  title text,
  guid text not null,
  source_id int not null constraint records_source_id_fk references sources,
  content text not null,
  date timestamp not null
);

create unique index records_unique_guid_source_key on records (guid, source_id);

