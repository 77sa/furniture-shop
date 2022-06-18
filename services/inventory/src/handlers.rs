use crate::inventory::Inventory;

pub async fn get_inventory(
    inventory: Inventory,
) -> Result<impl warp::Reply, warp::Rejection> {
    let json = inventory.lock().unwrap();
    Ok(warp::reply::json(&*json))
}
