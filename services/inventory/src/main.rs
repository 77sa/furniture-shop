use inventory::api;
use inventory::inventory::load_inventory;
use inventory::rpc::rpc_server;
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
        tokio::join!(warp::serve(filter).run(([0, 0, 0, 0], 8000)), rpc);
    rpc?;
    
    Ok(())
}
