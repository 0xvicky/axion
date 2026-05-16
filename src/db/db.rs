use crate::db::state::DBState;
use crate::models::user::User;
use mongodb::{Client, bson::doc};
use std::env;

pub async fn ping_db(state: &DBState) -> Result<(), Box<dyn std::error::Error>> {
    state.db.run_command(doc! {"ping":1}).await?;
    println!("ping success");
    Ok(())
}

pub async fn db_init() -> Result<DBState, Box<dyn std::error::Error>> {
    let mongo_uri = env::var("MONGO_URI")?;

    let client = Client::with_uri_str(mongo_uri).await?;

    let database = client.database("dev");
    let user_collection = database.collection::<User>("users");

    let db_state = DBState {
        db: database,
        collection: user_collection,
    };
    ping_db(&db_state).await?;

    Ok(db_state)
}
