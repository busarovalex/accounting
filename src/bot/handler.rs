use std::str::FromStr;

use registry::Registry;
use accounting::{Entry, Product};
use accounting::UserId;
use error::Error;
use config::Config;

pub fn handle(
    data: &str,
    config: &Config,
    registry: &Registry,
    user: UserId,
) -> Result<String, Error> {
    let mut words = data.split_whitespace();
    if let Some(command) = words.next() {
        match command {
            "help" | "Help" | "/help" => Ok(help()),
            "отчет" | "/отчет" | "Отчет" => {
                super::report::report(&mut words, config, registry, user)
            }
            "Кат" | "кат" => super::category::category(&mut words, registry, user),
            _ => {
                let parsed_new_product = Product::from_str(&data)?;
                let new_entry = Entry::new(user, parsed_new_product);
                let response = added_entry(registry, &new_entry)?;
                registry.add_entry(new_entry)?;
                Ok(response)
            }
        }
    } else {
        Ok(help())
    }
}

fn help() -> String {
    format!("/help\n/отчет")
}

fn added_entry(registry: &Registry, entry: &Entry) -> Result<String, Error> {
    let categories = registry.categories(entry.user_id.clone())?;
    if let Some(category) = categories.iter().find(|c| c.product == entry.product.name) {
        Ok(format!(
            "{} ({}) - {} руб.",
            entry.product.name, category.category, entry.product.price
        ))
    } else {
        Ok(format!(
            "{} - {} руб.",
            entry.product.name, entry.product.price
        ))
    }
}
