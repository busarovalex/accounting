use super::{CategoryId, UserId};

#[derive(Debug)]
pub struct Category {
    pub id: CategoryId,
    pub user_id: UserId,
    pub product: String,
    pub category: String,
}

impl Category {
    pub fn new(user_id: UserId, product: String, category: String) -> Category {
        Category {
            id: CategoryId::generate(),
            user_id,
            product,
            category,
        }
    }
}
