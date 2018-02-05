use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

use std::env;
use std::str::FromStr;

use registry::Registry;
use accounting::Entry;

pub fn start(registry: Registry) -> Result<(), String> {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

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

    core.run(future).unwrap();

    Ok(())
}

fn handle(data: &str, registry: &Registry) -> Result<String, String> {
    match data {
        "list" | "/list" => {
            Ok(registry.list()?.into_iter().map(|e| format!("{:?}\n", e)).collect())                        
        },
        query @ _ => {
            let parsed_new_entry = Entry::from_str(&query)?;
            registry.add_entry(parsed_new_entry)?;
            Ok(format!("Ok"))
        }
    }
}
