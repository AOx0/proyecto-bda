use askama::Template;
use axum::extract::{Path, State};
use axum::http::{header, HeaderName};
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router, Server};
use chrono::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::sync::Arc;

// TODO: Esto es un mega haack exagerado. Deberia de ser el id directamente el año
const OFFSET: u16 = 1947;
const JS_HEADER: AppendHeaders<[(HeaderName, &str); 1]> =
    AppendHeaders([(header::CONTENT_TYPE, "text/javascript")]);

const CSS_HEADER: AppendHeaders<[(HeaderName, &str); 1]> =
    AppendHeaders([(header::CONTENT_TYPE, "text/css")]);

const SVG_HEADER: AppendHeaders<[(HeaderName, &str); 1]> =
    AppendHeaders([(header::CONTENT_TYPE, "image/svg+xml")]);
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

struct Section {
    name: &'static str,
    href: &'static str,
}

#[derive(Template)]
#[template(path = "hello.html")]
struct Hello<'a> {
    posts: &'a [Content],
    sects: &'a [Section],
}

async fn root() -> Hello<'static> {
    Hello {
        posts: &[
            Content {
                name: "Muertos",
                content: " ",
                desc: "+0.12 de la semana pasada",
                method: "/date/2023-02-23",
            },
            Content {
                name: "Robos",
                content: " ",
                desc: "Robos armados",
                method: "/date/2023-02-24",
            },
            Content {
                name: "Homicidios",
                content: " ",
                desc: "En esta semana",
                method: "/date/2023-02-25",
            },
            Content {
                name: "Carpetas",
                content: " ",
                desc: "En esta año",
                method: "/date/upnow",
            },
        ],
        sects: &[Section {
            name: "Zonas calientes",
            href: "#",
        }],
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

async fn mapa(Path(n): Path<usize>) -> impl IntoResponse {
    (
        SVG_HEADER,
        format!(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/MXfmt.svg")),
            n
        ),
    )
}

async fn scripts() -> impl IntoResponse {
    (
        JS_HEADER,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/script.js")),
    )
}

async fn tailwind() -> impl IntoResponse {
    (
        CSS_HEADER,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/style.css")),
    )
}

async fn date(State(state): State<Shared>, Path(date): Path<String>) -> String {
    let invalid = date.chars().any(|a| !(a.is_ascii_digit() || a == '-'));

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

#[derive(Serialize, Debug, Default)]
struct MapaPorcetajes {
    total: u64,
    valores: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct SolicitudMapaPorcentajes {
    #[serde(default = "min_year")]
    annio_inicio: u16,
    #[serde(default = "max_year")]
    annio_final: u16,
    #[serde(default)]
    categorias: Vec<u16>,
}

fn min_year() -> u16 {
    2016
}
fn max_year() -> u16 {
    2023
}

/// TODO: This seems expensive, benchmark and optimize
///
/// # Panics
///
/// Panics if .
async fn mapa_porcentajes(
    State(state): State<Shared>,
    Json(sol): Json<SolicitudMapaPorcentajes>,
) -> Json<MapaPorcetajes> {
    let SolicitudMapaPorcentajes {
        annio_inicio,
        annio_final,
        categorias,
    } = dbg!(sol);

    let annio_inicio = annio_inicio - OFFSET;
    let annio_final = annio_final - OFFSET;

    let (total,): (i64,) = if categorias.is_empty() {
        sqlx::query_as(&format!("SELECT COUNT(1) FROM delitos WHERE id_anio_hecho BETWEEN {annio_inicio} AND {annio_final};")) 
            .fetch_one(&state.db)
            .await
            .unwrap()
    } else {
        sqlx::query_as(&format!("SELECT COUNT(1) FROM delitos WHERE delitos.id_categoria IN ({0}) AND id_anio_hecho BETWEEN {annio_inicio} AND {annio_final};",
            categorias
                .iter()
                .map(|id| format!("{id}"))
                .collect::<Vec<_>>()
                .join(",")
        ))
            .fetch_one(&state.db)
            .await
            .unwrap()
    };

    let resultados: Vec<(i64,)> = if categorias.is_empty() {
        sqlx::query_as(&format!("SELECT COUNT(1) FROM delitos WHERE id_anio_hecho BETWEEN {annio_inicio} AND {annio_final} AND delitos.id_alcaldia_hecho IS NOT NULL GROUP BY delitos.id_alcaldia_hecho ORDER BY delitos.id_alcaldia_hecho;")) 
            .fetch_all(&state.db)
            .await
            .unwrap()
    } else {
        sqlx::query_as(&format!("SELECT COUNT(1) FROM delitos WHERE delitos.id_categoria IN ({0}) AND id_anio_hecho BETWEEN {annio_inicio} AND {annio_final} AND delitos.id_alcaldia_hecho IS NOT NULL GROUP BY delitos.id_alcaldia_hecho ORDER BY delitos.id_alcaldia_hecho;",
            categorias
                .iter()
                .map(|id| format!("{id}"))
                .collect::<Vec<_>>()
                .join(",")
        ))
            .fetch_all(&state.db)
            .await
            .unwrap()
    };

    MapaPorcetajes {
        total: u64::try_from(total).unwrap(),
        valores: resultados.into_iter().map(|(n,)| n as u64).collect(),
    }
    .into()
}

async fn untilnow(State(state): State<Shared>) -> String {
    let utc: DateTime<Utc> = Utc::now();
    let year = utc.format("%Y").to_string();
    let year: u16 = year.parse().unwrap();

    let row: Result<(String,), _> =
        sqlx::query_as("SELECT FORMAT((SELECT COUNT(1) FROM delitos WHERE id_anio_hecho = ?), 0)")
            .bind(&dbg!(year - OFFSET))
            .fetch_one(&state.db)
            .await;

    match row {
        Ok((row,)) => format!("+{}", row),
        Err(err) => {
            println!("Error: {err}");
            "INTERR".to_string()
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
        .route("/", get(root))
        .route("/alpine.js", get(alpine))
        .route("/tailwind.css", get(tailwind))
        .route("/htmx.js", get(htmx))
        .route("/script.js", get(scripts))
        .route("/mapa/:n", get(mapa))
        .route("/health", get(|| async { "alive" }))
        .route("/date/:date", get(date))
        .route("/map_percent", post(mapa_porcentajes))
        .route("/date/upnow", get(untilnow))
        .with_state(state);

    Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
