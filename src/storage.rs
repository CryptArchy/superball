extern crate time;

use std::path::Path;
use time::Timespec;
use rusqlite::Connection;

#[derive(Debug)]
pub struct KeyValue {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub dt_created: Timespec,
}

pub struct Storage {
    db: Connection,
}

impl Storage {
    pub fn new(db_name: &str) -> Storage {
        let db_path = Path::new(db_name);
        let db = Connection::open(db_path).unwrap();

        let sql_create_tbl = r"CREATE TABLE keyvalue (
            id           INTEGER PRIMARY KEY,
            key          TEXT NOT NULL,
            value        TEXT NOT NULL,
            dt_created   TEXT NOT NULL )";

        db.execute(sql_create_tbl, &[])
            .unwrap();

        Storage { db: db }
    }

    pub fn get(&self, key: String) -> KeyValue {
        self.db
            .query_row("SELECT id, key, value, dt_created FROM keyvalue WHERE key=$1",
                       &[&key],
                       |row| {
                KeyValue {
                    id: row.get(0),
                    key: row.get(1),
                    value: row.get(2),
                    dt_created: row.get(3),
                }
            })
            .unwrap()
    }

    pub fn set(&self, kv: &KeyValue) {
        self.db
            .execute("INSERT INTO keyvalue (key, value, dt_created) VALUES ($1, $2, $3)",
                     &[&kv.key, &kv.value, &kv.dt_created])
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    extern crate time;

    use rusqlite::Connection;
    use std::fs;
    use storage::{Storage, KeyValue};

    #[test]
    fn read_and_write_to_disk() {
        fs::remove_file("test.db");
        let store = Storage::new("test.db");
        let kv = KeyValue {
            id: 0,
            key: "Testing".to_string(),
            value: "Testing".to_string(),
            dt_created: time::get_time(),
        };
        store.set(&kv);
        let kv2 = store.get("Testing".to_owned());
        assert!(kv.id != kv2.id);
        assert_eq!(kv.key, kv2.key);
        assert_eq!(kv.value, kv2.value);
        assert_eq!(kv.dt_created.sec, kv2.dt_created.sec);
        fs::remove_file("test.db");
    }

    #[test]
    fn in_memory_write_read_keyvalue() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute("CREATE TABLE keyvalue (
            id           INTEGER PRIMARY KEY,
            key          TEXT NOT NULL,
            value        TEXT NOT NULL,
            dt_created   TEXT NOT NULL
        )",
                     &[])
            .unwrap();

        let kv = KeyValue {
            id: 0,
            key: "Testing".to_string(),
            value: "Testing".to_string(),
            dt_created: time::get_time(),
        };

        conn.execute("INSERT INTO keyvalue (key, value, dt_created)
            VALUES ($1, $2, $3)",
                     &[&kv.key, &kv.value, &kv.dt_created])
            .unwrap();

        let mut stmt = conn.prepare("SELECT id, key, value, dt_created FROM keyvalue").unwrap();
        let kv_iter = stmt.query_map(&[], |row| {
                KeyValue {
                    id: row.get(0),
                    key: row.get(1),
                    value: row.get(2),
                    dt_created: row.get(3),
                }
            })
            .unwrap();

        for kv in kv_iter {
            match kv {
                Ok(kv) => assert_eq!(kv.key, kv.value),
                Err(e) => assert!(false, e),
            }
        }
    }
}
