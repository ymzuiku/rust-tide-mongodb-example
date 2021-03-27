use crate::AppState;

async fn info(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok("info".into())
}

async fn add(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok("add".into())
}

async fn update(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok("update".into())
}

async fn find(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok("find".into())
}

async fn del(mut _req: tide::Request<AppState>) -> tide::Result {
    Ok("del".into())
}
