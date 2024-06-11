use actix_web::{test, App, web};

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









fn create_app() {
    let client = Client::with_uri_str("mongodb://beelive.mongo:27017/").await.unwrap();
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
