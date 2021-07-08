ALTER TABLE sources add column last_scrape_time timestamp not null default now();
