CREATE TABLE IF NOT EXISTS temp_auth_sessions
(
    id            UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    csrf_token    TEXT        NOT NULL,
    pkce_verifier TEXT        NOT NULL,
    expires_at    TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS temp_auth_sessions_expires_at_idx ON temp_auth_sessions (expires_at);