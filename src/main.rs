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

use structopt::StructOpt;

use std::str::FromStr;

mod accounting;
mod app;
mod registry;
mod bot;
mod persistence;
mod error;

use app::{App, Command};
use registry::Registry;
use accounting::{Entry};
use error::{Error, ErrorKind};

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
    let registry = Registry::new(app.data)?;

    match app.command {
        Command::Add{ input } => {
            let new_entry: String = input.into_iter().collect();
            let parsed_new_entry = Entry::from_str(&new_entry)?;
            registry.add_entry(parsed_new_entry)?;
        },
        Command::List{} => {
            for entry in registry.list()? {
                println!("{}", accounting::representation::EntryRepresentation::from(entry));
            }
        },
        Command::Bot{} => {
            bot::BotLauncher::new(registry, 5).start()?;
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
        }
    }

    Ok(())
}

