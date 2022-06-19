use inventory::api;
use inventory::inventory::{load_inventory, Article};
use std::str;

const HEALTH_PATH: &str = "/health";
const INVENTORY_PATH: &str = "/inventory/v1";

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
async fn get_inventory_route() {
    let inventory = load_inventory("test.json");
    let get_inventory = api::get_inventory(inventory);

    let res = warp::test::request()
        .method("GET")
        .path(INVENTORY_PATH)
        .reply(&get_inventory)
        .await;

    assert_eq!(res.status(), 200);

    let res_str = str::from_utf8(&res.body()).unwrap();
    let res: Vec<Article> = serde_json::from_str(res_str).unwrap();

    let test_res = vec![
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
    ];

    assert_eq!(res, test_res);
}
