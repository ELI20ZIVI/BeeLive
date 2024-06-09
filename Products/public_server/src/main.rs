
use actix_web::{get, middleware::Logger, web::{self, Data, Path}, App, HttpServer, Responder, HttpResponse};
use dao::objects::Event;
use mongodb::{Collection, Client};
use serde::Deserialize;
use crate::dao::objects::Category;

mod event_processor;
mod dao;

#[derive(Deserialize)]
struct EventQueryData {
    mode: Option<String>,
    addb: Option<String>,
    subb: Option<String>,
    addi: Option<String>,
    subi: Option<String>,
}

// TODO: formalize and document this endpoint
#[get("/events/{event_id}")]
pub async fn get_event(mongodb_events_collection: Data<Collection<Event>>, path: Path<u32>) -> impl Responder {

    let event_id = path.into_inner();
    let event = event_processor::get_event(mongodb_events_collection, event_id).await;

    if let Some(event) = event {
        HttpResponse::Ok().json(event)
    } else {
        HttpResponse::NotFound().finish()
    }
}

// TODO: formalize and document this endpoint
#[get("/events")]
pub async fn get_events(
    mongodb_events_collection: web::Data<Collection<Event>>,
    mongodb_categories_collection: web::Data<Collection<Category>>,
    data: web::Query<EventQueryData>
) -> impl Responder {

    // Esegui la query per ottenere gli eventi
    let events_result = event_processor::get_events(mongodb_events_collection.clone(), mongodb_categories_collection.clone(), data).await;

    // Ritorna il risultato della query
    events_result
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    //TODO: solve unwrap
    println!("Connecting to MongoDB...");
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017/").await.unwrap();
    //let client = Client::with_uri_str("mongodb://@beelive.mongo:27017").await.unwrap();
    println!("Connected to MongoDB!");
    let mongodb_events_collection = client.database("beelive_test").collection::<Event>("events");
    let mongodb_categories_collection = client.database("beelive_test").collection::<Category>("categories");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(mongodb_events_collection.clone()))
            .app_data(Data::new(mongodb_categories_collection.clone()))
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


#[cfg(test)]
mod tests;
