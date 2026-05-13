use axum::{Json, extract::Path};
use serde_json::{Value, json};

pub async fn get_name(Path(username): Path<String>) -> Json<Value> {
    let res = json!({
        "username":username
    });

    Json(res)
}
