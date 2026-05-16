mod api;
mod db;
mod dto;
mod models;
use crate::api::create_user::create_user;
use crate::api::get_name::get_name;
use crate::api::health::health;
use crate::db::db::db_init;

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");res data on heap and gives fixed-size pointer on stac
    //connect to db
    dotenv().ok();

    let db_state = db_init().await?;
    // let state = DBState { db_client: client };

    let app = Router::new()
        .route("/health", get(health))
        .route("/name/{username}", get(get_name))
        .route("/create-user", post(create_user))
        .with_state(db_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await?;
    println!("server running at 6969");
    axum::serve(listener, app).await?;
    Ok(())
}
