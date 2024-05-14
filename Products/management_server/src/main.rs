
use actix_web::{middleware::Logger, post, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use dao::objects::Event;
use mongodb::{Collection, Client};

mod event_manager;
mod dao;
mod event_processor;

// TODO: formalize and document this endpoint
#[post("/insert_new_event")]
async fn insert_event(mongodb_events_collection: Data<Collection<Event>>, event: web::Json<Event>) -> impl Responder {

    let result = event_manager::insert_new_event(mongodb_events_collection, event.into_inner()).await;
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
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
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
