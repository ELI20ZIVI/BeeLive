// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use actix_web::web::{self, Data};
use mongodb::{results::InsertOneResult, Collection};

use crate::dao::{self, objects::{Event, PrunedEvent}};

/// Pass-through a dao::query_pruned_events.
/// Per la documentazione riferirsi a dao::query_pruned_events.
pub async fn get_events(monbodb_collection: Data<Collection<Event>>) -> Vec<PrunedEvent>{
    dao::query_pruned_events(monbodb_collection).await
}

/// Pass-through a dao::query_full_event_single
/// Per la documentazione riferirsi a dao::query_full_event_single
pub async fn get_event(mongodb_collection: Data<Collection<Event>>, event_id: u32) -> Option<Event> {
    dao::query_full_event_single(mongodb_collection, event_id).await
}

pub async fn insert_new_event(mongodb_collection: Data<Collection<Event>>, event: web::Json<Event>) -> mongodb::error::Result<InsertOneResult> {
    dao::insert_new_event(mongodb_collection, event.into_inner()).await
}
