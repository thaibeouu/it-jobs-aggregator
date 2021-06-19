#[macro_use]
extern crate rocket;

mod db;
mod api;

#[rocket::main]
async fn main() {
    let client: mongodb::Client = db::Database::init().await.unwrap().client;
    // client
    //     .database("admin")
    //     .run_command(doc! {"ping": 1}, None)
    //     .await;
    println!("Connected successfully.");
    rocket::build()
        .mount("/", routes![api::scrape])
        .launch()
        .await;
}
