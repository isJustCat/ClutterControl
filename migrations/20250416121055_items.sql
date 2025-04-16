CREATE TABLE IF NOT EXISTS items
{
    uuid    TEXT    NOT NULL,
    name    TEXT    NOT NULL,
    description    TEXT,
    owner   TEXT,
    tags    TEXT,
    unique(uuid),
    unique(name),
};