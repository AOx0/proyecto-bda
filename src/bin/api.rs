use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router, Server};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::sync::Arc;

#[derive(Serialize)]
struct Test {
    nombre: String,
    resultado: String,
}

async fn date(
    State(state): State<Shared>,
    Path(date): Path<String>,
) -> Result<Json<Test>, Json<Test>> {
    let invalid = date.chars().find(|a| !(a.is_digit(10) || a == &'-')).is_some();
    if invalid {
        return Err(
            Test {
                nombre: "Error".to_string(),
                resultado: format!("Invalid date '{date}'")
            }.into()
        );
    }
    
    let row: Result<(i64,String), _> = sqlx::query_as(
        "SELECT COUNT(1), DATE_FORMAT(?, '%Y-%m-%d') FROM delitos WHERE fecha_hecho = ? GROUP BY fecha_hecho",
    )
    .bind(&date)
    .bind(&date)
    .fetch_one(&state.db)
    .await;

    match row {
        Ok((row, date)) => Ok(Test {
            nombre: date.into(),
            resultado: format!("{}", row),
        }
        .into()),
        Err(err) => {
            eprintln!("Error with query 'date' (date: {date}): {err}");
            Err(Test {
                nombre: "Error".into(),
                resultado: "Internal error".to_string(),
            }
            .into())
        }
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
    let address = std::env::args().nth(1).unwrap_or("[::]:80".to_string());

    dotenv().ok();

    let db = dotenv::var("DATABASE_URL")?;

    let state = Shared(Arc::new(Inner {
        db: MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db)
            .await?,
    }));

    let router = Router::new()
        .route("/date/:date", get(date))
        .route("/health", get(|| async { "alive" }))
        .with_state(state);

    Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
