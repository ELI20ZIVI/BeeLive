// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use actix_web::{HttpResponse, web};
use actix_web::web::Data;
use mongodb::bson::Document;
use mongodb::Collection;

use crate::dao::{self, objects::{Event, PrunedEvent}};
use crate::dao::objects::Category;
use crate::EventQueryData;

pub fn check_add_sub(value: &str, filter: &mut Vec<i32>) -> bool {
    for val in value.split(',') {
        match val.parse::<i32>() {
            Ok(num) => {
                if num < 0 {
                    return false;
                }
                filter.push(num);
            }
            Err(_) => {
                // Se il valore non può essere convertito in f64, restituisci un errore 400
                return false;
            }
        }
    }
    true
}

/// Pass-through a dao::query_pruned_events.
/// Per la documentazione riferirsi a dao::query_pruned_events.
pub async fn get_events(mongodb_event_collection: Data<Collection<Event>>, mongodb_categories_collection: Data<Collection<Category>>, data: web::Query<EventQueryData>) -> HttpResponse {

    let mut modeFilter = "";
    let mut addbFilter : Vec<i32> = vec![];
    let mut subbFilter : Vec<i32> = vec![];
    let mut addiFilter : Vec<i32> = vec![];
    let mut subiFilter : Vec<i32> = vec![];;

    // TODO: Prendi le preferenze degli utenti e applicale ai filtri
    if let Some(m) = &data.mode {
        match m.as_str() {
            "overwrite" | "combine" | "ifempty" => {
                modeFilter = m.as_str();
            }
            _ => {
                return HttpResponse::BadRequest().body("Invalid mode value.");
            }
        }
    }

    if let Some(value) = &data.addb {
        if !check_add_sub(value, &mut addbFilter) {
            return HttpResponse::BadRequest().body("Invalid addb value.");
        }
    }
    if let Some(value) = &data.subb {
        if !check_add_sub(value, &mut subbFilter) {
            return HttpResponse::BadRequest().body("Invalid subb value.");
        }
    }
    if let Some(value) = &data.addi {
        if !check_add_sub(value, &mut addiFilter) {
            return HttpResponse::BadRequest().body("Invalid addi value.");
        }
    }
    if let Some(value) = &data.subi {
        if !check_add_sub(value, &mut subiFilter) {
            return HttpResponse::BadRequest().body("Invalid subi value.");
        }
    }

    return dao::query_pruned_events(mongodb_event_collection, mongodb_categories_collection, modeFilter, addbFilter, subbFilter, addiFilter, subiFilter).await;
}

/// Pass-through a dao::query_full_event_single
/// Per la documentazione riferirsi a dao::query_full_event_single
pub async fn get_event(mongodb_collection: Data<Collection<Event>>, event_id: u32) -> Option<Event> {

    if event_id < 0 {
        return HttpResponse::BadRequest().body("Invalid event-id value.");
    }

    dao::query_full_event_single(mongodb_collection, event_id).await
}
