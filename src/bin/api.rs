use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router, Server};
use serde::{Deserialize, Serialize};
use sqlx::mysql:: MySqlPoolOptions;
use sqlx::{MySql, Pool};
use dotenv::dotenv;
use std::sync::Arc;
use anyhow::Result;

#[derive(Serialize, Deserialize)]
struct Test<'a> {
    nombre: &'a str,
}

async fn hello() -> Json<Test<'static>> {
    Test { nombre: "A" }.into()
}

#[derive(Clone)]
struct State {
    inner: Arc<Inner>
}

struct Inner {
    db: Pool<MySql>
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let db = dotenv::var("DATABASE_URL").unwrap();
    
    let state = State {
        inner: Arc::new( Inner {
            db: MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db).await?
        })
    };
    
     let router = Router::new().route("/", get(hello))
        .with_state(state);

 Server::bind(&"[::]:80".parse().unwrap())
        .serve(router.into_make_service())
        .await?;
        
    Ok(())
}
