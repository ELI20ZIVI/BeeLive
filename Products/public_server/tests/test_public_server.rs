use actix_web::{test, App};
// Improt librerie get_events e get_event
use crate::get_events;
use crate::get_event;

//use public_server::get_events;
//use public_server::get_event;

#[actix_web::test]
async fn test_get_events() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/api/v3/events").route(web::get().to(get_events)))
    )
        .await;

    // Tests
}

#[actix_web::test]
async fn test_get_event() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/api/v3/event/{id}").route(web::get().to(get_event)))
    )
        .await;

    // Tests
}
