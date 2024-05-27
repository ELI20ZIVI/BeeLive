
use std::cell::Cell;

use actix_web::{middleware::Logger, post, web::{self, Data, Path}, App, HttpResponse, HttpServer, Responder, get};
use dao::objects::Event;
use mongodb::{Collection, Client};
use crate::dao::objects::User;

mod event_manager;
mod dao;
mod event_processor;

#[derive(Clone)]
struct InsertNewEventEndpointData {
    counter: Cell<Option<i32>>,
    mongodb_events_collection: Collection<Event>,
}

// TODO: formalize and document this endpoint
/// Inserts a new event passed as payload.
///
/// Returns 201 in case of successful creation of the resource on the database, in which case the
/// event's ID will be written in the response's body as plain text.
/// Returns 400 in case on wrong resource representation.
/// Returns 422 in case of unvalid resource.
/// Other status codes can be sent according to the HTTP standard.
#[post("/insert_new_event")]
async fn insert_event(data: Data<InsertNewEventEndpointData>, event: web::Json<Event>) -> impl Responder {

    let (result, event_id) = event_manager::insert_new_event(data, event.into_inner()).await;
    match result {
        Ok(_) => {
            HttpResponse::Created().body(event_id.to_string())
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Ottenimento di tutti gli eventi di competenza dell'utente autorizzato
#[get("/list_events/{user_id}")]
async fn list_events_by_id(
    mongodb_events_collection: Data<Collection<Event>>,
    mongodb_id_collection: Data<Collection<User>>,
    path: Path<u32>) -> impl Responder {
    
    let events = event_manager::list_events_by_id(mongodb_events_collection.clone(), mongodb_id_collection.clone(), path.into_inner()).await;
    
    events
}

// Modifica evento
#[post("/modify_event/{event_id}")]
async fn modify_event(mongodb_events_collection: Data<Collection<Event>>, path: Path<u32>) -> impl Responder {

}

/// Eliminazione evento
/// Returns 200 if success
/// Returns 404 if the event was not found, with the event id inside the response body to differentiate it from other 404 errors due to malformed URLs
/// Returns 500 if the event could not be deleted (e.g. due to MongoDB problems)
/// Returns 423
/// Returns a JSON with all the events. Basically the same as the "/events" endpoint of the public server, but instead of having partial events, this returns all the details of all the events of competence
// Include also the token to know who modify what
#[post("/delete_event/{event_id}")]
async fn delete_event(mongodb_events_collection: Data<Collection<Event>>, path: Path<u32>) -> impl Responder {

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {



    env_logger::init();

    // Connessione al DB e ottenimento collezione degli eventi
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017").await.unwrap();
    let mongodb_events_collection = client.database("events").collection::<Event>("events");
    let mongodb_id_collection = client.database("users").collection::<User>("users");

    // Inserzione nuovo evento
    let insert_new_event_endpoint_data = InsertNewEventEndpointData {
        counter: Cell::new(None),
        mongodb_events_collection,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // Add request logging
            .app_data(Data::new(insert_new_event_endpoint_data.clone()))  // Share event data
            .app_data(Data::new(mongodb_events_collection.clone()))  // Share event collection
            .app_data(Data::new(mongodb_id_collection.clone()))  // Share ID collection (assuming it exists)
            .service(  // Define API endpoints under "/api/v3" scope
                       web::scope("/api/v3")
                           // Add your handler functions here
                           .service(insert_event)
                           .service(list_events_by_id)
                           .service(modify_event)
                           .service(delete_event)
            )
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

/*
Tre endpoint nuovi:
 - lista di tutti gli eventi di competenza dell'utente autorizzato (Per ora lista di tutti gli eventi e basta)
 - Modifica evento
 - eliminazione evento
*/