use actix_web::HttpResponse;
use futures::StreamExt;
use geojson::FeatureCollection;
use mongodb::{results::InsertOneResult, Collection, bson};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
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

pub async fn get_events_by_id(mongodb_events_collection: Collection<Event>, user_id: String) -> HttpResponse {

    /*let filter = bson::doc! { "id": user_id };
    let cursor = mongodb_events_collection
        .clone_with_type::<PrunedEvent>()
        .find(filter, None)
        .await
        .unwrap();

    let events: Vec<mongodb::error::Result<PrunedEvent>> = cursor.collect().await;
    let events: Vec<PrunedEvent> = events.into_iter().flatten().collect();*/

    let cursor = mongodb_events_collection
        .clone_with_type::<Event>()
        .find(None, None)
        .await
        .unwrap();

    let events: Vec<mongodb::error::Result<Event>> = cursor.collect().await;
    let events: Vec<Event> = events.into_iter().flatten().collect();

    HttpResponse::Ok().json(events)
}

pub async fn modify_event (mongodb_events_collection: Collection<Event>, event_id: u32, mut event: Event) -> HttpResponse{

    // L'evento da modificare viene passato per intero -> Viene sovrascritto nel database
    let filter = doc! { "id": event_id };
    let replacement = event;
    let res = mongodb_events_collection.replace_one(filter, replacement, None).await;

    // Controllo operazioni
    if let Ok(_) = res {
        // Se a buon fine -> 200 OK
        HttpResponse::Ok().finish()
    } else {
        // Se errore durante l'operazione -> 500 Internal Server Error
        HttpResponse::InternalServerError().finish()
    }
}

pub async fn delete_event (mongodb_events_collection: Collection<Event>, event_id: u32) -> HttpResponse{

    // L'evento da eliminare viene passato per intero -> Viene eliminato dal database
    let filter = doc! { "id": event_id };
    let res = mongodb_events_collection.delete_one(filter, None).await;

    // Controllo operazioni
    if let Ok(_) = res {
        // Se a buon fine -> 200 OK
        HttpResponse::Ok().finish()
    } else {
        // Se errore durante l'operazione -> 500 Internal Server Error
        HttpResponse::InternalServerError().finish()
    }
}