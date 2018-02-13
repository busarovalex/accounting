extern crate uuid;
extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bincode;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_yaml;

use structopt::StructOpt;

use std::str::FromStr;

mod accounting;
mod app;
mod registry;
mod bot;
mod persistence;
mod error;
mod config;

use app::{App, Command};
use registry::Registry;
use accounting::{Entry, Product, TelegramId};
use error::{Error, ErrorKind};
// use config::Config;

fn main() {
    env_logger::init();
    let app = App::from_args();
    info!("{:?}", &app);

    match start(app) {
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
            ::std::process::exit(1);
        },
        Ok(_) => {}
    };    
}

fn start(app: App) -> Result<(), Error> {
    let config = config::config(&app)?;
    info!("config: {:?}", &config);
    let registry = Registry::new(config.data_path.clone().into())?;

    match app.command {
        Command::Add{ input } => {
            let user = registry.find_or_create(TelegramId(config.telegram_user_id))?;
            let new_entry: String = input.into_iter().collect();
            let parsed_new_product = Product::from_str(&new_entry)?;
            let new_entry = Entry::new(user.id, parsed_new_product);
            registry.add_entry(new_entry)?;
        },
        Command::List{} => {
            for entry in registry.list()? {
                println!("{}", accounting::representation::EntryRepresentation::from(entry));
            }
        },
        Command::Bot{} => {
            bot::BotLauncher::new(registry, config).start()?;
        },
        Command::Migrate{ add, remove } => {
            match (add.is_empty(), remove) {
                (true, None) | (false, Some(_)) => {
                    return Err(ErrorKind::IncorrectApplicationUse("не указана миграция".into()).into());
                },
                (true, Some(field_name)) => {
                    registry.migrate_entries(::persistence::Migration::remove(field_name))?;
                },
                (false, None) => {
                    let mut arg_iter = add.into_iter();
                    if let (Some(field_name), Some(value)) = (arg_iter.next(), arg_iter.next()) {
                        if arg_iter.next().is_some() {
                            return Err(ErrorKind::IncorrectApplicationUse("должно быть два операнда".into()).into());
                        }
                        registry.migrate_entries(::persistence::Migration::add_from_str(field_name, &value)?)?;
                    }
                }
            }
        },
        Command::User{list, telegram_id} => {
            match (list, telegram_id) {
                (false, Some(user_telegram_id)) => {
                    let user = registry.find_or_create(TelegramId(user_telegram_id))?;
                    println!("{:?}", user);
                },
                (true, None) => {
                    for user in registry.list_users()? {
                        println!("{:?}", user);
                    }
                },
                _ => return Err(ErrorKind::IncorrectApplicationUse("Неверное использование".into()).into())
            }
        }
    }

    Ok(())
}
