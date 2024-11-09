CREATE TABLE IF NOT EXISTS users
(
    id            UUID         DEFAULT gen_random_uuid() PRIMARY KEY,
    name          TEXT                         NOT NULL,
    email         TEXT                         NOT NULL UNIQUE,
    apple_subject TEXT UNIQUE NULLS NOT DISTINCT,
    theme         VARCHAR(255) DEFAULT 'lunar' NOT NULL,
    timezone      TEXT         DEFAULT 'UTC'   NOT NULL
);