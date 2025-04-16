-- Add migration script here
CREATE TABLE IF NOT EXISTS locations {
    uuid    TEXT    NOT NULL,
    name    TEXT    NOT NULL,
    description     TEXT,
    parent  TEXT,
    owner   TEXT,
    tags,   text,
    unique(uuid),
    unique(name)
};