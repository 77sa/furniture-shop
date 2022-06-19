use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Article {
    pub id: usize,
    pub name: String,
    pub stock: usize,
}

pub type Inventory = Arc<Mutex<Vec<Article>>>;

pub fn load_inventory(file: &str) -> Inventory {
    let inventory_string = fs::read_to_string(file).unwrap();
    let inventory: Vec<Article> =
        serde_json::from_str(&inventory_string).unwrap();

    Arc::new(Mutex::new(inventory))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn it_should_parse_json_inventory() {
        let inventory = load_inventory("test.json");
        let inventory = inventory.lock().unwrap().clone();

        let inventory_test = Arc::new(Mutex::new(vec![
            Article {
                id: 0,
                name: "Wood".to_owned(),
                stock: 10,
            },
            Article {
                id: 1,
                name: "Metal".to_owned(),
                stock: 12,
            },
        ]));
        let test_inventory = inventory_test.lock().unwrap().clone();

        assert_eq!(inventory, test_inventory);
    }
}
