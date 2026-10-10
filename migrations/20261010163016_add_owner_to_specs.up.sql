-- Add up migration script here
DELETE FROM specs;

ALTER TABLE specs
    ADD COLUMN owner_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE;

CREATE INDEX specs_owner_created_idx ON specs (owner_id, created_at DESC);