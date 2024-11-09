CREATE TABLE IF NOT EXISTS entries
(
    id                UUID PRIMARY KEY,
    author            UUID   NOT NULL REFERENCES users ON DELETE CASCADE,
    date              DATE   NOT NULL,
    emotion_scale     FLOAT4 NOT NULL CHECK (emotion_scale >= 0 AND emotion_scale <= 10),
    encrypted_content BYTEA  NOT NULL,
    content_key       BYTEA  NOT NULL,
    nonce             BYTEA  NOT NULL,
    UNIQUE (author, date)
);

CREATE INDEX IF NOT EXISTS idx_entries_author_date ON entries (author, date DESC);
CREATE INDEX IF NOT EXISTS idx_entries_author_date_id ON entries (author, date DESC, id DESC);