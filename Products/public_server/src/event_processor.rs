// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use actix_web::{HttpResponse, web};
use actix_web::web::Data;
use mongodb::bson::Document;
use mongodb::Collection;

use crate::dao::{self, objects::{Event, PrunedEvent}};
use crate::EventQueryData;

fn process_comma_separated_values(field_name: &str, field_value: &Option<String>, filter: &mut Document) {
    if let Some(value) = field_value {
        let mut vec = Vec::new();
        for val in value.split(',') {
            match val.parse::<f64>() {
                Ok(num) => vec.push(num),
                Err(_) => println!("Il valore '{}' nel campo '{}' non è un numero valido.", val, field_name),
            }
        }
        filter.insert(field_name, vec);
    }
}

/// Pass-through a dao::query_pruned_events.
/// Per la documentazione riferirsi a dao::query_pruned_events.
pub async fn get_events(monbodb_collection: Data<Collection<Event>>, data: web::Query<EventQueryData>) -> Vec<PrunedEvent> {

    let mut modeFilter = mongodb::bson::Document::new();
    let mut addbFilter = mongodb::bson::Document::new();
    let mut addiFilter = mongodb::bson::Document::new();
    let mut subbFilter = mongodb::bson::Document::new();
    let mut subiFilter = mongodb::bson::Document::new();

    if let Some(m) = &data.mode {
        match m.as_str() {
            "overwrite" | "combine" | "ifempty" => {
                modeFilter.insert("mode", m.clone());
            }
            _ => {
                println!("'{}' non è un valore valido per il campo mode", m);
            }
        }
    }

    process_comma_separated_values("addb", &data.addb, &mut addbFilter);
    process_comma_separated_values("subb", &data.subb, &mut subbFilter);
    process_comma_separated_values("subi", &data.subi, &mut subiFilter);
    process_comma_separated_values("addi", &data.addi, &mut addiFilter);

    dao::query_pruned_events(monbodb_collection, modeFilter, addbFilter, subbFilter, addiFilter, subiFilter).await
}


/// Pass-through a dao::query_full_event_single
/// Per la documentazione riferirsi a dao::query_full_event_single
pub async fn get_event(mongodb_collection: Data<Collection<Event>>, event_id: u32) -> Option<Event> {
    dao::query_full_event_single(mongodb_collection, event_id).await
}
