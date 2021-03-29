use std::time;

use crate::env_json;
use futures::stream::StreamExt;
use mongodb::{bson::Document, Cursor};

pub async fn client() -> Result<mongodb::Client, anyhow::Error> {
    let mut client_options = mongodb::options::ClientOptions::parse(&format!(
        "mongodb://{}:{}@localhost:5701",
        env_json::load().get("mgo_user").unwrap(),
        env_json::load().get("mgo_password").unwrap(),
    ))
    .await?;

    client_options.max_idle_time = Some(time::Duration::new(1, 0));

    let client = mongodb::Client::with_options(client_options).unwrap();

    Ok(client)
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
