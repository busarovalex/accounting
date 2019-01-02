use actix_web::{FromRequest, HttpRequest};
use failure::Error as FailureError;

use super::AppState;

#[derive(Debug, Clone)]
pub struct AndroidAuth {
    pub token: Option<String>,
}

const ANDROID_AUTH_TOKEN_HEADER: &'static str = "android-token";

impl FromRequest<AppState> for AndroidAuth {
    type Config = ();
    type Result = Result<AndroidAuth, FailureError>;
    fn from_request(req: &HttpRequest<AppState>, _cfg: &Self::Config) -> Self::Result {
        let token = if let Some(token_header) = req.headers().get(ANDROID_AUTH_TOKEN_HEADER) {
            Some(token_header.to_str()?.to_string())
        } else {
            None
        };

        Ok(AndroidAuth { token })
    }
}
