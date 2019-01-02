use chrono::naive::NaiveDateTime;

use super::{SmsId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sms {
    pub id: SmsId,
    pub user: UserId,
    pub from: String,
    pub text: String,
    pub time: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSms {
    pub user: UserId,
    pub from: String,
    pub text: String,
    pub time: NaiveDateTime,
}

impl Into<Sms> for NewSms {
    fn into(self) -> Sms {
        Sms {
            id: SmsId::generate(),
            user: self.user,
            from: self.from,
            text: self.text,
            time: self.time,
        }
    }
}
