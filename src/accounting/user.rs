use chrono::naive::NaiveDateTime;

use super::UserId;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TelegramId(pub i64);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AndroidAuth(pub String);

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub telegram_id: Option<TelegramId>,
    pub android_auth_token: Option<AndroidAuth>,
    pub offset: Option<NaiveDateTime>,
}

impl User {
    pub fn with_telegram_id(telegram_id: TelegramId) -> User {
        User {
            id: UserId::generate(),
            telegram_id: Some(telegram_id),
            offset: None,
            android_auth_token: None,
        }
    }
}
