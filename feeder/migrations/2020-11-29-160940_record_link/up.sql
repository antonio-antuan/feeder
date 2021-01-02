ALTER TABLE records add column external_link text not null default '';
ALTER TABLE records rename guid to source_record_id;

ALTER TABLE sources add column external_link textZ;
UPDATE sources set external_link = origin;
ALTER TABLE sources alter column external_link set not null;
