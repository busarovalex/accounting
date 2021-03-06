use failure::Error as FailureError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::fmt::Debug;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

pub mod error;
mod migrate;

use self::error::PersistenceError;
pub use self::migrate::Migration;

#[derive(Debug)]
pub struct Table<P: Serialize + DeserializeOwned + Debug + Into<R> + From<R>, R: Debug> {
    name: String,
    base_path: PathBuf,
    p_: PhantomData<P>,
    r_: PhantomData<R>,
}

pub fn exist_with_name<P: Into<PathBuf>, S: Into<String>>(path: P, name: S) -> bool {
    let mut full_path = path.into();
    let name = name.into();
    full_path.push(format!("{}.table", &name));
    full_path.is_file()
}

impl<P: Serialize + DeserializeOwned + Debug + Into<R> + From<R>, R: Debug> Table<P, R> {
    pub fn create<T: Into<PathBuf>, S: Into<String>>(
        path: T,
        name: S,
    ) -> Result<Table<P, R>, FailureError> {
        let base_path = path.into();
        let name = name.into();
        info!("creating new table \"{}\" at {:?}", &name, &base_path);
        let mut full_path = base_path.clone();
        let full_name = format!("{}.table", &name);
        full_path.push(full_name);

        if full_path.exists() {
            return Err(PersistenceError::AlreadyInUse {
                used_path: full_path,
            }.into());
        }

        {
            let io_error: FailureError = PersistenceError::IoError.into();
            let parent_directory: &Path = full_path.parent().ok_or(io_error)?;
            fs::create_dir_all(parent_directory)?;
        }

        let _ = File::create(full_path)?;

        Ok(Table {
            name,
            base_path,
            p_: PhantomData,
            r_: PhantomData,
        })
    }

    pub fn load<T: Into<PathBuf>, S: Into<String>>(
        path: T,
        name: S,
    ) -> Result<Table<P, R>, FailureError> {
        let base_path = path.into();
        let name = name.into();
        info!("loading existing table \"{}\" at {:?}", &name, &base_path);
        Ok(Table {
            name: name,
            base_path,
            p_: PhantomData,
            r_: PhantomData,
        })
    }

    pub fn select<F: Fn(&R) -> bool>(&self, predicate: F) -> Result<Vec<R>, FailureError> {
        debug!("selecting data");
        let mut file = self.file_read()?;
        let mut content = String::with_capacity(2048);
        file.read_to_string(&mut content)?;
        let mut entries = Vec::new();
        for line in content.lines() {
            let entry: P = ::serde_json::from_str::<P>(line)?;
            let representation: R = entry.into();
            if predicate(&representation) {
                entries.push(representation);
            }
        }
        Ok(entries)
    }

    pub fn insert(&self, value: R) -> Result<(), FailureError> {
        debug!("inserting data {:?}", value);
        let mut file = self.file_append()?;
        let persistence_entry: P = P::from(value);
        let json_serialized = ::serde_json::to_string(&persistence_entry)?;
        file.write_all(json_serialized.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }

    pub fn update<F: Fn(&R) -> bool, T: Fn(&mut R)>(
        &self,
        predicate: F,
        transformer: T,
    ) -> Result<(), FailureError> {
        debug!("updating data");
        let original_entries = self.select(|_| true)?;
        let mut updated_entries = Vec::new();
        for mut entry in original_entries {
            if predicate(&entry) {
                transformer(&mut entry);
            }
            updated_entries.push(entry);
        }
        self.replace(updated_entries)
    }

    pub fn migrate(&self, migration: Migration) -> Result<(), FailureError> {
        migrate::migrate(self.table_path(), migration)
    }

    fn file_read(&self) -> Result<File, FailureError> {
        let full_path = self.table_path();

        let file = OpenOptions::new().read(true).open(full_path)?;

        Ok(file)
    }

    fn file_append(&self) -> Result<File, FailureError> {
        let full_path = self.table_path();

        let file = OpenOptions::new().append(true).open(full_path)?;

        Ok(file)
    }

    fn replace(&self, new_entries: Vec<R>) -> Result<(), FailureError> {
        let mut backup_path = self.base_path.clone();
        backup_path.push(format!("{}_backup.table", &self.name));
        let mut backup_file = File::create(&backup_path)?;

        for entry in new_entries {
            let persistence_entry: P = P::from(entry);
            let json_serialized = ::serde_json::to_string(&persistence_entry)?;
            backup_file.write_all(json_serialized.as_bytes())?;
            backup_file.write_all(b"\n")?;
        }

        backup_file.flush()?;

        fs::copy(&backup_path, self.table_path())?;

        fs::remove_file(&backup_path)?;

        Ok(())
    }

    fn table_path(&self) -> PathBuf {
        let mut full_path = self.base_path.clone();
        full_path.push(format!("{}.table", &self.name));
        full_path
    }
}
