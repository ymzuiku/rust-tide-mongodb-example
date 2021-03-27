use mongodb::{
    bson::{doc, Document},
    options::{self, FindOptions},
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
    age: i64,
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

pub async fn update(mut _req: tide::Request<AppState>) -> tide::Result {
    let body: Document = _req.body_json().await?;

    let coll = get_user_coll(&_req.state().mongo_client);

    let res = coll
        .update_many(
            doc! {"age":body.get_i32("age")?},
            doc! {"name":body.get_str("name")?},
            None,
        )
        .await?;

    Ok(json!(res.upserted_id).into())
}

pub async fn find(mut _req: tide::Request<AppState>) -> tide::Result {
    let body: Animal = _req.body_json().await?;
    let coll = get_user_coll(&_req.state().mongo_client);

    let mut fo = FindOptions::default();
    fo.sort = Some(doc! {"age":1});
    fo.limit = Some(5);

    let data = mgo::find_to_list(coll.find(doc! {"name":body.name}, fo).await?).await;

    Ok(json!({"count": data.len(), "data": data}).into())
}

pub async fn find_by_document(mut _req: tide::Request<AppState>) -> tide::Result {
    let body: Document = _req.body_json().await?;
    let coll = get_user_coll(&_req.state().mongo_client);

    let mut fo = FindOptions::default();
    fo.sort = Some(doc! {"age":1});
    fo.limit = Some(5);

    let data = mgo::find_to_list(coll.find(doc! {"name":body.get_str("name")?}, fo).await?).await;

    Ok(json!({"count": data.len(), "data": data}).into())
}

pub async fn del(mut _req: tide::Request<AppState>) -> tide::Result {
    let coll = get_user_coll(&_req.state().mongo_client);
    let res = coll.delete_many(doc! {}, None).await?;

    Ok(json!({"type":"user delete many", "count": res.deleted_count }).into())
}
