use crate::dto::user_dto::{self, UserResponse};
use axum::Json;
use time::OffsetDateTime;

pub async fn create_user(Json(payload): Json<user_dto::UserPayload>) -> Json<UserResponse> {
    println!("Email:{}", payload.user_email);
    println!("Password:{}", payload.user_password);
    let email = payload.user_email;
    let created_at = OffsetDateTime::now_utc();
    // println!("{}", created_at);
    Json(UserResponse {
        user_email: email,
        created_at: created_at,
    })
}
