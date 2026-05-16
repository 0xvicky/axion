use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub user_id: ObjectId,
    pub user_email: String,
    pub password: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}
