use serde::{Serialize};
use serde::de::DeserializeOwned;

use std::path::PathBuf;

use accounting::{Entry};
use error::{Error, ErrorKind};
use persistence::{Migration, Table};

#[derive(Debug)]
pub struct Registry {
    path: PathBuf,
    entries: Table<Entry>
}

impl Registry {
    pub fn new(path: PathBuf) -> Result<Registry, Error> {
        if !path.is_dir() {
            return Err(ErrorKind::InvalidRegistryPath(path).into());
        }

        let entries = table(path.clone(), "entries")?;

        Ok(Registry{
            path,
            entries
        })
    }

    pub fn add_entry(&self, entry: Entry) -> Result<(), Error> {
        self.entries.insert(&entry)?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Entry>, Error> {
        let entries = self.entries.select()?;
        Ok(entries)
    }

    pub fn migrate_entries(&self, migration: Migration) -> Result<(), Error> {
        self.entries.migrate(migration)?;
        Ok(())
    }
}

fn table<T: Serialize + DeserializeOwned>(base_path: PathBuf, table_name: &str) -> Result<Table<T>, Error> {
    let table: Table<T> = if ::persistence::exist_with_name(&base_path, table_name) {
        Table::load(base_path, table_name)?
    } else {
        Table::create(base_path, table_name)?
    };
    Ok(table)
}
