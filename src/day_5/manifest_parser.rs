use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub package: Package,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub authors: Vec<String>,
    pub keywords: Vec<String>,
    pub metadata: Option<Metadata>, // Metadata is optional because it may not always be present
}

#[derive(Default, Deserialize, Debug)]
pub struct Metadata {
    #[serde(default)]
    pub orders: Vec<Order>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Order {
    pub item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    pub quantity: Option<u32>,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.quantity {
            Some(num) => write!(f, "{}: {:?}", self.item, num),
            None => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5::manifest_parser::Manifest;

    #[test]
    fn parse_toml_ok() {
        let toml_data = r#"
            [package]
            name = "not-a-gift-order"
            authors = ["Not Santa"]
            keywords = ["Christmas 2024"]

            [[package.metadata.orders]]
            item = "Toy car"
            quantity = 2

            [[package.metadata.orders]]
            item = "Lego brick"
            quantity = 230
        "#;

        let parsed: Result<Manifest, toml::de::Error> = toml::from_str(toml_data);

        assert!(parsed.is_ok(), "Failed to parse TOML");

        let manifest = parsed.unwrap();

        assert_eq!(manifest.package.name, "not-a-gift-order");
        assert_eq!(manifest.package.authors, vec!["Not Santa"]);
        assert_eq!(manifest.package.keywords, vec!["Christmas 2024"]);

        if let Some(metadata) = manifest.package.metadata {
            assert_eq!(metadata.orders.len(), 2);

            assert_eq!(metadata.orders[0].item, "Toy car");
            assert_eq!(metadata.orders[0].quantity, Some(2));

            assert_eq!(metadata.orders[1].item, "Lego brick");
            assert_eq!(metadata.orders[1].quantity, Some(230));
        } else {
            panic!("Metadata should be present");
        }
    }

    #[test]
    fn parse_toml_invalid_order() {
        let toml_data = r#"
        [package]
        name = "not-a-gift-order"
        authors = ["Not Santa"]
        keywords = ["Christmas 2024"]

        [[package.metadata.orders]]
        item = "Toy car"
        quantity = 2

        [[package.metadata.orders]]
        item = "Lego brick"
        quantity = 1.5

        [[package.metadata.orders]]
        item = "Doll"
        quantity = 2

        [[package.metadata.orders]]
        quantity = 5
        item = "Cookie:::\n"

        [[package.metadata.orders]]
        item = "Thing"
        count = 3
        "#;

        let parsed: Result<Manifest, toml::de::Error> = toml::from_str(toml_data);

        assert!(parsed.is_ok(), "Failed to parse TOML");

        let manifest = parsed.unwrap();

        assert_eq!(manifest.package.name, "not-a-gift-order");
        assert_eq!(manifest.package.authors, vec!["Not Santa"]);
        assert_eq!(manifest.package.keywords, vec!["Christmas 2024"]);

        if let Some(metadata) = manifest.package.metadata {
            assert_eq!(metadata.orders.len(), 5);

            assert_eq!(metadata.orders[0].item, "Toy car");
            assert_eq!(metadata.orders[0].quantity, Some(2));

            assert_eq!(metadata.orders[1].item, "Lego brick");
            assert_eq!(metadata.orders[1].quantity, None);

            assert_eq!(metadata.orders[2].item, "Doll");
            assert_eq!(metadata.orders[2].quantity, Some(2));

            assert_eq!(metadata.orders[3].item, "Cookie:::\n");
            assert_eq!(metadata.orders[3].quantity, Some(5));

            assert_eq!(metadata.orders[4].item, "Thing");
            assert_eq!(metadata.orders[4].quantity, None);
        } else {
            panic!("Metadata should be present");
        }
    }
}
