-- Add up migration script here

CREATE TABLE users (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    email         TEXT        NOT NULL CHECK (char_length(email) BETWEEN 3 AND 254),
    password_hash TEXT        NOT NULL,
    display_name  TEXT        CHECK (char_length(display_name) BETWEEN 1 AND 100),
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- email უნიკალურია რეგისტრის მიუხედავად: Soso@mail.com = soso@mail.com
CREATE UNIQUE INDEX users_email_lower_idx ON users (lower(email));