use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub requires: Vec<usize>,
}

pub type Products = Arc<Vec<Product>>;

pub fn load_products() -> Products {
    let products_string = fs::read_to_string("products.json").unwrap();
    let products: Vec<Product> = serde_json::from_str(&products_string).unwrap();
    Arc::new(products)
}
