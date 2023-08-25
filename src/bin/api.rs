use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router, Server};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Test<'a> {
    nombre: &'a str,
}

async fn hello() -> Json<Test<'static>> {
    Test { nombre: "A" }.into()
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello));

    Server::bind(&"[::]:80".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
