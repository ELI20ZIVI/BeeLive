
use std::cell::Cell;

use actix_web::{middleware::Logger, post, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use dao::objects::Event;
use mongodb::{Collection, Client};

mod event_manager;
mod dao;
mod event_processor;

struct InsertNewEventEndpointData {
    counter: Cell<Option<i32>>,
    mongodb_events_collection: Collection<Event>,
}

// TODO: formalize and document this endpoint
/// Inserts a new event passed as payload.
///
/// Returns 201 in case of successful creation of the resource on the database.
/// Returns 400 in case on wrong resource representation.
/// Returns 422 in case of unvalid resource.
/// Other status codes can be sent according to the HTTP standard.
#[post("/insert_new_event")]
async fn insert_event(data: Data<InsertNewEventEndpointData>, event: web::Json<Event>) -> impl Responder {

    let result = event_manager::insert_new_event(data, event.into_inner()).await;
    match result {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {



    env_logger::init();

    //TODO: solve unwrap
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017").await.unwrap();
    let mongodb_events_collection = client.database("events").collection::<Event>("events");

 
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(mongodb_events_collection.clone()))
            .service(
                web::scope("/api/v3")
                //.service(get_events)
                .service(insert_event)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
