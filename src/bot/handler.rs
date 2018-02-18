use std::str::FromStr;

use registry::Registry;
use accounting::{Entry, Product};
use accounting::UserId as AccountingUserId;
use error::Error as AppError;

use representation::{EntryRepresentation};

pub fn handle(data: &str, registry: &Registry, user: AccountingUserId) -> Result<String, AppError> {
    match data {
        "help" | "Help" | "/help" => {
            Ok(format!("/list"))
        },
        "List" | "list" | "/list" => {
            Ok(registry.list(user)?.into_iter().map(|e| format!("{}\n", EntryRepresentation::from(e))).collect())                        
        },
        query @ _ => {
            let parsed_new_product = Product::from_str(&query)?;
            let new_entry = Entry::new(user, parsed_new_product);
            registry.add_entry(new_entry)?;
            Ok(format!("Ok"))
        }
    }
}
