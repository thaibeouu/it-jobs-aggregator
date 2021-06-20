#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
use rocket::fs::FileServer;

mod db;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .attach(db::stage())
        .attach(cors)
        .mount("/", FileServer::from("client/public"))
        .launch()
        .await?;

    Ok(())
}
