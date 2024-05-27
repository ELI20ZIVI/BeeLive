use actix_web::HttpResponse;
use actix_web::web::Data;
use futures::StreamExt;
use mongodb::{results::InsertOneResult, Collection, bson};
use crate::dao::objects::*;

pub mod objects;


/// Inserts a new event into the collection defined by the "mongobd_collection" handle.
/// The return is the result returned by the mongodb::Collection::insert_one call executed
/// in this function.
///
///
/// * `mongodb_collection` -  handle to the mongobd collection you want to insert this new event into.
/// * `event` -  event you want to insert into given collection 
pub async fn insert_new_event(mongodb_collection: Collection<Event>, mut event: Event, id: i32) -> mongodb::error::Result<InsertOneResult> {
    event.id = id;
    mongodb_collection.insert_one(event, None).await
}

pub async fn get_events_by_id(mongodb_events_collection: Data<Collection<Event>>, user_id: u32) -> HttpResponse {

    let filter = bson::doc! { "id": user_id };
    let cursor = mongodb_events_collection
        .clone_with_type::<PrunedEvent>()
        .find(filter, None)
        .await
        .unwrap();

    let events: Vec<mongodb::error::Result<PrunedEvent>> = cursor.collect().await;
    let events: Vec<PrunedEvent> = events.into_iter().flatten().collect();

    HttpResponse::Ok().json(events)
}