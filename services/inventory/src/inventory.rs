use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub id: usize,
    pub name: String,
    pub stock: usize,
}

pub type Inventory = Arc<Mutex<Vec<Article>>>;

pub fn load_inventory() -> Inventory {
    let inventory_string = fs::read_to_string("inventory.json").unwrap();
    let inventory: Vec<Article> =
        serde_json::from_str(&inventory_string).unwrap();

    Arc::new(Mutex::new(inventory))
}
