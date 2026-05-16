use crate::db::state::DBState;
use crate::dto::user_dto::{self, UserResponse};
use crate::models::user::User;
use axum::{Json, extract::State};
use mongodb::bson::oid::ObjectId;
use time::OffsetDateTime;

pub async fn create_user(
    State(state): State<DBState>,
    Json(payload): Json<user_dto::UserPayload>,
) -> Result<Json<UserResponse>, String> {
    // println!("Email:{}", payload.user_email);
    // println!("Password:{}", payload.user_password);
    let created_at = OffsetDateTime::now_utc();
    let email = payload.user_email.clone();
    // println!("{}", created_at);
    let new_user = User {
        user_id: ObjectId::new(),
        user_email: payload.user_email,
        password: payload.user_password,
        created_at,
    };

    //insert into db
    let insert_result = state
        .collection
        .insert_one(new_user)
        .await
        .map_err(|err| err.to_string())?
        .inserted_id
        .as_object_id()
        .ok_or("Invalid object id")
        .map_err(|err| err.to_string())?
        .to_hex();

    Ok(Json(UserResponse {
        user_id: insert_result,
        user_email: email,
        created_at,
    }))
}
