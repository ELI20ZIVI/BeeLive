use std::iter;

use actix_web::{dev::{Service, ServiceResponse}, http, test, web::{self, Data}, App, Error};
use actix_http::Request;
use chrono::DateTime;
use mongodb::{Client, Database};
use rand::random;

// Import librerie get_events e get_event
use crate::{dao::objects::{Category, Event, NullableDateTimeRange, PrunedEvent, RiskLevel, SubEvent}, get_event, get_events};

#[actix_web::test]
async fn test_get_events_empty() {
    let app = create_app().await;

    let request = test::TestRequest::get()
        .uri("/api/v3/events").to_request();

    let response = test::call_service(&app, request).await;

    // Checks status code 200.
    assert_eq!(
        response.status(), http::StatusCode::OK, 
        "Could't fetch the event list: {:?} {:?}", response.status(), response.response().body()
    );
}

#[actix_web::test]
async fn test_get_events() {
    let app = create_app().await;
    let db = create_db().await;
    let events = db.collection::<Event>("events");

    let request = test::TestRequest::get()
        .uri("/api/v3/events").to_request();

    const N : usize = 10;

    let generated_events = (0..N).map(|i| random_event(i.try_into().unwrap(), None));
    events.insert_many(generated_events, None).await.expect("Cannot insert events");

    let response: Vec<PrunedEvent> = test::call_and_read_body_json(&app, request).await;

    assert_eq!(response.len(), N, "Fetched {}/{} events", response.len(), N);
}

#[actix_web::test]
async fn test_get_events_visibility() {
    let app = create_app().await;
    let db = create_db().await;
    let events = db.collection::<Event>("events");

    let request = test::TestRequest::get()
        .uri("/api/v3/events").to_request();

    const N : usize = 10;

    let generated_events = (0..N)
        .map(|i| {
            let mut event = random_event(i.try_into().unwrap(), None);
            if i % 2 == 0 {
                // This event is surely invisible
                event.visibility.end = DateTime::from_timestamp(0, 0);
            }
            event
        });
    events.insert_many(generated_events, None).await.expect("Cannot insert events");

    let response: Vec<PrunedEvent> = test::call_and_read_body_json(&app, request).await;

    assert_eq!(response.len(), (N + 1) / 2, "Fetched {}/{} events", response.len(), (N + 1) / 2);
}

#[actix_web::test]
async fn test_get_events_valid_mode() {
    let app = create_app().await;

    for mode in ["overwrite", "ifempty", "combine"] {
        let request = test::TestRequest::get()
            .uri(&format!("/api/v3/events?mode={}", mode)).to_request();

        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), http::StatusCode::OK, "Unexpected status code");
    }
}

#[actix_web::test]
async fn test_get_events_invalid_mode() {
    let app = create_app().await;

    let request = test::TestRequest::get()
        .uri("/api/v3/events?mode=invalid_mode").to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST, "Unexpected status code");
}

#[actix_web::test]
async fn test_get_events_valid_category() {
    let app = create_app().await;
    let db = create_db().await;

    let categories = db.collection::<Category>("categories");

    categories.insert_many(vec![
        create_category(1),
        create_category(2),
        create_category(3),
    ], None).await.expect("Cannot insert categories");

    for param in ["addi", "subi", "addb", "subb"] {
        let request = test::TestRequest::get()
            .uri(&format!("/api/v3/events?{}=1,2,3", param)).to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::OK, "Unexpected status code with valid {} categories", param);
    }
}

#[actix_web::test]
async fn test_get_events_inexistent_category() {
    let app = create_app().await;
    let db = create_db().await;

    let categories = db.collection::<Category>("categories");

    categories.insert_many(vec![
        create_category(1),
        create_category(2),
        create_category(3),
    ], None).await.expect("Cannot insert categories");

    for param in ["addi", "subi", "addb", "subb"] {
        let request = test::TestRequest::get()
            .uri(&format!("/api/v3/events?{}=1,2,3,4", param)).to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY, "Unexpected status code with invalid {} categories", param);
    }
}

#[actix_web::test]
async fn test_get_events_invalid_category() {
    let app = create_app().await;
    let db = create_db().await;

    let categories = db.collection::<Category>("categories");

    categories.insert_many(vec![
        create_category(1),
        create_category(2),
        create_category(3),
    ], None).await.expect("Cannot insert categories");

    for param in ["addi", "subi", "addb", "subb"] {
        let request = test::TestRequest::get()
            .uri(&format!("/api/v3/events?{}=1,2,3,a", param)).to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST, "Unexpected status code with invalid {} categories", param);
    }
}

#[actix_web::test]
async fn test_get_event_details() {
    let app = create_app().await;
    let db = create_db().await;

    let events = db.collection::<Event>("events");

    const ID: i32 = 154;
    let event = random_event(ID, None);

    events.insert_one(event.clone(), None).await.expect("Cannot insert the event");

    let request = test::TestRequest::get()
        .uri(&format!("/api/v3/events/{}", ID)).to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK, "Event #{} should be present inside the database", ID);

    let request = test::TestRequest::get()
        .uri(&format!("/api/v3/events/{}", ID)).to_request();
    let response : Event = test::call_and_read_body_json(&app, request).await;

    assert_eq!(response, event, "wrong event");
}

#[actix_web::test]
async fn test_get_event_details_invalid_id() {
    let app = create_app().await;

    let request = test::TestRequest::get().uri("/api/v3/events/").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

    let request = test::TestRequest::get().uri("/api/v3/events/non_numeric_id").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_get_event_details_inexistent_event() {
    let app = create_app().await;
    let db = create_db().await;

    let events = db.collection::<Event>("events");

    const ID: i32 = 154;
    let event = random_event(ID, None);

    events.insert_one(event.clone(), None).await.expect("Cannot insert the event");

    let request = test::TestRequest::get()
        .uri(&format!("/api/v3/events/{}", ID + 1)).to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND, "Event #{} should be present inside the database", ID);
}

fn random_event(id: i32, categories: Option<Vec<i32>>) -> Event {
    let polygons = geojson::FeatureCollection::from_iter(iter::empty());
    let validity = NullableDateTimeRange::new(None, None);

    Event{
        id,
        creator_id: random(),
        title: "Random event".to_owned(),
        description: "Random event".to_owned(),
        summary: "Random event".to_owned(),
        locked_by: None,
        polygons: polygons.clone(),
        categories: categories.unwrap_or_default(),
        remote_document: None,
        risk_level: RiskLevel::Info,
        validity: validity.clone(),
        visibility: NullableDateTimeRange::new(None, None),
        subevents: vec![
            SubEvent{
                title: "Random subevent".to_owned(),
                description: "Random subevent".to_owned(),
                polygons,
                validity,
            }
        ],
    }
}

fn create_category(id: i32) -> Category {
    Category {
        id,
        name: id.to_string(),
        modifiable: false,
        supercategory_id: None,
        subcategories_ids: vec![]
    }
}


async fn create_db() -> Database {
    let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017/").await
        .expect("Cannot create the client");
    client.database("beelive_test")
}

async fn create_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let database = create_db().await;

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
