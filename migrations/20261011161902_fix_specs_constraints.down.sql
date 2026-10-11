-- Add down migration script here
ALTER TABLE specs DROP CONSTRAINT specs_name_length;
ALTER TABLE specs DROP CONSTRAINT specs_pkey;