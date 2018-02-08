extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

use std::str::FromStr;

mod accounting;
mod app;
mod registry;
mod bot;

use app::App;
use registry::Registry;
use accounting::{Entry};

fn main() {
    let app = App::from_args();

    match start(app) {
        Err(err) => {
            println!("{}", err);
            ::std::process::exit(1);
        },
        Ok(_) => {}
    };    
}

fn start(app: App) -> Result<(), String> {
    let registry = Registry::new(app.data)?;

    if app.list {
        for entry in registry.list()? {
            println!("{}", accounting::representation::EntryRepresentation::from(entry));
        }
        return Ok(());
    }

    if let Some(new_entry) = app.entry {
        let parsed_new_entry = Entry::from_str(&new_entry)?;
        registry.add_entry(parsed_new_entry)?;
        return Ok(());
    }

    if app.bot {
        bot::BotLauncher::new(registry, 5).start()?;
    }

    Ok(())
}

