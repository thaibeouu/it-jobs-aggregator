use self::diesel::prelude::*;
use itertools::Itertools;
use rocket::fairing::AdHoc;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel};
use select::document::Document;
use select::predicate::Attr;
use std::time::SystemTime;

#[database("diesel")]
struct Db(diesel::SqliteConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "posts"]
struct Post {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    title: String,
    text: String,
    url: String,
    #[serde(skip_deserializing)]
    published: bool,
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        url -> Text,
        text -> Text,
        published -> Bool,
    }
}

#[get("/scrape")]
async fn scrape(db: Db) -> Result<Json<bool>> {
    let mut handles: std::vec::Vec<_> = Vec::new();
    for i in 0..8 {
        let job = tokio::spawn(async move { scrape_remoteok(&i).await });
        handles.push(job);
    }

    let mut results = Vec::new();
    for job in handles {
        results.push(job.await);
    }
    let flattened_results: Vec<Post> = results
        .into_iter()
        .map(|x| -> Result<Vec<Post>, Box<_>> { x? })
        .map(|x| x.unwrap())
        .flatten()
        .unique_by(|p| format!("{}{}", p.title, p.text))
        .collect();
    db.run(move |conn| diesel::delete(posts::table).execute(conn))
        .await?;
    db.run(move |conn| {
        diesel::insert_into(posts::table)
            .values(&flattened_results)
            .execute(conn)
    })
    .await?;
    Ok(Json(true))
}

async fn scrape_remoteok(i: &i32) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>> {
    let base_url = "https://remoteok.io";
    let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let url = format!(
        "{}/?pagination={}&worldwide=true",
        base_url,
        duration.as_secs() as i32 - (60000 * i)
    );
    let response = reqwest::get(&url).await?.text().await?;
    let document = Document::from(response.as_str());

    let mut results = Vec::new();
    let titles: Vec<String> = document
        .find(Attr("itemprop", "title"))
        .map(|x| x.first_child().unwrap().text())
        .collect();
    let urls: Vec<String> = document
        .find(Attr("itemprop", "title"))
        .map(|x| x.parent().unwrap().attr("href").unwrap().to_string())
        .collect();
    let companies: Vec<String> = document
        .find(Attr("itemprop", "name"))
        .map(|x| x.first_child().unwrap().text())
        .collect();
    for i in 0..titles.len() {
        let job = Post {
            title: titles.get(i).unwrap().to_string(),
            text: companies.get(i).unwrap().to_string(),
            published: true,
            url: format!("{}{}", base_url, urls.get(i).unwrap().to_string()),
            id: None,
        };
        results.push(job)
    }
    return Ok(results);
}

#[post("/", data = "<post>")]
async fn create(db: Db, post: Json<Post>) -> Result<Created<Json<Post>>> {
    let post_value = post.clone();
    db.run(move |conn| {
        diesel::insert_into(posts::table)
            .values(&post_value)
            .execute(conn)
    })
    .await?;

    Ok(Created::new("/").body(post))
}

#[get("/")]
async fn list(db: Db) -> Result<Json<Vec<Post>>> {
    let ids: Vec<Post> = db.run(move |conn| posts::table.load(conn)).await?;

    Ok(Json(ids))
}

#[get("/<id>")]
async fn read(db: Db, id: i32) -> Option<Json<Post>> {
    db.run(move |conn| posts::table.filter(posts::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[delete("/<id>")]
async fn delete(db: Db, id: i32) -> Result<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::delete(posts::table)
                .filter(posts::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}

#[delete("/")]
async fn destroy(db: Db) -> Result<()> {
    db.run(move |conn| diesel::delete(posts::table).execute(conn))
        .await?;

    Ok(())
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("db/migrations");
    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");
    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount("/api", routes![list, read, create, delete, destroy, scrape])
    })
}
