use std::collections::HashMap;

pub struct Item {
    pub sku: char,
    pub unit_price: Price,
    special_price: Option<Price>
}

pub struct Price {
    quantity: u32,
    pub price: u32,
}

fn unit_price(of: u32) -> Price {
    Price { quantity: 1, price: of }
}

impl Item {
    pub fn parse_list(definition: &str) -> HashMap<char, Item> {
        definition
            .split("\n")
            .map(|line| {
                // todo: get rid of unwrap (change return type)
                let fields: Vec<&str> = line.split("|").collect();
                let sku = fields[0].chars().nth(0).unwrap();

                (sku, Item {
                    sku,
                    unit_price: unit_price(fields[1].parse::<u32>().unwrap()),
                    special_price: {
                        let parts : Vec<&str> = fields[2].split(" for ").collect();

                        match parts.len() {
                            2 => Some(Price {
                                quantity: parts[0].parse::<u32>().unwrap(),
                                price: parts[1].parse::<u32>().unwrap()
                            }),
                            _ => None,
                        }
                    }
                })
            })
            .collect()
    }
}