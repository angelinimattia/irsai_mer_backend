use mongodb::{error::Result, options::ClientOptions, Client, Collection, Database};
use std::env;
use crate::models::drone_status::DroneStatus;

pub struct MongoDB {
    db: Database,
}

impl MongoDB {
    pub async fn new() -> Result<Self> {
        let uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let db_name = env::var("MONGO_DB_NAME").unwrap_or_else(|_| "test_db".to_string());
        
        let options = ClientOptions::parse(&uri).await?;
        let client = Client::with_options(options)?;
        let db = client.database(&db_name);
        Ok(Self { db })
    }

    pub fn get_drone_status(&self) -> Collection<DroneStatus> {
        self.db.collection("drone_Status")
    }
}

