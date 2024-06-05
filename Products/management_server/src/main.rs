
use std::{env, fs::File, sync::Mutex};

use actix_web::{middleware::Logger, post, web::{self, Data, Path}, App, HttpResponse, HttpServer, Responder, get, put, delete};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use authentication_sdk::Authenticator;
use dao::{objects::Event, Dao};
use jsonwebtoken::Algorithm;
use mongodb::Client;
use mongodb::bson::doc;
use serde::Deserialize;
use crate::dao::objects::User;

mod event_manager;
mod dao;
mod event_processor;
mod locker;
mod tests;
mod authentication_sdk;
mod utils;

#[derive(Deserialize)]
struct EventQueryData {
    page: Option<u64>,
}

struct AppData {
    // Event id counter to implement id autoincrement.
    counter: Mutex<Option<i32>>,
    mongodb: Dao,
    authenticator: Authenticator,
}



/// Checks if the given user is an authorized user.
///
/// Returns the user id if it is authorized.\
/// Returns an error response status in case of unauthorized access.
async fn is_authorized(auth: &BearerAuth, authenticator: &Authenticator, mongodb: &Dao) -> Result<User, HttpResponse> {
    let token = auth.token();

    let user_id = match authenticator.decode_user_id(token) {
        Ok(id) => id,
        Err(e) => return Err(HttpResponse::Unauthorized().body(format!("Invalid token: {}", e))),
    };

    let authorized = match mongodb.is_authorized(&user_id).await {
        Ok(authz) => authz,
        Err(_) => return Err(HttpResponse::InternalServerError().finish()),
    };

    authorized.ok_or(HttpResponse::Forbidden().finish())
}

// TODO: formalize and document this endpoint
/// Inserts a new event passed as payload.
///
/// Returns 201 in case of successful creation of the resource on the database, in which case the
/// event's ID will be written in the response's body as plain text.\
/// Returns 400 in case on wrong resource representation.\
/// Returns 422 in case of unvalid resource.\
/// Other status codes can be sent according to the HTTP standard.
#[post("/insert_new_event")]
async fn insert_event(data: Data<AppData>, event: web::Json<Event>, auth: BearerAuth) -> impl Responder {
    let user = match is_authorized(&auth, &data.authenticator, &data.mongodb).await {
        Err(res) => return res,
        Ok(user) => user,
    };


    let (result, event_id) = event_manager::insert_new_event(&data, event.into_inner(), &user).await;
    match result {
        Ok(_) => {
            HttpResponse::Created().body(format!("{} {:?}", event_id, std::thread::current().id()))
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Errore - Modifica non eseguita")
        }
    }
}

// TODO: `by_id` is not clear.
/// Ottenimento di tutti gli eventi di competenza dell'utente autorizzato
#[get("/list_events")]
async fn list_events_by_id(data: Data<AppData>, query: web::Query<EventQueryData>, auth: BearerAuth) -> impl Responder {
    let user = match is_authorized(&auth, &data.authenticator, &data.mongodb).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Pagina degli eventi
    let page = &query.page;
    if page.is_some() {
        return HttpResponse::NotImplemented().finish();
    }


    // Ottenimento eventi
    event_manager::list_events_by_id(&data, page, &user).await
}


// Modifica evento
#[put("/modify_event/{event_id}")]
async fn modify_event(data: Data<AppData>, path: Path<u32>, event: web::Json<Event>, auth: BearerAuth) -> impl Responder {
    let user = match is_authorized(&auth, &data.authenticator, &data.mongodb).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Modifica evento
    event_manager::modify_event(&data, path.into_inner(), event.into_inner(), &user).await
}


/// Eliminazione evento
/// Returns 200 if success
/// Returns 404 if the event was not found, with the event id inside the response body to differentiate it from other 404 errors due to malformed URLs
/// Returns 500 if the event could not be deleted (e.g. due to MongoDB problems)
/// Returns 423
/// Returns a JSON with all the events. Basically the same as the "/events" endpoint of the public server, but instead of having partial events, this returns all the details of all the events of competence
#[delete("/delete_event/{event_id}")]
async fn delete_event(data: Data<AppData>, path: Path<u32>, auth: BearerAuth) -> impl Responder {
    let user = match is_authorized(&auth, &data.authenticator, &data.mongodb).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Eliminazione evento
    event_manager::delete_event(&data, path.into_inner(), &user).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    // Connessione al DB e ottenimento collezione degli eventi
    println!("Connecting to MongoDB...");
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017").await.unwrap();
    println!("Connected to MongoDB!");

    let mongodb = client.database("beelive_develop");
    
    #[cfg(debug_assertions)]
    let base_cert_path = "assets";
    #[cfg(not(debug_assertions))]
    let base_cert_path = "/usr/share/beelive_management";

    let cert_path = format!("{}/CA/casdoor.pem", base_cert_path);
    let mut cert = File::open(cert_path).expect("Cannot open the CA pem file");

    let casdoor_client_id = env::var("BEELIVE_CASDOOR_CLIENT_ID")
        .expect("Expected BEELIVE_CASDOOR_CLIENT_ID enviroment variable");

    let authenticator = Authenticator::from_file(Some(Algorithm::RS256), &mut cert, &casdoor_client_id)
        .expect("Cannot create the authenticator");

    // Inserzione nuovo evento
    let data = AppData {
        counter: Mutex::new(None),
        mongodb: mongodb.into(),
        authenticator,
    };
    let data = Data::new(data);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // Add request logging
            .app_data(data.clone())
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
