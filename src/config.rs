use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub data_path: String,
    pub telegram_user_id: i64,
    pub allowed_telegram_users: Vec<i64>,
    pub max_number_of_tries: i32,
    pub email_from: Option<String>,
    pub email_smtp_host: Option<String>,
    pub email_smtp_credential_username: Option<String>,
    pub email_smtp_credential_password: Option<String>,
}

pub fn config(config_path: &Option<PathBuf>) -> Result<Config, Error> {
    let config_path: PathBuf = config_path.clone().unwrap_or("./config.yml".into());
    let mut file = File::open(config_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: Config = ::serde_yaml::from_str(&content)?;
    Ok(config)
}
