// La structure `Package` est définie avec des champs de données et des méthodes associées.
#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    // La méthode `new` crée une nouvelle instance de `Package`.
    // Si le poids est inférieur à 10, elle panique avec un message d'erreur.
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            panic!("Cannot ship a package with weight below 10 grams.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    // La méthode `is_international` vérifie si l'envoi est international.
    fn is_international(&self) -> bool { // J'ai modifié ??? à bool pour savoir si le colis est international ou non.
        self.sender_country != self.recipient_country
    }

    // La méthode `get_fees` calcule les frais de transport en fonction du poids du colis.
    fn get_fees(&self, cents_per_gram: u32) -> u32 { // J'ai modifié le type de retour de ??? à u32 pour renvoyer le montant des frais de transport calculés en fonction du poids du colis.
        self.weight_in_grams * cents_per_gram
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

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
