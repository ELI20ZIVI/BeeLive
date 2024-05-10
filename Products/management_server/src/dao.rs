use mongodb::{results::InsertOneResult, Collection};
use crate::dao::objects::*;

pub mod objects;


/// Inserts a new event into the collection defined by the "mongobd_collection" handle.
/// The return is the result returned by the mongodb::Collection::insert_one call executed
/// in this function.
///
///
/// * `mongodb_collection` -  handle to the mongobd collection you want to insert this new event into.
/// * `event` -  event you want to insert into given collection 
pub async fn insert_new_event(mongodb_collection: &Collection<Event>, event: Event) -> mongodb::error::Result<InsertOneResult> {
    mongodb_collection.insert_one(event, None).await
}

