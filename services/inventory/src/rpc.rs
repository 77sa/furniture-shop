use crate::inventory::{self, Article};
use futures::{future, prelude::*};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};

#[tarpc::service]
trait Inventory {
    async fn get(id: Option<usize>) -> Vec<Article>;
    async fn buy(requires: Vec<usize>) -> String;
}

#[derive(Clone)]
struct InventoryServer(SocketAddr, inventory::Inventory);

#[tarpc::server]
impl Inventory for InventoryServer {
    async fn get(self, _: context::Context, id: Option<usize>) -> Vec<Article> {
        let articles_lock = self.1.lock().unwrap();

        if let Some(id) = id {
            let article = &articles_lock[id];
            let article = article.clone();
            vec![article]
        } else {
            let articles = &*articles_lock;
            let articles = articles.clone();
            articles
        }
    }

    async fn buy(self, _: context::Context, requires: Vec<usize>) -> String {
        let mut articles_lock = self.1.lock().unwrap();
        let mut stock: Vec<usize> = Vec::new();

        for id in &requires {
            stock.push(articles_lock[*id].stock)
        }

        if stock.contains(&0) {
            return String::from("Out of stock");
        }

        for article in &mut *articles_lock {
            for id in &requires {
                if article.id == *id {
                    article.stock -= 1
                }
            }
        }

        String::from("Success")
    }
}

pub async fn rpc_server(inventory: inventory::Inventory) -> anyhow::Result<()> {
    const PORT: u16 = 12000;
    let server_addr = (IpAddr::V4(Ipv4Addr::UNSPECIFIED), PORT);

    let mut listener =
        tarpc::serde_transport::tcp::listen(&server_addr, Json::default)
            .await?;
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = InventoryServer(
                channel.transport().peer_addr().unwrap(),
                inventory.clone(),
            );
            channel.execute(server.serve())
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;
    println!("RPC server started");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::inventory::load_inventory;

    #[tokio::test]
    async fn it_should_get_the_inventory() {
        let inventory = load_inventory("test.json");
        const PORT: u16 = 12000;
        let inventory_server = InventoryServer(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), PORT),
            inventory,
        );

        let res = inventory_server.get(context::current(), Some(0)).await;

        let test_res = vec![Article {
            id: 0,
            name: "Wood".to_owned(),
            stock: 10,
        }];

        assert_eq!(res, test_res);
    }

    #[tokio::test]
    async fn it_should_decrement_the_stock() {
        let inventory = load_inventory("test.json");
        const PORT: u16 = 12000;
        let inventory_server = InventoryServer(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), PORT),
            inventory,
        );

        let res = inventory_server.clone().buy(context::current(), vec![0, 1]).await;

        assert_eq!(res, "Success".to_owned());

        let res = inventory_server.get(context::current(), None).await;

        let test_res = vec![
            Article {
                id: 0,
                name: "Wood".to_owned(),
                stock: 9,
            },
            Article {
                id: 1,
                name: "Metal".to_owned(),
                stock: 11,
            },
        ];

        assert_eq!(res, test_res);
    }
}
