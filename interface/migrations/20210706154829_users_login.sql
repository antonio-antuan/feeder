ALTER TABLE users add column login text not null unique;
ALTER TABLE users add column password text not null;