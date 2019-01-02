use failure::Error as FailureError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::fmt::Debug;
use std::path::PathBuf;

use accounting::statistics::Statistics;
use accounting::{Category, Entry, NewSms, Sms, TelegramId, User, UserId};
use error::AppError;
use persistence::{Migration, Table};

mod table;

use self::table::{RawCategory, RawEntry, RawUser};

#[derive(Debug)]
pub struct Registry {
    path: PathBuf,
    entries: Table<RawEntry, Entry>,
    users: Table<RawUser, User>,
    categories: Table<RawCategory, Category>,
    sms: Table<Sms, Sms>,
}

impl Registry {
    pub fn new(path: PathBuf) -> Result<Registry, FailureError> {
        debug!("creating registry at {:?}", &path);
        if !path.is_dir() {
            return Err(AppError::InvalidRegistryPath { used_path: path }.into());
        }

        let entries = table(path.clone(), "entries")?;
        let users = table(path.clone(), "users")?;
        let categories = table(path.clone(), "categories")?;
        let sms = table(path.clone(), "sms")?;

        Ok(Registry {
            path,
            entries,
            users,
            categories,
            sms,
        })
    }

    pub fn find_or_create(&self, telegram_id: TelegramId) -> Result<User, FailureError> {
        debug!("finding or creating user with {:?}", &telegram_id);
        let users: Vec<User> = self
            .users
            .select(|user| user.telegram_id == Some(telegram_id))?;
        let user = match users.into_iter().next() {
            None => {
                let new_user = User::with_telegram_id(telegram_id);
                self.users.insert(new_user.clone())?;
                new_user
            }
            Some(user) => user,
        };
        Ok(user)
    }

    pub fn find_user<F: Fn(&User) -> bool>(
        &self,
        predicate: F,
    ) -> Result<Option<User>, FailureError> {
        debug!("find user by predicate");
        let mut users: Vec<User> = self.users.select(predicate)?;
        if users.len() > 1 {
            return Err(format_err!("Found more than one user by predicate"));
        }
        Ok(users.pop())
    }

    pub fn add_entry(&self, entry: Entry) -> Result<(), FailureError> {
        debug!("adding entry {:?}", &entry);
        self.entries.insert(entry)?;
        Ok(())
    }

    pub fn list(&self, user: UserId) -> Result<Vec<Entry>, FailureError> {
        debug!("listing entries for {:?}", &user);
        let entries = self.entries.select(|e| e.user_id == user)?;
        Ok(entries)
    }

    pub fn list_users(&self) -> Result<Vec<User>, FailureError> {
        debug!("listing users");
        let users = self.users.select(|_| true)?;
        Ok(users)
    }

    pub fn update_user<F: Fn(&mut User)>(
        &self,
        user: UserId,
        update: F,
    ) -> Result<(), FailureError> {
        debug!("updating user {:?}", &user);
        self.users.update(|u| u.id == user, update)?;
        Ok(())
    }

    pub fn migrate_entries(&self, migration: Migration) -> Result<(), FailureError> {
        debug!("migrating entries with {:?}", &migration);
        self.entries.migrate(migration)?;
        Ok(())
    }

    pub fn categories(&self, user: UserId) -> Result<Vec<Category>, FailureError> {
        debug!("listing categories for {:?}", &user);
        let categories = self.categories.select(|c| c.user_id == user)?;
        Ok(categories)
    }

    pub fn add_category(
        &self,
        user: UserId,
        product_name: String,
        category_name: String,
    ) -> Result<(), FailureError> {
        debug!(
            "adding categories for {:?}: {} - {}",
            &user, &product_name, &category_name
        );
        let existing = self.categories(user.clone())?;
        if let Some(existing) = existing.iter().filter(|c| c.product == product_name).next() {
            return Err(AppError::ProductAlreadyHasCategory {
                product: existing.product.to_owned(),
                category: existing.category.to_owned(),
            }.into());
        }
        let new_category = Category::new(user, product_name, category_name);
        self.categories.insert(new_category)?;
        Ok(())
    }

    pub fn statistics(&self, user: UserId) -> Result<Statistics, FailureError> {
        debug!("getting statistics for {:?}", &user);
        let entries = self.list(user.clone())?;
        let categiries = self.categories(user.clone())?;
        Ok(Statistics::new(entries, categiries))
    }

    pub fn add_sms(&self, sms_list: Vec<NewSms>) -> Result<(), FailureError> {
        debug!("Adding {} sms", sms_list.len());
        for sms in sms_list {
            self.sms.insert(sms.into())?;
        }
        Ok(())
    }

    pub fn get_sms_list(&self, user: UserId) -> Result<Vec<Sms>, FailureError> {
        debug!("searching sms for {}", user);
        self.sms.select(|sms| sms.user == user)
    }
}

fn table<P: Serialize + DeserializeOwned + Debug + Into<R> + From<R>, R: Debug>(
    base_path: PathBuf,
    table_name: &str,
) -> Result<Table<P, R>, FailureError> {
    let table: Table<P, R> = if ::persistence::exist_with_name(&base_path, table_name) {
        Table::load(base_path, table_name)?
    } else {
        Table::create(base_path, table_name)?
    };
    Ok(table)
}
