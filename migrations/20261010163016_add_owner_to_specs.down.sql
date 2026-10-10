-- Add down migration script here
DROP INDEX specs_owner_created_idx;
ALTER TABLE specs DROP COLUMN owner_id;