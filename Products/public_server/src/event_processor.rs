// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use actix_web::{HttpResponse, web};
use actix_web::web::Data;
use mongodb::bson::Document;
use mongodb::Collection;

use crate::dao::{self, objects::{Event, PrunedEvent}};
use crate::EventQueryData;

/// Pass-through a dao::query_pruned_events.
/// Per la documentazione riferirsi a dao::query_pruned_events.
pub async fn get_events(monbodb_collection: Data<Collection<Event>>, data: web::Query<EventQueryData>) -> HttpResponse {

    let mut modeFilter = Document::new();
    let mut addbFilter = Document::new();
    let mut addiFilter = Document::new();
    let mut subbFilter = Document::new();
    let mut subiFilter = Document::new();

    if let Some(m) = &data.mode {
        match m.as_str() {
            "overwrite" | "combine" | "ifempty" => {
                modeFilter.insert("mode", m.clone());
            }
            _ => {
                return HttpResponse::BadRequest().body("Invalid mode value.");
            }
        }
    }

    if let Some(value) = &data.addb {
        for val in value.split(',') {
            match val.parse::<f64>() {
                Ok(num) => {
                    addbFilter.insert("addb", num);
                }
                Err(_) => {
                    // Se il valore non può essere convertito in f64, restituisci un errore 400
                    return HttpResponse::BadRequest().body("Invalid addb value.");
                }
            }
        }
    }

    if let Some(value) = &data.addi {
        for val in value.split(',') {
            match val.parse::<f64>() {
                Ok(num) => {
                    addiFilter.insert("addi", num);
                }
                Err(_) => {
                    // Se il valore non può essere convertito in f64, restituisci un errore 400
                    return HttpResponse::BadRequest().body("Invalid addi value.");
                }
            }
        }
    }

    if let Some(value) = &data.subi {
        for val in value.split(',') {
            match val.parse::<f64>() {
                Ok(num) => {
                    subiFilter.insert("subi", num);
                }
                Err(_) => {
                    // Se il valore non può essere convertito in f64, restituisci un errore 400
                    return HttpResponse::BadRequest().body("Invalid subi value.");
                }
            }
        }
    }

    if let Some(value) = &data.subb {
        for val in value.split(',') {
            match val.parse::<f64>() {
                Ok(num) => {
                    subiFilter.insert("subb", num);
                    subiFilter.add("subb", num);
                }
                Err(_) => {
                    // Se il valore non può essere convertito in f64, restituisci un errore 400
                    return HttpResponse::BadRequest().body("Invalid subb value.");
                }
            }
        }
    }

    return dao::query_pruned_events(monbodb_collection, modeFilter, addbFilter, subbFilter, addiFilter, subiFilter).await
}


/// Pass-through a dao::query_full_event_single
/// Per la documentazione riferirsi a dao::query_full_event_single
pub async fn get_event(mongodb_collection: Data<Collection<Event>>, event_id: u32) -> Option<Event> {
    dao::query_full_event_single(mongodb_collection, event_id).await
}
