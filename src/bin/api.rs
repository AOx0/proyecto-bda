use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router, Server};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Serialize)]
struct Test<T: Serialize> {
    nombre: Cow<'static, str>,
    resultado: T,
}

async fn hello(
    State(state): State<Shared>,
) -> Result<Json<Test<impl Serialize>>, Json<Test<impl Serialize>>> {
    let row: Result<(i64,), _> = sqlx::query_as("SELECT DISTINCT YEAR(fecha_hecho) FROM delitos")
        .fetch_one(&state.db)
        .await;

    match row {
        Ok((row,)) => Ok(Test {
            nombre: "SELECT DISTINCT YEAR(fecha_hecho) FROM delitos".into(),
            resultado: row,
        }
        .into()),
        Err(err) => Err(Test {
            nombre: "SELECT DISTINCT YEAR(fecha_hecho) FROM delitos".into(),
            resultado: err.to_string(),
        }
        .into()),
    }
}

#[derive(Clone)]
struct Shared(Arc<Inner>);

impl std::ops::Deref for Shared {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Inner {
    db: Pool<MySql>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db = dotenv::var("DATABASE_URL").unwrap();

    let state = Shared(Arc::new(Inner {
        db: MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db)
            .await?,
    }));

    let router = Router::new().route("/", get(hello)).with_state(state);

    Server::bind(&"[::]:80".parse().unwrap())
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
