use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPayload {
    pub user_email: String,
    pub user_password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_email: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}
