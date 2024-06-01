
use std::cell::Cell;

use actix_web::{middleware::Logger, post, web::{self, Data, Path}, App, HttpResponse, HttpServer, Responder, get, put, delete};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use authentication_sdk::Authenticator;
use dao::{objects::Event, MongodbExtension};
use mongodb::{Collection, Client, Database};
use mongodb::bson::doc;
use serde::Deserialize;
use crate::dao::objects::User;

mod event_manager;
mod dao;
mod event_processor;
mod locker;
mod tests;
mod authentication_sdk;

#[derive(Deserialize)]
struct EventQueryData {
    page: Option<u64>,
}

#[derive(Clone)]
struct AppData {
    counter: Cell<Option<i32>>,
    mongodb: Database,
    authenticator: Authenticator,
}


/// Checks if the given user is an authorized user.
///
/// Returns the user id if it is authorized.\
/// Returns an error response status in case of unauthorized access.
async fn is_authorized(auth: &BearerAuth, authenticator: &Authenticator, mongodb: &Database) -> Result<String, HttpResponse> {
    let token = auth.token();

    let user_id = match authenticator.decode_user_id(token) {
        Ok(id) => id,
        Err(_) => return Err(HttpResponse::Unauthorized().body("Invalid token")),
    };

    let authorized = match mongodb.is_authorized(&user_id).await {
        Ok(authz) => authz,
        Err(_) => return Err(HttpResponse::InternalServerError().finish()),
    };

    if authorized {
        Ok(user_id)
    } else {
        Err(HttpResponse::Forbidden().finish())
    }
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
    if let Err(res) = is_authorized(&auth, &data.authenticator, &data.mongodb).await {
        return res;
    }


    let (result, event_id) = event_manager::insert_new_event(&data, event.into_inner()).await;
    match result {
        Ok(_) => {
            HttpResponse::Created().body("Modifica eseguita con successo")
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
    let user_id = match is_authorized(&auth, &data.authenticator, &data.mongodb).await {
        Err(res) => return res,
        Ok(id) => id,
    };

    // Pagina degli eventi
    let page = &query.page;
    if page.is_some() {
        return HttpResponse::NotImplemented().finish();
    }


    // Ottenimento eventi
    event_manager::list_events_by_id(&data, page, &user_id).await
}


// Modifica evento
#[put("/modify_event/{event_id}")]
async fn modify_event(data: Data<AppData>, path: Path<u32>, event: web::Json<Event>, auth: BearerAuth) -> impl Responder {
    let user_id = match is_authorized(&auth, &data.authenticator, &data.mongodb).await {
        Err(res) => return res,
        Ok(id) => id,
    };

    // Modifica evento
    event_manager::modify_event(&data, path.into_inner(), event.into_inner(), &user_id).await
}


/// Eliminazione evento
/// Returns 200 if success
/// Returns 404 if the event was not found, with the event id inside the response body to differentiate it from other 404 errors due to malformed URLs
/// Returns 500 if the event could not be deleted (e.g. due to MongoDB problems)
/// Returns 423
/// Returns a JSON with all the events. Basically the same as the "/events" endpoint of the public server, but instead of having partial events, this returns all the details of all the events of competence
#[delete("/delete_event/{event_id}")]
async fn delete_event(data: Data<AppData>, path: Path<u32>, auth: BearerAuth) -> impl Responder {
    let user_id = match is_authorized(&auth, &data.authenticator, &data.mongodb).await {
        Err(res) => return res,
        Ok(id) => id,
    };

    // Eliminazione evento
    event_manager::delete_event(&data, path.into_inner(), &user_id).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    // Connessione al DB e ottenimento collezione degli eventi
    println!("Connecting to MongoDB...");
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017").await.unwrap();
    println!("Connected to MongoDB!");

    let mongodb = client.database("beelive_develop");
    let mongodb_events_collection = mongodb.collection::<Event>("events");
    let mongodb_users_collection = mongodb.collection::<User>("users");

    // TODO: read from file
    let cert = "-----BEGIN CERTIFICATE-----
MIIFRzCCAvugAwIBAgIDAeJAMEEGCSqGSIb3DQEBCjA0oA8wDQYJYIZIAWUDBAID
BQChHDAaBgkqhkiG9w0BAQgwDQYJYIZIAWUDBAIDBQCiAwIBQDApMRAwDgYDVQQK
EwdiZWVsaXZlMRUwEwYDVQQDDAxjZXJ0X2JlZWxpdmUwHhcNMjQwNTI4MTY1MDUy
WhcNNDQwNTI4MTY1MDUyWjApMRAwDgYDVQQKEwdiZWVsaXZlMRUwEwYDVQQDDAxj
ZXJ0X2JlZWxpdmUwggIiMA0GCSqGSIb3DQEBAQUAA4ICDwAwggIKAoICAQDQ7t10
2ld+ZdH2ytL0TFLCh2cQZeJGD2UJH2C5CXcd6uJLINC+oSIbEZXUiT6DbbGVUOZ5
GEqSEXuQ0o0v1dzqCyrEhmXzKL94gIcR+j7NJ0JfObIiC+ggVjbeTE12NxtebFRR
m7WfHGX/ArvcVWL3BvGBHGFiZnhiyfCl38yeed6wSMvDni6vzCphzuykRSmsKYKO
ARzWrF9dnk2r4QDDJW/3xrA++iTmz85W4cPAp0UqxgUlx03R4MSJVXba0xMpBn8v
ed0ysvKFmr8jhBgTOjnLaMb1A2rpvorZO6aZe9xc9tQ/F3tozhz9Iqoa0Os8hCuu
iqFUmiQpeDW//fN58ACjlSUTL0vc2yglzxwJ01q9g+RcnhTzY/0Df/KYazYVUDB1
6+WzaRWCW0v6RlthsYLTgij0Nnb3UNjp5z2RojUTd8JQ4Af7icti70wWN+mxMuXT
YY24fyYVTdsU1AJSUIT0I4CKEvtb1Bvaa120Qe6f9/52hJAgzJ+znvvs+8zETdbM
vEAXwbH/9magS/4rzUexzvFbVrtW7BljuAzAJM1urqPOvwZzjHGACIj3bHDDYlp7
v1ExN+z73nhoUh1g083WCM7Dhp4BV+g/TmHpDRng+PbMZETEyVJSENshjvphVFUb
zQmyfniER+KqLH0qfIOg5pMquJqZiT3Oh8fAdwIDAQABoxAwDjAMBgNVHRMBAf8E
AjAAMEEGCSqGSIb3DQEBCjA0oA8wDQYJYIZIAWUDBAIDBQChHDAaBgkqhkiG9w0B
AQgwDQYJYIZIAWUDBAIDBQCiAwIBQAOCAgEAMfXT7sNCHLM1uhQ/NwZFTU56Zlq0
SH2QxYKMZoRKMcsWvJrUbDYIRO2zGyTllf0LZ/B5f+BfJBOM3dOwaZyPNF0NWzzH
D9dO3+qJaM7HrUJ+WDAcGKMEGYjunfx/YtXbyPqeRb8NnQOyYXePK8DaZbTxepEm
rmSNAcqkyf4ED46bNaJs1CiBsdulTpEalvrYGc9oBLlKWAQ+TXvVFBadG7B3SJ1z
I/hIylrziBUL2Rn5cqVa7RwVylREY6owI1HOl2DR4pXcRL2ZAaZo8kjejbgwlzr2
KHUfXXrUeDFGZGEno37kT3r+s7cEvL+RpwDx9lSN/tmFb3av27UUbEAq6T2mUGkP
qZlCt5oZlj0uve8hWmMg4e+ez12WqwZXj3WGY1WbczIGcWxu0mH1EJtTQ/038UPM
UDxJQN+kJ1qeb9Svsly8Wf4fWs8k9bwDFPBfQGQkCLFM4IyJ6vJGbozSpkput2dg
jvS6+YyeGKEUV4OFDJW3Tck5eR+II2rf2W182Px3qOV0FOLJ2irmgrOj8FHU3lxy
XKmmI12Qo0HcYjKo+ie0B+L2VncEsBvLIpJa9NTtgVZqq1A8EgBE0PiA8F7OkCiA
VT0it8czQ3KFY34YeInrAe+5/T3wlPnk4Ef1nQ0/J5M306hPnxcq3zh5q6JfvyjN
2K68VcaCJIywVuw=
-----END CERTIFICATE-----
";
    let authenticator = Authenticator::new(None, cert).expect("Cannot create the authenticator");

    // Inserzione nuovo evento
    let data = AppData {
        counter: Cell::new(None),
        mongodb,
        authenticator,
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
