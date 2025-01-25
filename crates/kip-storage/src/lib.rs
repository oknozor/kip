use model::Item;
use once_cell::sync::Lazy;
use std::fmt::Debug;
use std::sync::Arc;
use tracing::info;

use redb::TableDefinition;

pub mod model;
pub static DB: Lazy<Database> = Lazy::new(Database::default);

#[derive(Clone, Debug)]
pub struct Database {
    inner: Arc<redb::Database>,
}

impl Default for Database {
    fn default() -> Self {
        let path = dirs::runtime_dir()
            .expect("Cannot open local data dir")
            .join("kipd");

        std::fs::create_dir_all(&path).expect("Failed to create database directory");
        let path = path.join("db");
        let path = path.as_path();

        info!("Opening database {:?}", path);

        let database = match redb::Database::open(path) {
            Ok(db) => db,
            Err(_err) => redb::Database::create(path).expect("failed to create database"),
        };

        Database {
            inner: Arc::new(database),
        }
    }
}

impl Database {
    pub fn insert(&self, key: &str, value: &Vec<Item>) -> Result<(), redb::Error> {
        let db = self.inner.clone();
        let value = serde_json::to_string(&value).expect("failed to serialize item");
        let write_tnx = db.begin_write()?;
        {
            let definition = TableDefinition::<&str, &str>::new("kipd");
            let mut table = write_tnx.open_table(definition)?;
            table.insert(key, value.as_str())?;
        }
        write_tnx.commit()?;
        Ok(())
    }

    pub fn get_by_key(&self, key: &str) -> Option<Vec<Item>> {
        let definition = TableDefinition::<&str, &str>::new("kipd");
        let db = self.inner.clone();
        let Ok(read_txn) = db.begin_read() else {
            return None;
        };

        let table = read_txn.open_table(definition).ok()?;
        table.get(key).ok().flatten().map(|data| {
            serde_json::from_str::<Vec<Item>>(data.value()).expect("failed to deserialize item")
        })
    }
}
