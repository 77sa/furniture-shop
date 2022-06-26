use inventory::api;
use inventory::inventory::load_inventory;
use inventory::rpc::rpc_server;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let inventory = load_inventory("inventory.json");

    let rpc_inventory = inventory.clone();
    let rpc =
        tokio::spawn(async move { rpc_server(rpc_inventory).await.unwrap() });

    let api = api::api(inventory.clone());
    let filter = api.with(warp::log("api"));

    let (_api, rpc) =
        tokio::join!(warp::serve(filter).run(([0, 0, 0, 0], get_port())), rpc);
    rpc?;
    
    Ok(())
}

fn get_port() -> u16 {
    match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => 8000,
    }
}
