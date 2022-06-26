use products::api;
use products::products::load_products;
use products::rpc::rpc_client;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let products = load_products("products.json");
    let inventory = rpc_client().await.unwrap();

    let api = api::api(products.clone(), inventory.clone());
    let filter = api.with(warp::log("api"));

    warp::serve(filter).run(([0, 0, 0, 0], get_port())).await;
}

fn get_port() -> u16 {
    match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => 8001,
    }
}
