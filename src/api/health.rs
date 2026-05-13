use axum::Json;
use serde_json::{Value, json};

pub async fn health() -> Json<Value> {
    let res = json!({
        "status":"ok",
        "message":"axion"
    });

    Json(res)
}
