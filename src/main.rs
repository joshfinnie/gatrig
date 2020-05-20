extern crate hyper;
extern crate tokio;

use dotenv::var;
use hyper::{header, Client, Method, Request};
use hyper_tls::HttpsConnector;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
    repo: String,
}

static POST_DATA: &str = r#"{"event_type": "rerun-workers"}"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!{"Starting event...\n"}
    let args = Cli::from_args();

    let token = var("GITHUB_AUTH_TOKEN").unwrap();
    let auth_token = format!("token {}", token);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = format!("https://api.github.com/repos/{}/dispatches", &args.repo);
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(header::ACCEPT, "application/vnd.github.everest-preview+json")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::USER_AGENT, "joshfinnie")
        .header(header::AUTHORIZATION, auth_token)
        .body(POST_DATA.into())
        .unwrap();
    let res = client.request(req).await?;

    if 204 == res.status() {
        println!("Successfully triggered Github Action.")
    } else {
        println!("Something went wrong...")
    }
    Ok(())
}
