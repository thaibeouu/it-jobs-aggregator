use rocket::response::content;
use select::document::Document;
use select::predicate::Attr;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobRequest {
    pub title: String,
    pub company: String,
}

#[get("/scrape")]
pub async fn scrape() -> content::Json<&'static str> {
    let mut handles: std::vec::Vec<_> = Vec::new();
    for i in 0..2 {
        let job = tokio::spawn(async move { scrape_remoteok(&i).await });
        handles.push(job);
    }

    let mut results = Vec::new();
    for job in handles {
        results.push(job.await);
    }
    let fl: Vec<JobRequest> = results
        .into_iter()
        .map(|x| -> Result<Vec<JobRequest>, Box<_>> { x.unwrap() })
        .map(|x| x.unwrap())
        .flatten()
        .collect::<Vec<JobRequest>>();
    println!("{:?}", fl);
    content::Json("{\"success\":\"true\"}")
}

async fn scrape_remoteok(
    i: &i32,
) -> Result<Vec<JobRequest>, Box<dyn std::error::Error + Send + Sync>> {
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
    let titles: Vec<String> = document
        .find(Attr("itemprop", "title"))
        .map(|x| x.first_child().unwrap().text())
        .collect();
    let companies: Vec<String> = document
        .find(Attr("itemprop", "name"))
        .map(|x| x.first_child().unwrap().text())
        .collect();
    for i in 0..titles.len() {
        let job = JobRequest {
            title: titles.get(i).unwrap().to_string(),
            company: companies.get(i).unwrap().to_string(),
        };
        results.push(job)
    }
    return Ok(results);
}

// async fn scrape_indeed(i: &i32) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
//     let url = format!("https://vn.indeed.com/jobs?q=software&start={}0", i);
//     let response = reqwest::get(&url).await?.text().await?;
//     let document = Document::from(response.as_str());

//     let mut results = Vec::new();
//     for node in document.find(Class("jobsearch-SerpJobCard")) {
//         let title: String = node
//             .find(Class("jobtitle"))
//             .next()
//             .unwrap()
//             .text()
//             .trim_end()
//             .to_string()
//             .replace("\n", "");
//         let company: String = node
//             .find(Class("company"))
//             .next()
//             .unwrap()
//             .text()
//             .trim_end()
//             .to_string()
//             .replace("\n", "");
//         results.push(format!("{} with price of {}", title, company))
//     }
//     println!("{}", url);
//     return Ok(results);
// }
