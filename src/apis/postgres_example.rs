use crate::AppState;
use serde_json::json;

pub async fn create_table(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok("create-table".into())
}

pub async fn table_info(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok("table_info".into())
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
