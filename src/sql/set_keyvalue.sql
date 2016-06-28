INSERT OR IGNORE INTO keyvalue (key, value, dt_created, dt_modified) 
VALUES ($1, $2, $3, $3);

UPDATE OR IGNORE keyvalue
SET value = $2, 
    dt_modified = $3
WHERE key = $1;
