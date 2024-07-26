mod data;
mod repository;
mod routes;
mod utils;

use repository::mongo::mongo::DB;
use routes::routes::create_router;
use utils::error::MyError;
use utils::info::info;

use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
pub struct AppState {
    db: DB,
}
#[tokio::main]
async fn main() -> Result<(), MyError> {
    dotenv().ok();

    info();

    let db = DB::init().await?;

    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap()) //TODO: improve by specifying actual origin
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: db.clone() })).layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
