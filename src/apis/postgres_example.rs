use crate::AppState;
use serde_json::{json, Value};
use sqlx::query_scalar;

pub async fn create_table(mut _req: tide::Request<AppState>) -> tide::Result {
    let pool = &_req.state().pg_pool;
    let rows: Value = query_scalar("select * from user").fetch_one(pool).await?;

    Ok(rows.into())
}

pub async fn table_info(mut _req: tide::Request<AppState>) -> tide::Result {
    let pool = &_req.state().pg_pool;
    let rows: Value = query_scalar("select * from user").fetch_one(pool).await?;

    Ok(rows.into())
}

pub async fn add(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok(json!({"msg":"add"}).into())
}

pub async fn update(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok(json!({"msg":"update"}).into())
}

pub async fn find(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok(json!({"msg":"find"}).into())
}

pub async fn del(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok(json!({"msg":"del"}).into())
}
