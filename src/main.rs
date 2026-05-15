mod api;
mod dto;
use crate::api::create_user::create_user;
use crate::api::get_name::get_name;
use crate::api::health::health;

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() -> Result<()> {
    // println!("Hello, world!");
    let app = Router::new()
        .route("/health", get(health))
        .route("/name/{username}", get(get_name))
        .route("/create-user", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await?;
    println!("server running at 6969");
    axum::serve(listener, app).await?;
    Ok(())
}
