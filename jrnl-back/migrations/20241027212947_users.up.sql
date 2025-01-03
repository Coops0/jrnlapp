CREATE TABLE IF NOT EXISTS users
(
    id                UUID         DEFAULT gen_random_uuid() PRIMARY KEY,
    name              TEXT                         NOT NULL,
    google_subject    TEXT,
    apple_subject     TEXT,
    theme             VARCHAR(255) DEFAULT 'lunar' NOT NULL,
    timezone          TEXT         DEFAULT 'UTC'   NOT NULL,
    has_had_tour      BOOLEAN      DEFAULT FALSE   NOT NULL,
    has_seen_app_push BOOLEAN      DEFAULT FALSE   NOT NULL,
    CONSTRAINT unique_auth UNIQUE NULLS NOT DISTINCT (google_subject, apple_subject)
);