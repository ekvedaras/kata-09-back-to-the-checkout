use std::collections::HashMap;
use crate::stock::Item;

mod stock;

fn main() {
    println!("Hello, world!");
}

struct Checkout {
    total: u32,
    pricing_rules: HashMap<char, Item>,
}

impl Checkout {
    pub fn scan(&mut self, item: &char) {
        // todo recalculate total instead of jsut adding
        self.total += self.pricing_rules[item].unit_price.price;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_pricing_rules() -> HashMap<char, Item> {
        Item::parse_list([
            "A|50|3 for 100",
            "B|30|2 for 45",
            "C|20||",
            "D|15||",
        ].join("\n").as_str()
        )
    }

    fn price(goods: &str) -> u32 {
        let mut checkout = Checkout {
            total: 0,
            pricing_rules: default_pricing_rules(),
        };
        goods.chars().for_each(|item| checkout.scan(&item));
        checkout.total
    }

    #[test]
    fn test_totals() {
        assert_eq!(0, price(""));
        assert_eq!(50, price("A"));
        assert_eq!(80, price("AB"));
        assert_eq!(115, price("CDBA"));

        assert_eq!(100, price("AA"));
        assert_eq!(130, price("AAA"));
        assert_eq!(180, price("AAAA"));
        assert_eq!(230, price("AAAAA"));
        assert_eq!(260, price("AAAAAA"));

        assert_eq!(160, price("AAAB"));
        assert_eq!(175, price("AAABB"));
        assert_eq!(190, price("AAABBD"));
        assert_eq!(190, price("DABABA"));
    }

    #[test]
    fn test_incremental() {
        let mut checkout = Checkout { total: 0, pricing_rules: default_pricing_rules() };
        assert_eq!(0, checkout.total);
        checkout.scan(&'A');
        assert_eq!(50, checkout.total);
        checkout.scan(&'B');
        assert_eq!(80, checkout.total);
        checkout.scan(&'A');
        assert_eq!(130, checkout.total);
        checkout.scan(&'A');
        assert_eq!(160, checkout.total);
        checkout.scan(&'B');
        assert_eq!(175, checkout.total);
    }
}