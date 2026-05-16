use crate::models::user::User;
use mongodb::{Collection, Database};

#[derive(Clone)]
pub struct DBState {
    pub collection: Collection<User>,
    pub db: Database,
}
