-- Add up migration script here
ALTER TABLE specs ADD PRIMARY KEY (id);
ALTER TABLE specs ADD CONSTRAINT specs_name_length
    CHECK (char_length(name) BETWEEN 1 AND 200);