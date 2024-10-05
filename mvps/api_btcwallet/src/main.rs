#![allow(unused)]
use axum::extract::Path;
use axum::extract::{Extension, Query};
use axum::middleware;
use axum::response::Response;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::routing::get_service;
use axum::Router;
use env_logger;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper));

    // response mapper
    async fn main_response_mapper(res: Response) -> Response {
        println!("-->>{:<12}", "MAPPER RESPONSE");
        println!(" ");
        res
    }
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("->>  LISTENING ON {}", &addr);
    axum::serve(listener, app).await.unwrap();
}

async fn routes_static() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
