CREATE TABLE IF NOT EXISTS temp_auth_sessions
(
    csrf_token UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    nonce      UUID DEFAULT gen_random_uuid() NOT NULL,
    expiry TIMESTAMPTZ                    NOT NULL
);

CREATE INDEX IF NOT EXISTS temp_auth_sessions_expiry_idx ON temp_auth_sessions (expiry);