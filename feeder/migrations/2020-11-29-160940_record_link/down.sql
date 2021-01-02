ALTER TABLE records drop column external_link;
ALTER TABLE records rename source_record_id to guid;
ALTER TABLE sources drop column external_link;