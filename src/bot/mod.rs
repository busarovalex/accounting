use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use chrono::prelude::*;

use std::env;
use std::str::FromStr;

use registry::Registry;
use accounting::Entry;

mod representation;

use self::representation::{EntryRepresentation};

fn start(registry: &Registry) -> Result<(), String> {
    let mut core = Core::new().map_err(|e| format!("{:?}", e))?;

    let token = env::var("TELEGRAM_BOT_TOKEN").map_err(|e| format!("{:?}", e))?;
    let api = Api::configure(token).build(core.handle()).map_err(|e| format!("{:?}", e))?;

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            if message.from.id != UserId::new(221296637) {
                return Ok(());
            }
            if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.
                println!("{:?}", &message);
                println!("<{}>: {}", &message.from.first_name, data);

                
                match handle(data, &registry) {
                    Ok(msg) => api.spawn(message.text_reply(msg)),
                    Err(msg) => api.spawn(message.text_reply(msg)),
                }
                
                
            }
        }

        Ok(())
    });

    core.run(future).map_err(|e| format!("{:?}", e))?;

    Ok(())
}

fn handle(data: &str, registry: &Registry) -> Result<String, String> {
    match data {
        "list" | "/list" => {
            Ok(registry.list()?.into_iter().map(|e| format!("{}\n", EntryRepresentation::from(e))).collect())                        
        },
        query @ _ => {
            let parsed_new_entry = Entry::from_str(&query)?;
            registry.add_entry(parsed_new_entry)?;
            Ok(format!("Ok"))
        }
    }
}

pub struct BotLauncher {
    registry: Registry,
    latest_start: NaiveDateTime,
    number_of_tries: i32,
    max_number_of_tries: i32
}

impl BotLauncher {
    pub fn new(registry: Registry, max_number_of_tries: i32) -> BotLauncher {
        BotLauncher {
            registry,
            max_number_of_tries,
            number_of_tries: 0,
            latest_start: ::chrono::offset::Local::now().naive_local()
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        loop {
            let now = ::chrono::offset::Local::now().naive_local();

            if now.signed_duration_since(self.latest_start).num_seconds() > 10 {
                self.latest_start = now;
                self.number_of_tries = 0;
            }

            self.number_of_tries += 1;

            if self.number_of_tries == self.max_number_of_tries {
                return Err("number of tries exceeded".to_owned());
            }

            match start(&self.registry) {
                Ok(_) => unreachable!(),
                Err(msg) => println!("{}", msg)
            }
        }
    }
}
