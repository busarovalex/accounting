use uuid::Uuid;

use super::{UserId};

#[derive(Debug)]
pub struct CategoryId(pub String);

#[derive(Debug)]
pub struct Category {
    pub id: CategoryId,
    pub user_id: UserId,
    pub product: String,
    pub category: String
}

impl Category {
    pub fn new(user_id: UserId, product: String, category: String) -> Category {
        Category {
            id: CategoryId::generate(),
            user_id,
            product,
            category
        }
    }
}

impl CategoryId {
    pub fn new(value: String) -> Self {
        CategoryId(value)
    }

    fn generate() -> Self {
        CategoryId(format!("{}", Uuid::new_v4()))
    }
}
