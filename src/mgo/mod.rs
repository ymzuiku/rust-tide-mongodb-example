use std::{env::var, time};

use futures::stream::StreamExt;
use mongodb::{bson::Document, Cursor};

const MGO_USER: String = var("MGO_USER").unwrap();
const MGO_PASSWORD: String = var("MGO_PASSWORD").unwrap();

pub async fn client() -> mongodb::Client {
    let mut client_options = mongodb::options::ClientOptions::parse(&format!(
        "mongodb://{}:{}@localhost:5701",
        MGO_USER, MGO_PASSWORD
    ))
    .await
    .unwrap();

    client_options.max_idle_time = Some(time::Duration::new(1, 0));

    let client = mongodb::Client::with_options(client_options).unwrap();

    client
}

pub async fn find_to_list(mut cursor: Cursor<Document>) -> Vec<Document> {
    let mut data = vec![];

    while let Some(item) = cursor.next().await {
        match item {
            Ok(v) => data.push(v),
            Err(e) => {
                println!("find_to_list error: {}", e);
            }
        }
    }
    return data;
}
