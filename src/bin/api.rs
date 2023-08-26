use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router, Server};
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::mysql:: MySqlPoolOptions;
use sqlx::{MySql, Pool};
use dotenv::dotenv;
use std::sync::Arc;
use std::borrow::Cow;


#[derive(Serialize, Deserialize)]
struct Test {
    nombre: Cow<'static, str>,
}

async fn hello(State(state): State<Shared>) -> Result<Json<Test>, Json<Test>> {
    let row: Result<(i64,), _> = sqlx::query_as("SELECT DISTINCT YEAR(fecha_hecho) FROM delitos")
        .fetch_one(&state.db).await;
        
    match row {
        Ok((row,)) => Ok(Test { nombre: format!("{row:?}").into() }.into()),
        Err(err) => Err(Test { nombre: format!("{err:?}").into() }.into())
    }
        
    
}

#[derive(Clone)]
struct Shared (Arc<Inner>);

impl std::ops::Deref for Shared {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Inner {
    db: Pool<MySql>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    
    let db = dotenv::var("DATABASE_URL").unwrap();
    
    let state = Shared (
        Arc::new( Inner {
            db: MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db).await?
        })
    );
    
     let router = Router::new().route("/", get(hello))
        .with_state(state);

 Server::bind(&"[::]:80".parse().unwrap())
        .serve(router.into_make_service())
        .await?;
        
    Ok(())
}
