use crate::products::Products;
use crate::rpc::InventoryClient;
use serde::{Deserialize, Serialize};
use tarpc::context;

#[derive(Serialize)]
pub struct Response {
    pub id: usize,
    pub name: String,
    pub stock: usize,
}

#[derive(Deserialize)]
pub struct Buy {
    id: usize,
}

pub async fn get(
    products: Products,
    inventory: InventoryClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    let inventory = inventory.get(context::current(), None).await.unwrap();
    let mut result: Vec<Response> = Vec::new();

    for product in &*products {
        let requires = &product.requires;
        let mut stock_vec: Vec<usize> = Vec::new();

        for article in &inventory {
            if requires.contains(&article.id) {
                stock_vec.push(article.stock)
            }
        }

        let mut stock = 0;
        while !stock_vec.contains(&0) {
            for i in &mut stock_vec {
                *i -= 1
            }
            stock += 1;
        }

        let response = Response {
            id: product.id,
            name: product.name.clone(),
            stock,
        };
        result.push(response)
    }

    Ok(warp::reply::json(&result))
}

pub async fn buy(
    products: Products,
    inventory: InventoryClient,
    json: Buy,
) -> Result<impl warp::Reply, warp::Rejection> {
    if json.id >= products.len() {
        return Ok(warp::reply::json(&"Product not found".to_owned()));
    }
    
    let requires = products[json.id].requires.clone();
    let result = inventory.buy(context::current(), requires).await.unwrap();
    Ok(warp::reply::json(&result))
}
