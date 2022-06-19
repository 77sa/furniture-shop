use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub requires: Vec<usize>,
}

pub type Products = Arc<Vec<Product>>;

pub fn load_products(file: &str) -> Products {
    let products_string = fs::read_to_string(file).unwrap();
    let products: Vec<Product> = serde_json::from_str(&products_string).unwrap();
    Arc::new(products)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_parse_json_products() {
        let products = load_products("test.json");

        let test_products = Arc::new(vec![
            Product {
                id: 0,
                name: "Bed".to_owned(),
                requires: vec![0, 1, 4, 5]
            },
            Product {
                id: 1,
                name: "Sofa".to_owned(),
                requires: vec![0, 1, 2, 4]
            }
        ]);

        assert_eq!(*products, *test_products);
    }
}
