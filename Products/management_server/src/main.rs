
use std::sync::Mutex;

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

    // TODO: read from file
    let cert = "-----BEGIN CERTIFICATE-----
MIIE3zCCAsegAwIBAgIDAeJAMA0GCSqGSIb3DQEBCwUAMCkxEDAOBgNVBAoTB2Jl
ZWxpdmUxFTATBgNVBAMMDGNlcnRfYmVlbGl2ZTAeFw0yNDA2MDQyMDU5NDhaFw00
NDA2MDQyMDU5NDhaMCkxEDAOBgNVBAoTB2JlZWxpdmUxFTATBgNVBAMMDGNlcnRf
YmVlbGl2ZTCCAiIwDQYJKoZIhvcNAQEBBQADggIPADCCAgoCggIBANOpmWqQydAd
U2oJcUxAYo3kL+uIoOvkRxudtupZTTN1K5Puc1cdWKhvZ7E/Dow6rCOX+YkGDGpF
vZP0H1GTzJAwj6p+UVHNhiZKp7O6mq6SLoRxjQcLzCw8rsF6TKI9Ij6oc7k2XuR1
6qPvLGuSsqaXLG7Vw0zka4euQGL2OytwM3h/QiU/orE/JlvP4ASnzx0lZgYqCEX1
U85lbUzx+PbAJ4MrrQCtjw9+JA8jOcBnFBlQEFCdasDoNhOTZsJysYW1RH6YDfog
dHF2T9BMdr07aL6G/dzF+eiXF0ugcL/PuqAIvahGAXcDeXnDX1tH/BONkh9GOjcl
dD8Vq+H2ZM30AGmHzMtTrppLIN6+oUSaAoehsBhs2CNlLmXQLyILQXlVYOiTcyM8
w4UbQraJ6DmkcaheKYgjVAMeuIXwFeVxUbAyG9mgqy4pd7RxPxokeBn2qM943p1m
6MsxTkHjtOzCYaS5eIqOcIaP77AaZDbeONWCe13ig7IplnyMHc7elJ7L45LnSs/y
BlvbzEpEuNmpv1WxT2GcUVYSp8kDLmnFb68DRy85roGiFojMlT/dk4b8XrxvaUTS
V7FnIpw90q19kgBs671V/C0uOSGzJB3A9/hdcaj5/h2N3URTxjhpbAWq1uiOmDGJ
v/wYzKgIqQkU71Leh11aFgG1MCy9it3RAgMBAAGjEDAOMAwGA1UdEwEB/wQCMAAw
DQYJKoZIhvcNAQELBQADggIBACibpyuoHjaCgcmRLKFTA9u0HoJG29p6PvjBe1mg
vR2mUN8cwnQxyM5YiiCUyIapyrT3Qq+ynh9fK/6kOuWobvP+TH3WMj3/1l6aPEY6
ZXHS26ksxfl2j4PS1nGo0uXlm+RzP3qCR1sn/ubL8pXMwv6h/0JXfPP6/vLVWzRO
Z1Qno3u7/2DSKXW/kAxUHoiLFTtv+bzZ132nhE8xHSFuSgPRZPLsi2dQMZyuvsnH
Ubven2PdLClcF/J0DLpOJFEyCXpmlLeJW79cM+WgsIzkIS0qsigh9bWxjv+eZt1O
o2hWrH+u4kS6oVoL5Nw7Y5+kMCqKn90ne7cvMVaChm5ofvcYVwXe+Np7peJwxyvk
GxYSxZ3y+CwoaGMyeABMIha7GanE6mqgkI4JG/EXE0K3RXvLWOeQ1NOoWOQhb4Ej
24QsIdeUuV/OhQwAd/ftnVJxuQLtPntfznqYm98UFX9hb9AhdeyoCnGY6NdW3+V9
TjJQKkUWz4ww6vW7vdyqU+E5HrV/vVZ/4DurGobKbkwKJai91/vRIKZj2ZL9839Q
UWdS4gQVX0QTPQy0o4wHAVB6aw4rzAT1Vi3gscKHSfI28lxIYGDOAF3MNzbaQRio
jqAgo3CTNvU9uVhYrQJkcvLM7gpbYxdQ8KMjwUCS+F0tb7+Xr6piwo8PJmZHWGM/
FS6o
-----END CERTIFICATE-----";

    let authenticator = Authenticator::new(Some(Algorithm::RS256), cert, "712b8aaffd9c4c71ab7a")
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
