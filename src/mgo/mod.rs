use std::time;

use futures::stream::StreamExt;
use mongodb::{bson::Document, Cursor};

pub async fn client() -> mongodb::Client {
    let mut client_options =
        mongodb::options::ClientOptions::parse("mongodb://root:pillarQwe.123@localhost:5701")
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
