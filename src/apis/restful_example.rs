use crate::AppState;
use mongodb::bson::Document;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    age: i32,
}

pub async fn post_error(mut _req: tide::Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(tide::StatusCode::Unauthorized);
    res.set_body(json!({"error":"teset error"}));

    // 主动返回错误，使用 Ok 返回，statuc 中为错误码即可；而不是 Err。
    Ok(res)
}

pub async fn post_get_error(mut _req: tide::Request<AppState>) -> tide::Result {
    let data: Document = _req.body_json().await?;

    // 错误会捕获成为 tide::StatusCode::BadRequest, body 为 ""
    Ok(json!({"the-name":data.get_str("name")?}).into())
}

pub async fn post_unwrap(mut _req: tide::Request<AppState>) -> tide::Result {
    let data: Document = _req.body_json().await?;

    // 不要使用 unwrap，无法捕获成为 tide::Error
    Ok(json!({"the-name":data.get_str("name").unwrap()}).into())
}

pub async fn post_panic(mut _req: tide::Request<AppState>) -> tide::Result {
    // 不要使用 painc，无法捕获成为 tide::Error
    if true {
        panic!("oh no! panic!")
    }

    Ok(json!({"the-name":20}).into())
}

pub async fn get_query(mut _req: tide::Request<AppState>) -> tide::Result {
    let Animal { name, age } = _req.query()?;

    Ok(json!({"the-get-name": name, "age": age}).into())
}

pub async fn get_params(mut _req: tide::Request<AppState>) -> tide::Result {
    let name: String = _req.param("name")?.to_owned();
    let age: i32 = _req.param("age")?.parse()?;

    Ok(json!({"the-get-name": name, "age": age}).into())
}

pub async fn post_body(mut _req: tide::Request<AppState>) -> tide::Result {
    let Animal { name, age } = _req.body_json().await?;

    Ok(json!({"the-name":name, "the-age":age}).into())
}

pub async fn post_body_doc(mut _req: tide::Request<AppState>) -> tide::Result {
    let data: Document = _req.body_json().await?;

    Ok(json!({"the-name":data.get_str("name")?, "the-age":data.get_i32("age")?}).into())
}
