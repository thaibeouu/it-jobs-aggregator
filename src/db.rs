use mongodb::{options::ClientOptions, Client};

#[derive(Clone, Debug)]
pub struct Database {
    pub client: Client,
}

impl Database {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("booky".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }
}
