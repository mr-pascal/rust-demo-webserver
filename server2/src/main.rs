use actix_web::{get, App, HttpServer, Responder};

#[get("/hello")]
async fn greet() -> impl Responder {
    println!("Got request!");
    return format!("Hello from server TWOOOOO!");
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = option_env!("PORT").unwrap_or("8080").parse().unwrap();
    let host = option_env!("HOST").unwrap_or("127.0.0.1");
    let startup_msg = format!("Server2: Starting on {host}:{port}");

    println!("{}", startup_msg);

    HttpServer::new(|| App::new().service(greet))
        .bind((host, port))?
        .run()
        .await
}
