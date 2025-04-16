CREATE TABLE IF NOT EXISTS user_groups {
uuid    TEXT    NOT NULL,
user_id    TEXT     NOT NULL,
group_id    TEXT    NOT NULL,
unique(uuid)
};--EXT Add migration script here
