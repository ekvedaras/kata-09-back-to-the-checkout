use std::collections::HashMap;
use std::string::String;

pub struct Item {
    pub unit_price: Price,
    pub special_price: Option<Price>,
}

#[derive(Clone)]
pub struct Price {
    pub quantity: u32,
    pub price: u32,
}

impl Item {
    pub fn parse_list(definition: &str) -> Result<HashMap<char, Item>, String> {
        definition
            .split("\n")
            .map(|line| -> Result<(char, Item), String> {
                let fields: Vec<&str> = line.split("|").collect();
                if fields.len() < 2 {
                    return Err(format!("Invalid line: {}", line));
                }

                let sku = fields[0]
                    .chars()
                    .next()
                    .ok_or_else(|| format!("Invalid sku: {}", fields[0]))?;

                let unit_price = fields[1]
                    .parse::<u32>()
                    .map(|price| Price { quantity: 1, price })
                    .map_err(|_| format!("Invalid unit price: {}", fields[1]))?;

                let special_price = fields.get(2).and_then(|s| {
                    let parts: Vec<&str> = s.split(" for ").collect();
                    match parts.as_slice() {
                        [quantity, price] => Some(Price {
                            quantity: quantity.parse::<u32>().ok()?,
                            price: price.parse::<u32>().ok()?,
                        }),
                        _ => None
                    }
                });

                Ok((sku, Item { unit_price, special_price }))
            })
            .collect()
    }
}