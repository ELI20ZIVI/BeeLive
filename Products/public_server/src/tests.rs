use actix_web::{dev::{Service, ServiceResponse}, test, web, App, Error};
use actix_http::Request;

use super::*;
    
    // Import librerie get_events e get_event
use crate::{get_events, get_event};

#[actix_web::test]
async fn test_get_events() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/api/v3/events").route(web::get().to(get_events)))
    ).await;

    // Tests
}
    
#[actix_web::test]
async fn test_get_event() {
   let mut app = test::init_service(
       App::new()
           .service(web::resource("/api/v3/event/{id}").route(web::get().to(get_event)))
   ).await;

   // Tests
}









async fn create_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    //TODO: solve unwrap
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017/").await
        .expect("Cannot create a client for the database");

    let database = client.database("beelive_test");

    let mongodb_events_collection = Data::new(database.collection::<Event>("events"));
    let mongodb_categories_collection = Data::new(database.collection::<Category>("categories"));

    database.drop(None).await.expect("Cannot drop the database");

    test::init_service(
        App::new()
            .app_data(mongodb_events_collection)
            .app_data(mongodb_categories_collection)
            .service(
                web::scope("/api/v3")
                    .service(get_events)
                    .service(get_event)
            )
    ).await
}
