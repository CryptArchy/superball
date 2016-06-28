CREATE TABLE IF NOT EXISTS keyvalue (
    id           INTEGER PRIMARY KEY,
    key          TEXT UNIQUE NOT NULL,
    value        TEXT NOT NULL,
    dt_created   TEXT NOT NULL,
    dt_modified  TEXT NOT NULL 
);
