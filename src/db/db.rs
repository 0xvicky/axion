use mongodb::{Client, bson::doc};
use std::env;

pub async fn ping_db(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    client
        .database("admin")
        .run_command(doc! {"ping":1})
        .await?;

    println!("ping success");
    Ok(())
}

pub async fn db_init() -> Result<Client, Box<dyn std::error::Error>> {
    let mongo_uri = env::var("MONGO_URI")?;

    let client = Client::with_uri_str(mongo_uri).await?;
    ping_db(&client).await?;
    Ok(client)
}
