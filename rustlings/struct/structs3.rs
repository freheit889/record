
#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
           return Package{recipient_country,sender_country,-weight_in_grams};
        } else {
            return Package {sender_country, recipient_country, weight_in_grams};
        }
    }

    fn is_international(&self) -> blue {
        // Something goes here...
        true
    }

    fn get_fees(&self, cents_per_kg: i32) -> i32 {
        // Something goes here... (beware of grams to kg conversion)
        1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");
        
        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_kg = 2000;
        
        let package = Package::new(sender_country, recipient_country, 1500);
        
        assert_eq!(package.get_fees(cents_per_kg), 4500);
    }
}
