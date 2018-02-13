use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use chrono::prelude::*;

use std::env;
use std::str::FromStr;

use registry::Registry;
use accounting::{Entry, Product, TelegramId};
use accounting::UserId as AccountingUserId;
use error::Error as AppError;
use error::ErrorKind;
use config::Config;

use accounting::representation::{EntryRepresentation};

pub struct BotLauncher {
    registry: Registry,
    latest_start: NaiveDateTime,
    number_of_tries: i32,
    max_number_of_tries: i32,
    allowed_telegram_users: Vec<i64>
}

impl BotLauncher {
    pub fn new(registry: Registry, config: Config,) -> BotLauncher {
        BotLauncher {
            registry,
            max_number_of_tries: config.max_number_of_tries,
            number_of_tries: 0,
            allowed_telegram_users: config.allowed_telegram_users,
            latest_start: ::chrono::offset::Local::now().naive_local()
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
                Err(msg) => warn!("{}", msg)
            }

            ::std::thread::sleep(::std::time::Duration::new(5, 0));
        }
    }

    fn run(&self) -> Result<(), AppError> {
        let mut core = Core::new().map_err(|e| format!("{:?}", e))?;

        let token = env::var("TELEGRAM_BOT_TOKEN").map_err(|e| format!("{:?}", e))?;
        let api = Api::configure(token).build(core.handle()).map_err(|e| format!("{:?}", e))?;
        
        let future = api.stream().for_each(|update| {

            if let UpdateKind::Message(message) = update.kind {
                let author_id: i64 = message.from.id.into();
                
                if !self.allowed_telegram_users.contains(&author_id) {
                    return Ok(());
                }
                if let MessageKind::Text {ref data, ..} = message.kind {
                    trace!("{:?}", &message);
                    trace!("<{}>: {}", &message.from.first_name, data);

                    let user = self.registry.find_or_create(TelegramId(i64::from(message.from.id))).map_err(|e| format!("{:?}", e))?;
                    
                    match handle(data, &self.registry, user.id) {
                        Ok(msg) => api.spawn(message.text_reply(msg)),
                        Err(msg) => api.spawn(message.text_reply(format!("Error: {}", msg))),
                    }                
                }
            }

            Ok(())
        });

        core.run(future).map_err(|e| format!("{:?}", e))?;

        Ok(())
    }
}

fn handle(data: &str, registry: &Registry, user: AccountingUserId) -> Result<String, AppError> {
    match data {
        "help" | "Help" | "/help" => {
            Ok(format!("/list"))
        },
        "List" | "list" | "/list" => {
            Ok(registry.list()?.into_iter().map(|e| format!("{}\n", EntryRepresentation::from(e))).collect())                        
        },
        query @ _ => {
            let parsed_new_product = Product::from_str(&query)?;
            let new_entry = Entry::new(user, parsed_new_product);
            registry.add_entry(new_entry)?;
            Ok(format!("Ok"))
        }
    }
}
