use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dao::objects::Event;
use mongodb::*;

mod dao;
mod endpoints;

#[get("/events")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let mongodb_events_collection = client.database("events").collection::<Event>("events");


    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
