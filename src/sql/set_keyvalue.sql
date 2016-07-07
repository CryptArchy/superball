INSERT OR REPLACE INTO keyvalue (id, key, value, dt_created, dt_modified)
 SELECT old.id,
        new.key,
        new.value,
        coalesce(old.dt_created, new.dt_created),
        new.dt_modified
 FROM ( SELECT
     $1 AS key,
     $2 AS value,
     datetime('now') AS dt_created,
     datetime('now') AS dt_modified
 ) AS new
 LEFT JOIN (
     SELECT id, key, value, dt_created, dt_modified
     FROM keyvalue
 ) AS old ON new.key = old.key;