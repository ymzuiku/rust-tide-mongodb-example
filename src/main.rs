use serde_json::json;
use tide::{utils::After, Server};
mod mgo;

mod apis;

#[derive(Debug, Clone)]
pub struct AppState {
    pub mongo_client: mongodb::Client,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let port = "8802";
    println!("listen http://127.0.0.1:{}", port);

    let mut app = make_app().await?;

    // app.with(After(after_error));

    apis::register_apis(&mut app);
    app.at("/").get(static_index);
    app.at("/").serve_dir("public")?;

    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}

async fn make_app() -> tide::Result<Server<AppState>> {
    let mongo_client = mgo::client().await;
    let app = Server::with_state(AppState { mongo_client });

    Ok(app)
}

async fn after_error(mut res: tide::Response) -> tide::Result<tide::Response> {
    if res.is_empty().unwrap_or(true) {
        res.set_body(json!({"error":"get value error"}).to_string());
    }

    Ok(res)
}

async fn static_index(_req: tide::Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(tide::StatusCode::Ok);
    res.set_body(tide::Body::from_file("public/index.html").await.unwrap());

    Ok(res)
}
