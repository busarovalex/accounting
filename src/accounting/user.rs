use chrono::naive::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserId(pub String);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TelegramId(pub i64);

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub telegram_id: Option<TelegramId>,
    pub offset: Option<NaiveDateTime>
}

impl User {
    pub fn with_telegram_id(telegram_id: TelegramId) -> User {
        User {
            id: UserId::generate(),
            telegram_id: Some(telegram_id),
            offset: None
        }
    }
}

impl UserId {
    pub fn new(value: String) -> UserId {
        UserId(value)
    }

    fn generate() -> UserId {
        UserId(format!("{}", Uuid::new_v4()))
    }
}
