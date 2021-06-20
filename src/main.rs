#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
use rocket::fs::{FileServer, relative};

mod manual {
    use std::path::{PathBuf, Path};
    use rocket::fs::NamedFile;

    #[rocket::get("/second/<path..>")]
    pub async fn second(path: PathBuf) -> Option<NamedFile> {
        let mut path = Path::new(super::relative!("client/public")).join(path);
        if path.is_dir() {
            path.push("index.html");
        }

        NamedFile::open(path).await.ok()
    }
}

mod db;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllowedOrigins::some_exact(&[
            "http://localhost:5000",
            "http://127.0.0.1:5000",
        ]),
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .attach(db::stage())
        .attach(cors)
        .mount("/", routes![manual::second])
        .mount("/", FileServer::from(relative!("client/public")))
        .launch()
        .await?;

    Ok(())
}
