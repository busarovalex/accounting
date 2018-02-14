use serde::{Serialize};
use serde::de::DeserializeOwned;

use std::path::PathBuf;
use std::fmt::Debug;

use accounting::{Entry, TelegramId, User, UserId, Category};
use error::{Error, ErrorKind};
use persistence::{Migration, Table};

mod table;

use self::table::{RawEntry, RawUser, RawCategory};

#[derive(Debug)]
pub struct Registry {
    path: PathBuf,
    entries: Table<RawEntry>,
    users: Table<RawUser>,
    categories: Table<RawCategory>
}

impl Registry {
    pub fn new(path: PathBuf) -> Result<Registry, Error> {
        if !path.is_dir() {
            return Err(ErrorKind::InvalidRegistryPath(path).into());
        }

        let entries = table(path.clone(), "entries")?;
        let users = table(path.clone(), "users")?;
        let categories = table(path.clone(), "categories")?;

        Ok(Registry{
            path,
            entries,
            users,
            categories
        })
    }

    pub fn find_or_create(&self, telegram_id: TelegramId) -> Result<User, Error> {
        let users: Vec<User> = self.users.select()?
            .into_iter()
            .map(|raw| {let user: User = raw.into(); user })
            .filter(|user| user.telegram_id == Some(telegram_id))
            .collect();
        let user = match users.into_iter().next() {
            None => {
                let new_user = User::with_telegram_id(telegram_id);
                self.users.insert(&RawUser::from(new_user.clone()))?;
                new_user
            },
            Some(user) => user
        };
        Ok(user)
    }

    pub fn add_entry(&self, entry: Entry) -> Result<(), Error> {
        self.entries.insert(&RawEntry::from(entry))?;
        Ok(())
    }

    pub fn list(&self, user: UserId) -> Result<Vec<Entry>, Error> {
        let entries = self.entries.select()?
            .into_iter()
            .map(|raw| {let e: Entry = raw.into(); e})
            .filter(|e| e.user_id == user)
            .collect();
        Ok(entries)
    }

    pub fn list_users(&self) -> Result<Vec<User>, Error> {
        let users = self.users.select()?
            .into_iter()
            .map(RawUser::into)
            .collect();
        Ok(users)
    }

    pub fn migrate_entries(&self, migration: Migration) -> Result<(), Error> {
        self.entries.migrate(migration)?;
        Ok(())
    }

    pub fn categories(&self, user: UserId) -> Result<Vec<Category>, Error> {
        let categories = self.categories.select()?
            .into_iter()
            .map(|raw| {let c: Category = raw.into(); c})
            .filter(|c| c.user_id == user)
            .collect();
        Ok(categories)
    }

    pub fn add_category(&self, user: UserId, product_name: String, category_name: String) -> Result<(), Error> {
        let existing = self.categories(user.clone())?;
        if let Some(existing) = existing.iter()
            .filter(|c| c.product == product_name)
            .next() {
            return Err(ErrorKind::ProductAlreadyHasCategory(existing.product.to_owned(), existing.category.to_owned()).into());        
        }  
        let new_category = Category::new(user, product_name, category_name);
        self.categories.insert(&RawCategory::from(new_category))?;      
        Ok(())
    }
}

fn table<T: Serialize + DeserializeOwned + Debug>(base_path: PathBuf, table_name: &str) -> Result<Table<T>, Error> {
    let table: Table<T> = if ::persistence::exist_with_name(&base_path, table_name) {
        Table::load(base_path, table_name)?
    } else {
        Table::create(base_path, table_name)?
    };
    Ok(table)
}
