#[macro_use]
extern crate rocket;

use rocket::response::content;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Job {
    pub title: String,
    pub company: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn scrape_remoteok(i: &i32) -> Result<Vec<Job>, Box<dyn std::error::Error + Send + Sync>> {
    let base_url = "https://remoteok.io";
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let url = format!(
        "{}/?pagination={}&worldwide=true",
        base_url,
        duration.as_secs() as i32 - (36000 * i)
    );
    let response = reqwest::get(&url).await?.text().await?;
    let document = Document::from(response.as_str());

    let mut results = Vec::new();
    let mut titles = Vec::new();
    let mut companies = Vec::new();
    for node in document.find(Attr("itemprop", "title")) {
        println!("{:?}", node);
        let title: String = node
            .first_child()
            .unwrap()
            .text()
            .trim_end()
            .to_string()
            .replace("\n", "");
        titles.push(title);
    }
    for node in document.find(Attr("itemprop", "name")) {
        let company: String = node
            .first_child()
            .unwrap()
            .text()
            .trim_end()
            .to_string()
            .replace("\n", "");
        companies.push(company);
    }
    for i in 0..titles.len() {
        let obj = Job {
            title: titles.get(i).unwrap().to_string(),
            company: companies.get(i).unwrap().to_string(),
        };
        results.push(obj)
    }
    return Ok(results);
}

async fn scrape_indeed(i: &i32) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://vn.indeed.com/jobs?q=software&start={}0", i);
    let response = reqwest::get(&url).await?.text().await?;
    let document = Document::from(response.as_str());

    let mut results = Vec::new();
    for node in document.find(Class("jobsearch-SerpJobCard")) {
        let title: String = node
            .find(Class("jobtitle"))
            .next()
            .unwrap()
            .text()
            .trim_end()
            .to_string()
            .replace("\n", "");
        let company: String = node
            .find(Class("company"))
            .next()
            .unwrap()
            .text()
            .trim_end()
            .to_string()
            .replace("\n", "");
        results.push(format!("{} with price of {}", title, company))
    }
    println!("{}", url);
    return Ok(results);
}

#[get("/json")]
async fn json() -> content::Json<&'static str> {
    let mut handles: std::vec::Vec<_> = Vec::new();
    for i in 0..10 {
        let job = tokio::spawn(async move { scrape_remoteok(&i).await });
        handles.push(job);
    }

    let mut results = Vec::new();
    for job in handles {
        results.push(job.await);
    }
    println!("{:?}", results);
    content::Json("{\"success\":\"true\"}")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![json])
        .mount("/agg", routes![index])
}
