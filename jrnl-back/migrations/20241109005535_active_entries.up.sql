CREATE TABLE IF NOT EXISTS active_entries
(
    id            UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    author        UUID        NOT NULL REFERENCES users ON DELETE CASCADE,
    date          DATE        NOT NULL DEFAULT CURRENT_DATE,
    emotion_scale FLOAT4      NOT NULL CHECK (emotion_scale >= 0 AND emotion_scale <= 10),
    text          TEXT,
    expiry        TIMESTAMPTZ NOT NULL,
    ephemeral     BOOLEAN     NOT NULL DEFAULT FALSE,
    UNIQUE (author, date)
);

CREATE INDEX IF NOT EXISTS idx_active_entries_author_date ON entries (author, date DESC);
CREATE INDEX IF NOT EXISTS idx_active_entries_author_date_id ON entries (author, date DESC, id DESC);