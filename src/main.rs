#![recursion_limit="128"]

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
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_yaml;
extern crate lettre;
extern crate lettre_email;
extern crate mime;
extern crate base64;

use std::str::FromStr;

mod accounting;
mod app;
mod registry;
mod bot;
mod persistence;
mod error;
mod config;
mod representation;
mod dates;

use app::{App, Command, EntryCmd, MigrateCmd, UserCmd, CategoryCmd};
use registry::Registry;
use accounting::{Entry, Product, TelegramId};
use error::{Error, ErrorKind};

fn main() {
    env_logger::init();
    let app = match App::from_args() {
        Ok(app) => app,
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
            ::std::process::exit(1);
        }
    };
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
    let config_without_passwords = config::Config { email_smtp_credential_password: None, ..config.clone()};
    info!("config: {:?}", &config_without_passwords);
    let registry = Registry::new(config.data_path.clone().into())?;
    info!("registry created");
    match app.command {
        Command::Entry(EntryCmd::List) => {
            let user = registry.find_or_create(TelegramId(config.telegram_user_id))?;
            for entry in registry.list(user.id)? {
                println!("{}", representation::EntryRepresentation::from(entry));
            }
        },
        Command::Entry(EntryCmd::Add(new_entry)) => {
            let user = registry.find_or_create(TelegramId(config.telegram_user_id))?;
            let parsed_new_product = Product::from_str(&new_entry)?;
            let new_entry = Entry::new(user.id, parsed_new_product);
            registry.add_entry(new_entry)?;
        },
        Command::Bot => {
            bot::BotLauncher::new(registry, config).start()?;
        },
        Command::Migrate(MigrateCmd::Add(field_name, value)) => {
            registry.migrate_entries(::persistence::Migration::add_from_str(field_name, &value)?)?;
        },
        Command::Migrate(MigrateCmd::Remove(field_name)) => {
            registry.migrate_entries(::persistence::Migration::remove(field_name))?;
        },
        Command::Migrate(MigrateCmd::GenerateUid(field_name)) => {
            registry.migrate_entries(::persistence::Migration::generate_uid(field_name))?;
        },
        Command::User(UserCmd::List) => {
            for user in registry.list_users()? {
                println!("{:?}", user);
            }
        },
        Command::User(UserCmd::Add(user_telegram_id)) => {
            let user = registry.find_or_create(TelegramId(user_telegram_id))?;
            println!("{:?}", user);
        },
        Command::User(UserCmd::SetTimezone(offset_in_minutes)) => {
            let user = registry.find_or_create(TelegramId(config.telegram_user_id))?;
            let offset = chrono::NaiveDateTime::from_timestamp(offset_in_minutes as i64 * 60, 0);
            registry.update_user(user.id, |ref mut u| u.offset = Some(offset))?;
        },
        Command::Category(CategoryCmd::List) => {
            let user = registry.find_or_create(TelegramId(config.telegram_user_id))?;
            for category in registry.categories(user.id)? {
                println!("{:?}", category);
            }
        },
        Command::Category(CategoryCmd::Add(product_name, category_name)) => {
            let user = registry.find_or_create(TelegramId(config.telegram_user_id))?;
            registry.add_category(user.id, product_name, category_name)?;
        },
        Command::Report(time_period, html) => {
            let user = registry.find_or_create(TelegramId(config.telegram_user_id))?;
            let stats = registry.statistics(user.id)?;
            let err: Error = ErrorKind::NoDataForPeriod.into();
            let report = stats.report(time_period)?.ok_or(err)?;
            if html {
                println!("{}", representation::ReactReportRepresentation::from(report));
            } else {
                println!("{}", representation::ReportRepresentation::from(report));
            }
        }
    }

    Ok(())
}
