CREATE TABLE token_mints (
    pubkey       TEXT      PRIMARY KEY,
    decimals     INT4      NOT NULL,
    supply       INT8,
    slot_updated INT8      NOT NULL
);