CREATE TABLE IF NOT EXISTS mobile_nonce_oneshots
(
    nonce   UUID        NOT NULL,
    payload TEXT        NOT NULL,
    expiry  TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS mobile_nonce_oneshots_nonce_idx ON mobile_nonce_oneshots (nonce);