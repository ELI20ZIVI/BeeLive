
use std::cell::Cell;

use actix_web::{middleware::Logger, post, web::{self, Data, Path}, App, HttpResponse, HttpServer, Responder, get, put, delete};
use actix_web_httpauth::extractors::bearer::{self, BearerAuth};
use dao::objects::Event;
use mongodb::{Collection, Client};
use mongodb::bson::doc;
use serde::Deserialize;
use crate::dao::objects::User;

mod event_manager;
mod dao;
mod event_processor;
mod locker;
mod tests;

#[derive(Deserialize)]
struct EventQueryData {
    page: Option<u64>,
}

#[derive(Clone)]
pub struct ManagementServerData {
    counter: Cell<Option<i32>>,
    mongodb_events_collection: Collection<Event>,
    mongodb_users_collection: Collection<User>,
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
async fn insert_event(data: Data<ManagementServerData>, event: web::Json<Event>, auth: BearerAuth) -> impl Responder {

    // ID utente
    let id = auth.token();

    // Ccontrollo esistenza utente nel db
    let user = data.mongodb_users_collection.find_one(doc! { "id": id.clone() }, None).await.unwrap();
    if user.is_none() {
        return HttpResponse::Forbidden().body("User not authorized");
    }

    let (result, event_id) = event_manager::insert_new_event(data, event.into_inner()).await;
    match result {
        Ok(_) => {
            HttpResponse::Created().body("Modifica eseguita con successo")
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Errore - Modifica non eseguita")
        }
    }
}

// Ottenimento di tutti gli eventi di competenza dell'utente autorizzato
#[get("/list_events")]
async fn list_events_by_id(data: Data<ManagementServerData>, query: web::Query<EventQueryData>, auth: BearerAuth) -> impl Responder {

    // Pagina degli eventi
    let page = & query.page;
    if let Some(_) = page {
        return HttpResponse::NotImplemented().finish();
    }

    // ID utente
    let id = auth.token();

    // Ottenimento eventi
    let events = event_manager::list_events_by_id(data, page, id).await;
    
    events
}


// Modifica evento
#[put("/modify_event/{event_id}")]
async fn modify_event(data: Data<ManagementServerData>, path: Path<u32>, event: web::Json<Event>, auth: BearerAuth) -> impl Responder {

    // ID utente -- TODO: Da sostituire con funzione di Pietro
    let id = auth.token();

    // Modifica evento
    event_manager::modify_event(data, path.into_inner(), event.into_inner(), id).await
}


/// Eliminazione evento
/// Returns 200 if success
/// Returns 404 if the event was not found, with the event id inside the response body to differentiate it from other 404 errors due to malformed URLs
/// Returns 500 if the event could not be deleted (e.g. due to MongoDB problems)
/// Returns 423
/// Returns a JSON with all the events. Basically the same as the "/events" endpoint of the public server, but instead of having partial events, this returns all the details of all the events of competence
// Include also the token to know who modify whatq
#[delete("/delete_event/{event_id}")]
async fn delete_event(data: Data<ManagementServerData>, path: Path<u32>, auth: BearerAuth) -> impl Responder {

    let id = auth.token();

    // Eliminazione evento
    event_manager::delete_event(data, path.into_inner(), id).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    // Connessione al DB e ottenimento collezione degli eventi
    println!("Connecting to MongoDB...");
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017").await.unwrap();
    println!("Connected to MongoDB!");
    let mongodb_events_collection = client.database("events").collection::<Event>("events");
    let mongodb_users_collection = client.database("users").collection::<User>("users");

    // Inserzione nuovo evento
    let data = ManagementServerData {
        counter: Cell::new(None),
        mongodb_events_collection,
        mongodb_users_collection,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // Add request logging
            .app_data(Data::new(data.clone()))
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
