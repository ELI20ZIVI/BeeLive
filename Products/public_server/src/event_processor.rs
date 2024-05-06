// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use actix_web::web::Data;
use mongodb::Collection;

use crate::dao::{self, objects::{Event, PrunedEvent}};

/// Pass-through a dao::query_events.
/// Per la documentazione riferirsi a dao::query_events.
pub async fn get_events(monbodb_collection: Data<Collection<Event>>) -> Vec<PrunedEvent>{
    dao::query_pruned_events(monbodb_collection).await
}
