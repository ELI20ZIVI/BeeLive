
use actix_web::{middleware::Logger, web::{self, Data}, App, HttpServer, Responder, post};
use dao::objects::Event;
use mongodb::{Collection, Client};
use serde::Deserialize;

mod event_manager;
mod dao;
mod event_processor;

#[derive(Deserialize)]
struct EventQueryData {
    mode: String,
    addb: Vec<u32>,
    subb: Vec<u32>,
    addi: Vec<u32>,
    subi: Vec<u32>,
}

// TODO: formalize and document this endpoint
#[post("/insert_new_event")]
async fn get_event(mongodb_events_collection: Data<Collection<Event>>, path: web::Path<u32>) -> impl Responder {
   
    let event_id = path.into_inner();
    //let event = event_manager::get_event(mongodb_events_collection, event_id).await;

    web::Json(Event::test_event())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {



    env_logger::init();

    //TODO: solve unwrap
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let mongodb_events_collection = client.database("events").collection::<Event>("events");

    dao::insert_new_event(&mongodb_events_collection, Event::test_event()).await.unwrap();
 
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(mongodb_events_collection.clone()))
            .service(
                web::scope("/api/v3")
                //.service(get_events)
                .service(get_event)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
