
use actix_web::{get, middleware::Logger, web::{self, Data, Path}, App, HttpRequest, HttpServer, Responder};
use dao::objects::Event;
use env_logger::Env;
use mongodb::{Collection, Client};
use serde::Deserialize;

mod event_processor;
mod dao;

/// Event query data for /events.
#[derive(Deserialize)]
struct EventQueryData {
    mode: Option<String>,
    addb: Option<Vec<u32>>,
    subb: Option<Vec<u32>>,
    addi: Option<Vec<u32>>,
    subi: Option<Vec<u32>>,
}

// TODO: formalize and document this endpoint
/// Access the detail of a specific event by [event_id].
///
/// Returns status code 200 in case of success.
/// Can return status 400 in case of invalid type for [event_id].
/// Can return status 404 in case of inexistent resource.
/// Other error codes can be thrown according to the HTTP standard.
#[get("/events/{event_id}")]
async fn get_event(mongodb_events_collection: Data<Collection<Event>>, path: Path<u32>) -> impl Responder {
   
    let event_id = path.into_inner();
    let event = event_processor::get_event(mongodb_events_collection, event_id).await;

    web::Json(event)
}

// TODO: formalize and document this endpoint
/// Requests the list of events filtered according to the [data] filters.
///
/// Returns status code 200 in case of success.
/// Can return status 400 in case of invalid query paramentes.
/// Can return status 422 in case of inexistent filters.
#[get("/events")]
async fn get_events(mongodb_events_collection: Data<Collection<Event>>, data: web::Query<EventQueryData>) -> impl Responder {

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

    // TODO: solve unwrap
    // TODO: hide credentials.
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017").await.unwrap();
    //let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    
    // Gets the events collection from the database.
    // TODO: migrate to the beelive develop database.
    let mongodb_events_collection = client.database("events").collection::<Event>("events");

 
    HttpServer::new(move || {

        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(mongodb_events_collection.clone()))
            .service(
                web::scope("/api/v3")
                .service(get_events)
                .service(get_event)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
