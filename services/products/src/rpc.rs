use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, Ipv4Addr},
    thread,
    time::Duration,
};
use tarpc::{client, tokio_serde::formats::Json};

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: usize,
    pub name: String,
    pub stock: usize,
}

#[tarpc::service]
pub trait Inventory {
    async fn get(id: Option<usize>) -> Vec<Article>;
    async fn buy(requires: Vec<usize>) -> String;
}

pub async fn rpc_client() -> anyhow::Result<InventoryClient> {
    const PORT: u16 = 12000;
    let server_addr = (IpAddr::V4(Ipv4Addr::UNSPECIFIED), PORT);

    let transport;
    loop {
        transport = match tarpc::serde_transport::tcp::connect(
            server_addr,
            Json::default,
        )
        .await
        {
            Ok(transport) => {
                println!("Connected to RPC server");
                transport
            }
            Err(_) => {
                println!("Failed to connect to RPC server");
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };
        break;
    }

    let client =
        InventoryClient::new(client::Config::default(), transport).spawn();

    Ok(client)
}
