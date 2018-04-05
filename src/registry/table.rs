use chrono::prelude::*;

use accounting::{Category, CategoryId, Entry, EntryId, Product, Tag, Tags, TelegramId, User,
                 UserId};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawEntry {
    id: String,
    user_id: String,
    product: String,
    price: i32,
    time: NaiveDateTime,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawCategory {
    id: String,
    user_id: String,
    product: String,
    category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawUser {
    id: String,
    telegram_id: Option<i64>,
    offset: Option<NaiveDateTime>,
}

impl Into<User> for RawUser {
    fn into(self) -> User {
        User {
            id: UserId::new(self.id),
            telegram_id: self.telegram_id.map(|val| TelegramId(val)),
            offset: self.offset,
        }
    }
}

impl From<User> for RawUser {
    fn from(user: User) -> RawUser {
        RawUser {
            id: user.id.0,
            telegram_id: user.telegram_id.map(|id| id.0),
            offset: user.offset,
        }
    }
}

impl Into<Category> for RawCategory {
    fn into(self) -> Category {
        Category {
            id: CategoryId::new(self.id),
            user_id: UserId::new(self.user_id),
            product: self.product,
            category: self.category,
        }
    }
}

impl From<Category> for RawCategory {
    fn from(category: Category) -> RawCategory {
        RawCategory {
            id: category.id.0,
            user_id: category.user_id.0,
            product: category.product,
            category: category.category,
        }
    }
}

impl Into<Entry> for RawEntry {
    fn into(self) -> Entry {
        Entry {
            id: EntryId::new(self.id),
            user_id: UserId::new(self.user_id),
            product: Product {
                name: self.product,
                price: self.price,
            },
            time: self.time,
            tags: Tags {
                tags: self.tags.into_iter().map(|value| Tag { value }).collect(),
            },
        }
    }
}

impl From<Entry> for RawEntry {
    fn from(entry: Entry) -> RawEntry {
        RawEntry {
            id: entry.id.0,
            user_id: entry.user_id.0,
            product: entry.product.name,
            price: entry.product.price,
            time: entry.time,
            tags: entry.tags.tags.into_iter().map(|tag| tag.value).collect(),
        }
    }
}
