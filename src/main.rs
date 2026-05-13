mod api;
use api::health::health;

use anyhow::Result;
use axum::{Router, routing::get};
#[tokio::main]
async fn main() -> Result<()> {
    // println!("Hello, world!");
    let app = Router::new().route("/", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await?;
    println!("server running at 6969");
    axum::serve(listener, app).await?;
    Ok(())
}
