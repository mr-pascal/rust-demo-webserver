use actix_web::{get, App, HttpServer, Responder};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}

async fn make_request() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let target_host = option_env!("TARGET").unwrap_or("http://0.0.0.0:8081");
    let target_url = format!("{}/hello", target_host);

    println!("Sending request to '{}'!", target_url);
    let resp200 = client
        // .get("https://httpbin.org/ip")
        .get(target_url)
        .header(CONTENT_TYPE, "text/plain")
        .send()
        .await?
        .text()
        .await?;

    let val = resp200;
    Ok(val)
}

#[get("/hello")]
async fn greet() -> impl Responder {
    let answer_from_server = make_request().await;
    let mut message = String::from("Oops.. this didn't worked...");
    match answer_from_server {
        Ok(a) => {
            message = a;
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

    // .unwrap_or(String::from("HELL BROKE LOSE!"));

    return format!("{message}!");
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = option_env!("PORT").unwrap_or("8080").parse().unwrap();
    let host = option_env!("HOST").unwrap_or("0.0.0.0");
    let startup_msg = format!("Server1: Starting on {host}:{port}");

    println!("{}", startup_msg);

    HttpServer::new(|| App::new().service(greet))
        .bind((host, port))?
        .run()
        .await
}
