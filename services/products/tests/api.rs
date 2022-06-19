use products::handlers::{PostBuyRequest, Response};
use products::{api, rpc};
use std::str;

const HEALTH_PATH: &str = "/health";
const INVENTORY_PATH: &str = "/products/v1";
const BUY_PATH: &str = "/products/v1/buy";

#[tokio::test]
async fn health_route() {
    let health = api::health();

    let res = warp::test::request()
        .method("GET")
        .path(HEALTH_PATH)
        .reply(&health)
        .await;

    assert_eq!(res.status(), 200);

    let res_str = str::from_utf8(&res.body()).unwrap();

    assert_eq!(res_str, "OK")
}

#[tokio::test]
async fn buy_and_get() {
    let products = products::products::load_products("test.json");
    let inventory = rpc::rpc_client().await.unwrap();

    
    // First get:
    let get = api::get(products.clone(), inventory.clone());
    let res = warp::test::request()
        .method("GET")
        .path(INVENTORY_PATH)
        .reply(&get)
        .await;

    assert_eq!(res.status(), 200);

    let res_str = str::from_utf8(&res.body()).unwrap();
    let res: Vec<Response> = serde_json::from_str(res_str).unwrap();

    let test_res = vec![
        Response {
            id: 0,
            name: "Bed".to_owned(),
            stock: 5,
        },
        Response {
            id: 1,
            name: "Sofa".to_owned(),
            stock: 5,
        },
    ];

    assert_eq!(res, test_res, "First GET");
    
    // Buy
    let buy = api::buy(products.clone(), inventory.clone());
    let req = PostBuyRequest { id: 0 };
    let res = warp::test::request()
        .method("POST")
        .path(BUY_PATH)
        .json(&req)
        .reply(&buy)
        .await;

    assert_eq!(res.status(), 200);

    let res_str = str::from_utf8(&res.body()).unwrap();
    let res: String = serde_json::from_str(res_str).unwrap();

    assert_eq!(res, "Success");

    // Second get:
    let get = api::get(products, inventory);
    let res = warp::test::request()
        .method("GET")
        .path(INVENTORY_PATH)
        .reply(&get)
        .await;

    assert_eq!(res.status(), 200);

    let res_str = str::from_utf8(&res.body()).unwrap();
    let res: Vec<Response> = serde_json::from_str(res_str).unwrap();

    let test_res = vec![
        Response {
            id: 0,
            name: "Bed".to_owned(),
            stock: 4,
        },
        Response {
            id: 1,
            name: "Sofa".to_owned(),
            stock: 4,
        },
    ];

    assert_eq!(res, test_res, "Second GET");
}
