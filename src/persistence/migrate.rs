use failure::Error as FailureError;
use serde_json::Value;
use uuid::Uuid;

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use persistence::error::PersistenceError;

#[derive(Debug)]
pub enum Migration {
    RemoveField(String),
    AddField(String, Value),
    GenerateUid(String),
}

pub fn migrate(table_file_path: PathBuf, migration: Migration) -> Result<(), FailureError> {
    info!(
        "migrating table {:?} with migration: {:?}",
        &table_file_path, &migration
    );
    let migration_data = migrate_to_string(table_file_path.clone(), migration)?;
    let mut overridden = File::create(table_file_path)?;
    overridden.write_all(migration_data.as_bytes())?;
    Ok(())
}

fn migrate_to_string(
    table_file_path: PathBuf,
    migration: Migration,
) -> Result<String, FailureError> {
    let mut table_file = File::open(&table_file_path)?;
    let mut content = String::with_capacity(2048);
    table_file.read_to_string(&mut content)?;

    let mut result = String::with_capacity(2048);
    for line in content.lines() {
        let entry: Value = ::serde_json::from_str(line)?;
        trace!("{:?}", &entry);
        let migrated_entry = migration.apply(entry)?;
        let json_serialized = ::serde_json::to_string(&migrated_entry)?;
        result.push_str(&json_serialized);
        result.push('\n');
    }

    Ok(result)
}

impl Migration {
    pub fn remove(field_name: String) -> Migration {
        Migration::RemoveField(field_name)
    }

    pub fn add_from_str(field_name: String, value: &str) -> Result<Migration, FailureError> {
        let value: Value = ::serde_json::from_str(value)?;
        Ok(Migration::AddField(field_name, value))
    }

    pub fn generate_uid(field_name: String) -> Migration {
        Migration::GenerateUid(field_name)
    }

    fn apply(&self, value: Value) -> Result<Value, FailureError> {
        let migrated = match value {
            Value::Object(mut key_value_map) => match self {
                &Migration::RemoveField(ref field_name) => {
                    if !key_value_map.remove(field_name).is_some() {
                        return Err(PersistenceError::NoSuchKeyInJsonValue.into());
                    }
                    Value::Object(key_value_map)
                }
                &Migration::AddField(ref field_name, ref field_value) => {
                    if key_value_map
                        .insert(field_name.clone(), field_value.clone())
                        .is_some()
                    {
                        return Err(PersistenceError::KeyWasAlreadyInObject.into());
                    }
                    Value::Object(key_value_map)
                }
                &Migration::GenerateUid(ref field_name) => {
                    let uid = format!("{}", Uuid::new_v4());
                    if key_value_map
                        .insert(field_name.clone(), Value::String(uid))
                        .is_some()
                    {
                        return Err(PersistenceError::KeyWasAlreadyInObject.into());
                    }
                    Value::Object(key_value_map)
                }
            },
            _ => {
                error!("json value {:?} is not an object", &value);
                return Err(PersistenceError::JsonValueIsNotObject.into());
            }
        };

        Ok(migrated)
    }
}
