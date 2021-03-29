use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::env_json;

pub async fn make_pool() -> Result<Pool<Postgres>, anyhow::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env_json::load().get("pg_url").unwrap())
        .await?;

    Ok(pool)
}

// pub async fn sql(pool: &Pool<Postgres>, str: &str) -> Result<Value, anyhow::Error> {
//     let mut rows = query(str).fetch_all(&pool).await?;
// }
