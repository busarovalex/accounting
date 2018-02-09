use chrono::prelude::*;

use std::str::FromStr;

use super::evaluation::evaluate;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub product: String,
    pub price: i32,
    pub time: NaiveDateTime
}

impl FromStr for Entry {
    type Err = String;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (price, product) = price_product(raw)?;

        Ok(Entry{
            product: product.to_owned(),
            price: evaluate(price).map_err(|e| format!("{:?}", e))?,
            time: ::chrono::offset::Local::now().naive_local()
        })
    }
}

fn price_product(raw: &str) -> Result<(&str, &str), String> {
    let raw = raw.trim();
    let mut price_first = None;
    let mut split_index = 0;
    for (index, ch) in raw.char_indices() {
        let price_part = match ch {
            '(' | ')' | ' '| '*' | '-' | '+' | '/' | '0'...'9' => true,
            _ => false
        };
        match (price_part, price_first) {
            (true, None) => price_first = Some(true),
            (true, Some(false)) => {
                split_index = index;
                break;
            },
            (true, Some(true)) => {},
            (false, None) => price_first = Some(false),
            (false, Some(false)) => {},
            (false, Some(true)) => {
                split_index = index;
                break;
            }
        }
    }

    let (price, product) = match price_first {
        None => return Err("В строке должны быть указаны продукт и цена".to_owned()),
        Some(true) => raw.split_at(split_index),
        Some(false) => { let (product, price) = raw.split_at(split_index); (price, product)}
    };

    if product.is_empty() {
        return Err("В строке должны быть указаны продукт и цена".to_owned());
    }

    Ok((price, product))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_splits() {
        assert_eq!(price_product("чай 75"), Ok((" 75", "чай")));
    }
}
