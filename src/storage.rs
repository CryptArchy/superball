extern crate time;

use std::path::Path;
use time::Timespec;
use rusqlite::Connection;

const CREATE_TBL_KEYVALUE: &'static str = include_str!("sql/create_keyvalue.sql");
const SET_KEYVALUE: &'static str = include_str!("sql/set_keyvalue.sql");
const GET_KEYVALUE: &'static str = include_str!("sql/select_keyvalue_by_key.sql");

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

        db.execute(CREATE_TBL_KEYVALUE, &[])
            .unwrap();

        Storage { db: db }
    }

    pub fn get(&self, key: String) -> KeyValue {
        self.db
            .query_row(GET_KEYVALUE, &[&key], |row| {
                KeyValue {
                    id: row.get("id"),
                    key: row.get("key"),
                    value: row.get("value"),
                    dt_created: row.get("dt_created"),
                }
            })
            .unwrap()
    }

    // @todo: UPDATE still isn't working right, no idea why not!
    pub fn set(&self, kv: &KeyValue) {
        self.db
            .execute(SET_KEYVALUE, &[&kv.key, &kv.value, &kv.dt_created])
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
        let kv1 = KeyValue {
            id: 0,
            key: "Testing".to_string(),
            value: "Testing".to_string(),
            dt_created: time::get_time(),
        };
        let kv2 = KeyValue {
            id: 0,
            key: "Tested".to_string(),
            value: "Rested".to_string(),
            dt_created: time::get_time(),
        };
        store.set(&kv1);
        store.set(&kv2);

        let kv1_out = store.get("Testing".to_owned());
        println!("{:?}", kv1_out);

        assert!(kv1.id != kv1_out.id);
        assert_eq!(kv1_out.id, 1);
        assert_eq!(kv1.key, kv1_out.key);
        assert_eq!(kv1.value, kv1_out.value);
        assert_eq!(kv1.dt_created.sec, kv1_out.dt_created.sec);

        let kv2_out = store.get("Tested".to_owned());
        println!("{:?}", kv2_out);

        assert!(kv2.id != kv2_out.id);
        assert_eq!(kv2_out.id, 2);
        assert_eq!(kv2.key, kv2_out.key);
        assert_eq!(kv2.value, kv2_out.value);
        assert_eq!(kv2.dt_created.sec, kv2_out.dt_created.sec);

        assert!(kv1_out.id != kv2_out.id);
        assert!(kv1_out.key != kv2_out.key);
        assert!(kv1_out.value != kv2_out.value);

        let kv1r =  KeyValue {
            id: 1,
            key: "Testing".to_string(),
            value: "Resting".to_string(),
            dt_created: time::get_time(),
        };

        store.set(&kv1r);
        let kv1r_out = store.get("Testing".to_owned());
        println!("{:?}", kv1r_out);

        assert!(kv1r.id == kv1r_out.id, "Revised ID not transferred!");
        assert!(kv1r.key == kv1r_out.key, "Revised Key not transferred!");
        assert_eq!(kv1r.value, kv1r_out.value);
        assert!(kv1r.value == kv1r_out.value, "Revised value not transferred!");
        assert!(kv1.key == kv1r_out.key, "Revised key not same!");
        assert!(kv1.value != kv1r_out.value, "Revised value was same!");
        assert!(kv1r.dt_created.sec != kv1r_out.dt_created.sec, "Revised time was same!");

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
