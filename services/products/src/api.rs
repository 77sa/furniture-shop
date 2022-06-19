use crate::handlers;
use crate::products::Products;
use crate::rpc::InventoryClient;
use warp::filters::BoxedFilter;
use warp::{path, Filter};

fn prefix() -> BoxedFilter<()> {
    path!("products" / "v1").boxed()
}

fn buy_prefix() -> BoxedFilter<()> {
    path!("products" / "v1" / "buy").boxed()
}

pub fn api(
    products: Products,
    inventory: InventoryClient,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health()
        .or(get(products.clone(), inventory.clone()))
        .or(buy(products, inventory))
}

pub fn health(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get().and(path!("health")).and_then(|| async {
        Ok::<_, warp::Rejection>(warp::reply::with_status(
            "OK",
            warp::http::status::StatusCode::OK,
        ))
    })
}

pub fn get(
    products: Products,
    inventory: InventoryClient,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(prefix())
        .and(warp::path::end())
        .and(warp::any().map(move || products.clone()))
        .and(warp::any().map(move || inventory.clone()))
        .and_then(handlers::get)
}

pub fn buy(
    products: Products,
    inventory: InventoryClient,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let json =
        warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(buy_prefix())
        .and(warp::path::end())
        .and(warp::any().map(move || products.clone()))
        .and(warp::any().map(move || inventory.clone()))
        .and(json)
        .and_then(handlers::buy)
}
