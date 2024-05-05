use actix_web::web::Data;
use futures::stream::StreamExt;
use mongodb::{results::InsertOneResult, Collection};
use crate::dao::objects::*;

pub mod objects;


/// Insert a new event into the collection defined by the "mongobd_collection" handle.
/// The return is the result returned by the mongodb::Collection::insert_one call executed
/// in this function.
///
///
/// * `mongodb_collection` -  handle to the mongobd collection you want to insert this new event into.
/// * `event` -  event you want to insert into given collection 
pub async fn insert_new_event(mongodb_collection: Collection<Event>, event: Event) -> mongodb::error::Result<InsertOneResult> {
    mongodb_collection.insert_one(event, None).await
}

/// Query all events stored in the collection defined by the "mongodb_collection" handle and
/// return them as a vector of Events. 
///
/// The crate `docs.rs/mongodb` allows automatic deserialization of BSON data (obtained from querying the
/// mongodb database), thus manual deserialization into `Vec<Event>` using the `serde` crate is not
/// needed.
pub async fn query_events(mongodb_collection: Data<Collection<Event>>) -> Vec<Event> {

    let cursor = mongodb_collection.find(None, None).await.unwrap();

    let events: Vec<mongodb::error::Result<Event>> = cursor.collect().await;
    let events: Vec<Event> = events.into_iter().flatten().collect();

    events
}

