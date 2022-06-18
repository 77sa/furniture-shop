use products::api;
use products::products::load_products;
use products::rpc::rpc_client;
use warp::Filter;

#[tokio::main]
async fn main() {
    let products = load_products();
    let inventory = rpc_client().await.unwrap();

    let api = api::api(products.clone(), inventory.clone());
    let filter = api.with(warp::log("api"));

    warp::serve(filter).run(([0, 0, 0, 0], 8001)).await;
}
