use serde::{Serialize};
use serde::de::DeserializeOwned;

use std::path::{Path, PathBuf};
use std::marker::PhantomData;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::fmt::Debug;

pub mod error;
mod migrate;

pub use self::migrate::{Migration};
use self::error::{Error, ErrorKind};

#[derive(Debug)]
pub struct Table<T: Serialize + DeserializeOwned + Debug> {
    name: String,
    base_path: PathBuf,
    p_: PhantomData<T>
}

pub fn exist_with_name<P: Into<PathBuf>, S: Into<String>>(path: P, name: S) -> bool {
    let mut full_path = path.into();
    let name = name.into();
    full_path.push(format!("{}.table", &name));
    full_path.is_file()
}

impl<T: Serialize + DeserializeOwned + Debug> Table<T> {
    pub fn create<P: Into<PathBuf>, S: Into<String>>(path: P, name: S) -> Result<Table<T>, Error> {
        let base_path = path.into();
        let name = name.into();
        info!("creating new table \"{}\" at {:?}", &name, &base_path);
        let mut full_path = base_path.clone();
        let full_name = format!("{}.table", &name);
        full_path.push(full_name);

        if full_path.exists() {
            return Err(ErrorKind::AlreadyInUse(full_path).into());
        }

        {
            let io_error: Error = ErrorKind::IoError.into();
            let parent_directory: &Path = full_path.parent().ok_or(io_error)?;
            fs::create_dir_all(parent_directory)?;
        }

        let _ = File::create(full_path)?;

        Ok(Table{
            name,
            base_path,
            p_: PhantomData
        })
    }

    pub fn load<P: Into<PathBuf>, S: Into<String>>(path: P, name: S) -> Result<Table<T>, Error> {
        let base_path = path.into();
        let name = name.into();
        info!("loading existing table \"{}\" at {:?}", &name, &base_path);
        Ok(Table{
            name: name,
            base_path,
            p_: PhantomData
        })
    }

    pub fn select(&self) -> Result<Vec<T>, Error> {
        debug!("selecting data");
        let mut file = self.file_read()?;
        let mut content = String::with_capacity(2048);
        file.read_to_string(&mut content)?;
        let mut entries = Vec::new();
        for line in content.lines() {
            let entry: T = ::serde_json::from_str::<T>(line)?;
            entries.push(entry);
        }
        Ok(entries)
    }

    pub fn insert(&self, value: &T) -> Result<(), Error> {
        debug!("inserting data {:?}", value);
        let mut file = self.file_append()?;
        let json_serialized = ::serde_json::to_string(value)?;
        file.write_all(json_serialized.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }

    pub fn migrate(&self, migration: Migration) -> Result<(), Error> {
        migrate::migrate(self.table_path(), migration)
    }

    fn file_read(&self) -> Result<File, Error> {
        let full_path = self.table_path();
        
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)?;     

        Ok(file)
    }

    fn file_append(&self) -> Result<File, Error> {
        let full_path = self.table_path();
        
        let file = OpenOptions::new()
            .append(true)
            .open(full_path)?;     

        Ok(file)
    }

    fn table_path(&self) -> PathBuf {
        let mut full_path = self.base_path.clone();
        full_path.push(format!("{}.table", &self.name));
        full_path
    }
}
