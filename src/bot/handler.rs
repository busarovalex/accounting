use std::str::FromStr;

use registry::Registry;
use accounting::{Entry, Product};
use accounting::UserId;
use error::Error;
use config::Config;

pub fn handle(data: &str, config: &Config, registry: &Registry, user: UserId) -> Result<String, Error> {
    let mut words = data.split_whitespace();
    if let Some(command) = words.next() {
        match command {
            "help" | "Help" | "/help" => {
                Ok(help())
            },
            "отчет" | "/отчет" | "Отчет" => {
                super::report::report(&mut words, config, registry, user)                
            },
            query @ _ => {
                let parsed_new_product = Product::from_str(&query)?;
                let new_entry = Entry::new(user, parsed_new_product);
                registry.add_entry(new_entry)?;
                Ok(format!("Ok"))
            }
        }
    } else {
        Ok(help())
    }
}

fn help() -> String {
    format!("/help\n/отчет")
}
