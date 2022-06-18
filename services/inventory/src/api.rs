use crate::handlers;
use crate::inventory::Inventory;
use warp::filters::BoxedFilter;
use warp::{path, Filter};

fn path_prefix() -> BoxedFilter<()> {
    path!("inventory" / "v1").boxed()
}

pub fn api(
    inventory: Inventory,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health().or(get_inventory(inventory.clone()))
}

fn health(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get().and(path!("health")).and_then(|| async {
        Ok::<_, warp::Rejection>(warp::reply::with_status(
            "OK",
            warp::http::status::StatusCode::OK,
        ))
    })
}

pub fn get_inventory(
    inventory: Inventory,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::any().map(move || inventory.clone()))
        .and_then(handlers::get_inventory)
}
