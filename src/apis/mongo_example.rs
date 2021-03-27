use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::mgo;
use crate::AppState;

pub async fn info(mut _res: tide::Request<AppState>) -> tide::Result {
    let client = &_res.state().mongo_client;

    Ok(json!({"msg":"get info", "dbs": client.list_database_names(None, None).await?}).into())
}

#[derive(Debug, Serialize, Deserialize)]
struct Animal {
    name: String,
    age: i32,
}

fn get_user_coll(client: &Client) -> Collection<Document> {
    let coll = client.database("rust").collection("user");
    coll
}

pub async fn add(mut _req: tide::Request<AppState>) -> tide::Result {
    let Animal { name, age } = _req.body_json().await?;

    let coll = get_user_coll(&_req.state().mongo_client);

    let res = coll.insert_one(doc! {"name":name, "age":age}, None).await?;

    Ok(json!(res.inserted_id).into())
}

pub async fn del(mut _req: tide::Request<AppState>) -> tide::Result {
    let coll = get_user_coll(&_req.state().mongo_client);
    let res = coll.delete_many(doc! {}, None).await?;

    Ok(json!({"type":"user delete many", "count": res.deleted_count }).into())
}

#[derive(Debug, Deserialize)]
struct Query {
    data: Document,
}

pub async fn find(mut _req: tide::Request<AppState>) -> tide::Result {
    let query: Query = _req.query()?;
    let coll = get_user_coll(&_req.state().mongo_client);

    println!("{:?}", query);
    println!("{:?}", doc! {"bbb":"ccc"});

    let mut fo = FindOptions::default();
    fo.sort = Some(doc! {"age":1});
    fo.limit = Some(5);

    let data = mgo::find_to_list(coll.find(doc! {"name":"dog"}, fo).await?).await;

    Ok(json!({"data": data, "count": data.len()}).into())
}
