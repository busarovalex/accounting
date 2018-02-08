use chrono::prelude::*;

use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub product: String,
    pub price: i32,
    pub time: NaiveDateTime
}

impl FromStr for Entry {
    type Err = String;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut iter_whitespace = raw.split_whitespace();

        let parsed = match (iter_whitespace.next(), iter_whitespace.next()) {
            (Some(value_left), Some(value_right)) => {
                match(i32::from_str(value_left), i32::from_str(value_right)) {
                    (Ok(price), Err(_)) => {
                        Entry {
                            product: value_right.to_owned(),
                            price,
                            time: ::chrono::offset::Local::now().naive_local()
                        }
                    },
                    (Err(_), Ok(price)) => {
                        Entry {
                            product: value_left.to_owned(),
                            price,
                            time: ::chrono::offset::Local::now().naive_local()
                        }
                    },
                    _ => return Err("В строке должны быть указаны продукт и цена".to_owned())
                }
            }
            _ => return Err("В строке должны быть указаны продукт и цена".to_owned())
        };

        if iter_whitespace.next().is_some() {
            return Err("В строке должны быть указаны только продукт и цена".to_owned());
        }

        Ok(parsed)
    }
}
