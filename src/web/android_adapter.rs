use actix_web::{Json, State};
use chrono::naive::NaiveDateTime;
use failure::Error as FailureError;

use super::auth::AndroidAuth;
use super::AppState;
use accounting::NewSms;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sms {
    pub time: String,
    pub from: String,
    pub text: String,
}

pub fn post_sms(
    state: State<AppState>,
    smses: Json<Vec<Sms>>,
    auth: AndroidAuth,
) -> Result<String, FailureError> {
    let registry = state
        .registry
        .lock()
        .map_err(|e| format_err!("Failed to get registry: {}", e))?;
    let request_auth_token = auth
        .token
        .ok_or(format_err!("Auth token is not provided"))?;
    let user = registry
        .find_user(|u| match u.android_auth_token {
            Some(ref user_auth_token) => user_auth_token.0 == request_auth_token,
            None => false,
        })?.ok_or(format_err!("User with auth token not found"))?;
    let smses = smses
        .into_inner();
    debug!("Received {} sms for user {}", smses.len(), user.id);
    let new_sms_list: Result<Vec<NewSms>, FailureError> = smses
        .into_iter()
        .map(|sms| {
            let timestamp_millis: i64 = sms.time.parse()?;
            let time = NaiveDateTime::from_timestamp_opt(timestamp_millis / 1000, 0)
                .ok_or(format_err!("NaiveDateTime::from_timestamp_opt failed"))?;
            Ok(NewSms {
                user: user.id.clone(),
                from: sms.from,
                text: sms.text,
                time,
            })
        }).collect();
    registry.add_sms(new_sms_list?)?;
    Ok("Ok".to_string())
}

pub fn get_sms_latest(state: State<AppState>, auth: AndroidAuth) -> Result<String, FailureError> {
    let registry = state
        .registry
        .lock()
        .map_err(|e| format_err!("Failed to get registry: {}", e))?;
    let request_auth_token = auth
        .token
        .ok_or(format_err!("Auth token is not provided"))?;
    let user = registry
        .find_user(|u| match u.android_auth_token {
            Some(ref user_auth_token) => user_auth_token.0 == request_auth_token,
            None => false,
        })?.ok_or(format_err!("User with auth token not found"))?;
    let sms_list = registry.get_sms_list(user.id)?;
    let latest_sms_date = sms_list
        .into_iter()
        .map(|sms| sms.time)
        .max()
        .map(|time| format!("{}", time.timestamp_millis()))
        .unwrap_or("0".to_string());
    debug!("latest_sms_date = {}", latest_sms_date);
    Ok(latest_sms_date)
}
