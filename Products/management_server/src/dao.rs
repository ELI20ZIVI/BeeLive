use actix_web::{web::Data, Result};
use futures::stream::StreamExt;
use mongodb::{bson::doc, options::FindOptions, results::InsertOneResult, Collection};
use crate::dao::objects::*;

pub mod objects;


/// Inserts a new event into the collection defined by the "mongobd_collection" handle.
/// The return is the result returned by the mongodb::Collection::insert_one call executed
/// in this function.
///
///
/// * `mongodb_collection` -  handle to the mongobd collection you want to insert this new event into.
/// * `event` -  event you want to insert into given collection 
pub async fn insert_new_event(mongodb_collection: Data<Collection<Event>>, event: Event) -> mongodb::error::Result<InsertOneResult> {
    mongodb_collection.insert_one(event, None).await
}

/// Queries all events stored in the collection defined by the "mongodb_collection" handle but
/// uses `PrunedEvent` as a projection document in order to not read all data about the events.
/// Returns a vector of PrunedEvents. 
///
/// The crate `docs.rs/mongodb` allows automatic deserialization of BSON data (obtained from querying the
/// mongodb database), thus manual deserialization into `Vec<Event>` using the `serde` crate is not
/// needed.
pub async fn query_pruned_events(mongodb_collection: Data<Collection<Event>>) -> Vec<PrunedEvent> {

    let find_options = FindOptions::builder().projection(PrunedEvent::mongobd_projection()).build();

    let cursor = mongodb_collection.clone_with_type::<PrunedEvent>().find(None, find_options).await.unwrap();

    let events: Vec<mongodb::error::Result<PrunedEvent>> = cursor.collect().await;
    let events: Vec<PrunedEvent> = events.into_iter().flatten().collect();

    events
}

pub async fn query_full_event_single(mongodb_collection: Data<Collection<Event>>, event_id: u32) -> Option<Event> {

    let filter = doc! { "id": event_id};
    let result = mongodb_collection.find_one(filter, None).await;

    match result {
        Ok(o_event) => {
            o_event
        }
        Err(error) => {
            println!("{:?}", error);
            None
        }

    }
}

