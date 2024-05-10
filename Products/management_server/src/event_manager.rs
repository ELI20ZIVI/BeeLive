// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use actix_web::web::{self, Data};
use mongodb::{results::InsertOneResult, Collection};

use crate::dao::{self, objects::Event};
use crate::event_processor::{EventProcessor, EventProcessorImpl};

static EVENT_PROCESSOR : EventProcessorImpl = EventProcessorImpl::new();
pub async fn insert_new_event(mongodb_collection: &Collection<Event>, mut event: Event) -> mongodb::error::Result<InsertOneResult> {
    EVENT_PROCESSOR.process(&mut event);

    dao::insert_new_event(mongodb_collection, event).await
}
