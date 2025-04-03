use actix_web::{web, App, HttpResponse, HttpServer};
use dotenvy::{self, dotenv};
use env_logger;

mod event_consumers;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    // Load the .env values in the enviorment
    dotenv().ok();
    
    // Init the event_logger 
    env_logger::init();

    actix_web::rt::spawn(async move { event_consumers::drone_status_consumer::connect_and_consume().await });

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
