use serde::{Serialize};
use serde::de::DeserializeOwned;

use std::path::PathBuf;
use std::fmt::Debug;

use accounting::{Entry, TelegramId, User, UserId, Category};
use accounting::statistics::Statistics;
use error::{Error, ErrorKind};
use persistence::{Migration, Table};

mod table;

use self::table::{RawEntry, RawUser, RawCategory};

#[derive(Debug)]
pub struct Registry {
    path: PathBuf,
    entries: Table<RawEntry, Entry>,
    users: Table<RawUser, User>,
    categories: Table<RawCategory, Category>
}

impl Registry {
    pub fn new(path: PathBuf) -> Result<Registry, Error> {
        debug!("creating registry at {:?}", &path);
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
        debug!("finding or creating user with {:?}", &telegram_id);
        let users: Vec<User> = self.users.select(|user| user.telegram_id == Some(telegram_id))?;
        let user = match users.into_iter().next() {
            None => {
                let new_user = User::with_telegram_id(telegram_id);
                self.users.insert(new_user.clone())?;
                new_user
            },
            Some(user) => user
        };
        Ok(user)
    }

    pub fn add_entry(&self, entry: Entry) -> Result<(), Error> {
        debug!("adding entry {:?}", &entry);
        self.entries.insert(entry)?;
        Ok(())
    }

    pub fn list(&self, user: UserId) -> Result<Vec<Entry>, Error> {
        debug!("listing entries for {:?}", &user);
        let entries = self.entries.select(|e| e.user_id == user)?;
        Ok(entries)
    }

    pub fn list_users(&self) -> Result<Vec<User>, Error> {
        debug!("listing users");
        let users = self.users.select(|_| true)?;
        Ok(users)
    }

    pub fn update_user<F: Fn(&mut User)>(&self, user: UserId, update: F) -> Result<(), Error> {
        debug!("updating user {:?}", &user);
        self.users.update(|u| u.id == user, update)?;
        Ok(())
    }

    pub fn migrate_entries(&self, migration: Migration) -> Result<(), Error> {
        debug!("migrating entries with {:?}", &migration);
        self.entries.migrate(migration)?;
        Ok(())
    }

    pub fn categories(&self, user: UserId) -> Result<Vec<Category>, Error> {
        debug!("listing categories for {:?}", &user);
        let categories = self.categories.select(|c| c.user_id == user)?;
        Ok(categories)
    }

    pub fn add_category(&self, user: UserId, product_name: String, category_name: String) -> Result<(), Error> {
        debug!("adding categories for {:?}: {} - {}", &user, &product_name, &category_name);
        let existing = self.categories(user.clone())?;
        if let Some(existing) = existing.iter()
            .filter(|c| c.product == product_name)
            .next() {
            return Err(ErrorKind::ProductAlreadyHasCategory(existing.product.to_owned(), existing.category.to_owned()).into());        
        }  
        let new_category = Category::new(user, product_name, category_name);
        self.categories.insert(new_category)?;      
        Ok(())
    }

    pub fn statistics(&self, user: UserId) -> Result<Statistics, Error> {
        debug!("getting statistics for {:?}", &user);
        let entries = self.list(user.clone())?;
        let categiries = self.categories(user.clone())?;
        Ok(Statistics::new(entries, categiries))
    }
}

fn table<P: Serialize + DeserializeOwned + Debug + Into<R> + From<R>, R: Debug>(base_path: PathBuf, table_name: &str) -> Result<Table<P, R>, Error> {
    let table: Table<P, R> = if ::persistence::exist_with_name(&base_path, table_name) {
        Table::load(base_path, table_name)?
    } else {
        Table::create(base_path, table_name)?
    };
    Ok(table)
}