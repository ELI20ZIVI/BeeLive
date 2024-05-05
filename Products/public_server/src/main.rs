
use futures::stream::StreamExt;
use actix_web::{get, web::{self, Data}, App,  HttpServer, Responder};
use dao::objects::Event;
use dao::*;
use mongodb::{Collection, Client};

mod dao;
mod endpoints;


// TODO: formalize and document this endpoint
#[get("/events")]
async fn get_events(mongodb_events_collection: Data<Collection<Event>>) -> impl Responder {
 
    let events = query_events(mongodb_events_collection).await;

    web::Json(events)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    //TODO: solve unwrap
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let mongodb_events_collection = client.database("events").collection::<Event>("events");
 
    HttpServer::new(move || {

        App::new()
            .app_data(Data::new(mongodb_events_collection.clone()))
            .service(get_events)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
