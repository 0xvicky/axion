use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct UserPayload {
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "userPassword")]
    pub user_password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_email: String,
}
