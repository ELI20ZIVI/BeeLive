
use actix_web::{get, middleware::Logger, web::{self, Data}, App, HttpRequest, HttpServer, Responder};
use dao::objects::Event;
use env_logger::Env;
use mongodb::{Collection, Client};
use serde::Deserialize;

mod endpoints;
mod event_processor;
mod dao;

#[derive(Deserialize)]
struct EventQueryData {
    mode: String,
    addb: Vec<u32>,
    subb: Vec<u32>,
    addi: Vec<u32>,
    subi: Vec<u32>,
}

// TODO: formalize and document this endpoint
#[get("/events")]
async fn get_events(mongodb_events_collection: Data<Collection<Event>>, data: web::Json<EventQueryData>) -> impl Responder {

    // Get API data fields as documented. We will use them in the near future. 
    let _mode = &data.mode;
    let _addi = &data.addi;
    let _subi = &data.subi;
    let _subb = &data.subb;
    let _addb = &data.addb;

    let events = event_processor::get_events(mongodb_events_collection).await;

    web::Json(events)
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
            .service(get_events)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
