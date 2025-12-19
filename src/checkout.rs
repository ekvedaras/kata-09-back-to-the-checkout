use std::collections::HashMap;
use crate::stock::{Item};

struct Checkout {
    total: u32,
    pricing_rules: HashMap<char, Item>,
    items: Vec<char>,
}

impl Checkout {
    pub fn scan(&mut self, item: &char) {
        self.items.push(*item);
        self.recalculate_total();
    }

    fn recalculate_total(&mut self) {
        self.total = 0;
        let unique = self.items.iter().collect::<std::collections::HashSet<_>>();
        let mut counts: HashMap<char, u32> = HashMap::new();
        for item in unique {
            counts.insert(*item, self.items.iter().filter(|i| **i == *item).count() as u32);
        }

        for (item, count) in counts {
            let unit_price = self.pricing_rules[&item].unit_price.price;
            let special_price = &self.pricing_rules[&item].special_price;

            let mut quantity = count;
            let mut price = 0;
            if let Some(special_price) = special_price {
                while quantity >= special_price.quantity {
                    quantity -= special_price.quantity;
                    price += special_price.price;
                }
            }
            self.total += unit_price * quantity + price;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::checkout::Checkout;
    use crate::stock::Item;

    fn default_pricing_rules() -> HashMap<char, Item> {
        Item::parse_list([
            "A|50|3 for 130",
            "B|30|2 for 45",
            "C|20||",
            "D|15||",
        ].join("\n").as_str()).unwrap()
    }

    fn price(goods: &str) -> u32 {
        let mut checkout = Checkout {
            total: 0,
            pricing_rules: default_pricing_rules(),
            items: vec![]
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
        let mut checkout = Checkout { total: 0, pricing_rules: default_pricing_rules(), items: vec![] };
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