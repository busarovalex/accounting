use chrono::prelude::*;
use futures::Stream;
use telegram_bot::*;
use tokio_core::reactor::Core;

use std::env;

use self::app::App;
use accounting::TelegramId;
use config::Config;
use error::Error as AppError;
use error::ErrorKind;
use registry::Registry;

mod app;
mod category;
mod email;
mod handler;
mod report;

pub fn start() {
    let app = match App::from_args() {
        Ok(app) => app,
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
            ::std::process::exit(1);
        }
    };
    info!("{:?}", &app);

    match start_bot(app) {
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
            ::std::process::exit(1);
        }
        Ok(_) => {}
    };
}

fn start_bot(app: App) -> Result<(), AppError> {
    let config = crate::config::config(&app.config_path)?;
    let config_without_passwords = crate::config::Config {
        email_smtp_credential_password: None,
        ..config.clone()
    };
    info!("config: {:?}", &config_without_passwords);
    let registry = Registry::new(config.data_path.clone().into())?;
    info!("registry created");
    BotLauncher::new(registry, config).start()
}

pub struct BotLauncher {
    registry: Registry,
    config: Config,
    latest_start: NaiveDateTime,
    number_of_tries: i32,
    max_number_of_tries: i32,
    allowed_telegram_users: Vec<i64>,
}

impl BotLauncher {
    pub fn new(registry: Registry, config: Config) -> BotLauncher {
        BotLauncher {
            registry,
            max_number_of_tries: config.max_number_of_tries,
            number_of_tries: 0,
            allowed_telegram_users: config.allowed_telegram_users.clone(),
            latest_start: ::chrono::offset::Local::now().naive_local(),
            config,
        }
    }

    pub fn start(&mut self) -> Result<(), AppError> {
        loop {
            let now = ::chrono::offset::Local::now().naive_local();

            if now.signed_duration_since(self.latest_start).num_seconds() > 10 {
                self.latest_start = now;
                self.number_of_tries = 0;
            }

            self.number_of_tries += 1;

            if self.number_of_tries == self.max_number_of_tries {
                return Err(ErrorKind::NumberOfLauchesExeeded.into());
            }

            match self.run() {
                Ok(_) => unreachable!(),
                Err(msg) => warn!("{}", msg),
            }

            ::std::thread::sleep(::std::time::Duration::new(5, 0));
        }
    }

    fn run(&self) -> Result<(), AppError> {
        let mut core = Core::new().map_err(|e| format!("{:?}", e))?;

        let token = env::var("TELEGRAM_BOT_TOKEN").map_err(|e| format!("{:?}", e))?;
        let api = Api::configure(token)
            .build(core.handle())
            .map_err(|e| format!("{:?}", e))?;

        let future = api.stream().for_each(|update| {
            if let UpdateKind::Message(message) = update.kind {
                let author_id: i64 = message.from.id.into();

                if !self.allowed_telegram_users.contains(&author_id) {
                    return Ok(());
                }
                if let MessageKind::Text { ref data, .. } = message.kind {
                    trace!("{:?}", &message);
                    trace!("<{}>: {}", &message.from.first_name, data);

                    let user = self
                        .registry
                        .find_or_create(TelegramId(i64::from(message.from.id)))
                        .map_err(|e| format!("{:?}", e))?;

                    match self::handler::handle(data, &self.config, &self.registry, user.id) {
                        Ok(msg) => api.spawn(message.text_reply(msg)),
                        Err(msg) => {
                            warn!("{}", msg);
                            api.spawn(message.text_reply(format!("Error: {}", msg)))
                        }
                    }
                }
            }

            Ok(())
        });

        core.run(future).map_err(|e| format!("{:?}", e))?;

        Ok(())
    }
}
