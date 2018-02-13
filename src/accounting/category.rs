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

impl CategoryId {
    pub fn new(value: String) -> Self {
        CategoryId(value)
    }
}
