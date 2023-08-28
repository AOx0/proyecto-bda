use askama::Template;
use axum::extract::{Path, State};
use axum::http::{header, HeaderName};
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::get;
use axum::{Router, Server};
use chrono::prelude::*;
use dotenv::dotenv;
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::sync::Arc;

const JS_HEADER: AppendHeaders<[(HeaderName, &str); 1]> =
    AppendHeaders([(header::CONTENT_TYPE, "text/javascript")]);

const CSS_HEADER: AppendHeaders<[(HeaderName, &str); 1]> =
    AppendHeaders([(header::CONTENT_TYPE, "text/css")]);

#[derive(Serialize)]
struct Test {
    nombre: String,
    resultado: String,
}

struct Content {
    name: &'static str,
    content: &'static str,
    desc: &'static str,
    method: &'static str,
}

#[derive(Template)]
#[template(path = "hello.html")]
struct Hello<'a> {
    name: &'a str,
    posts: &'a [Content],
}

async fn root() -> Hello<'static> {
    Hello {
        name: "world",
        posts: &[
            Content {
                name: "Muertos",
                content: "",
                desc: "+0.12 de la semana pasada",
                method: "/date/2023-02-23",
            },
            Content {
                name: "Robos",
                content: "",
                desc: "Robos armados",
                method: "/date/2023-02-24",
            },
            Content {
                name: "Homicidios",
                content: "",
                desc: "En esta semana",
                method: "/date/2023-02-25",
            },
            Content {
                name: "Carpetas",
                content: "",
                desc: "En esta año",
                method: "/date/upnow",
            },
        ],
    }
}

async fn htmx() -> impl IntoResponse {
    (
        JS_HEADER,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/htmx.min.js")),
    )
}

async fn alpine() -> impl IntoResponse {
    (
        JS_HEADER,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/cdn.min.js")),
    )
}

async fn tailwind() -> impl IntoResponse {
    (
        CSS_HEADER,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/style.css")),
    )
}

async fn date(State(state): State<Shared>, Path(date): Path<String>) -> String {
    let invalid = date
        .chars()
        .find(|a| !(a.is_digit(10) || a == &'-'))
        .is_some();

    if invalid {
        return format!("Invalid date '{date}'");
    }

    let row: Result<(String,), _> =
        sqlx::query_as("SELECT FORMAT((SELECT COUNT(1) FROM delitos WHERE fecha_hecho = ?), 0)")
            .bind(&date)
            .fetch_one(&state.db)
            .await;

    match row {
        Ok((row,)) => format!("+{}", row),
        Err(_) => "INTERR".to_string(),
    }
}

async fn untilnow(State(state): State<Shared>) -> String {
    let utc: DateTime<Utc> = Utc::now();
    let year = utc.format("%Y").to_string();

    let row: Result<(String,), _> = sqlx::query_as(
        "SELECT FORMAT((SELECT COUNT(1) FROM delitos WHERE YEAR(fecha_hecho) = ?), 0)",
    )
    .bind(&year)
    .fetch_one(&state.db)
    .await;

    match row {
        Ok((row,)) => format!("+{}", row),
        Err(err) => {
            println!("Error: {err}");
            "INTERR".to_string()
        },
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
        .route("/", get(root))
        .route("/alpine.js", get(alpine))
        .route("/tailwind.css", get(tailwind))
        .route("/htmx.js", get(htmx))
        .route("/health", get(|| async { "alive" }))
        .route("/date/:date", get(date))
        .route("/date/upnow", get(untilnow))
        .with_state(state);

    Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
