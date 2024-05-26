use actix_web::{web::Data, Result, HttpResponse};
use actix_web::http::StatusCode;
use futures::stream::StreamExt;
use mongodb::{bson::doc, options::FindOptions, results::InsertOneResult, Collection};
use mongodb::bson::Bson::ObjectId;
use crate::dao::objects::*;

pub mod objects;


/// Inserts a new event into the collection defined by the "mongobd_collection" handle.
/// The return is the result returned by the mongodb::Collection::insert_one call executed
/// in this function.
///
///
/// * `mongodb_collection` -  handle to the mongobd collection you want to insert this new event into.
/// * `event` -  event you want to insert into given collection 
pub async fn insert_new_event(mongodb_event_collection: &Collection<Event>, mongodb_category_collection: &Collection<Category>, event: Event) -> mongodb::error::Result<InsertOneResult> {
    mongodb_event_collection.insert_one(event, None).await
}

pub async fn checkCategories (target_ids : Vec<i32>, mongodb_category_collection: &Collection<Category>) -> bool {
    for id in target_ids {
        println!("Checking category with id {}...", id);
        let filter = doc! { "id": id};
        let result = mongodb_category_collection.find_one(filter, None).await;

        match result {
            Ok(o_category) => {
                match o_category {
                    Some(category) => {
                        // Category exists
                    }
                    None => {
                        // Category does not exist - error 422
                        return false;
                    }
                }
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
    return true;
}

/// Queries all events stored in the csollection defined by the "mongodb_collection" handle but
/// uses `PrunedEvent` as a projection document in order to not read all data about the events.
/// Returns a vector of PrunedEvents. 
///
/// The crate `docs.rs/mongodb` allows automatic deserialization of BSON data (obtained from querying the
/// mongodb database), thus manual deserialization into `Vec<Event>` using the `serde` crate is not
/// needed.
pub async fn query_pruned_events(mongodb_event_collection: Data<Collection<Event>>, mongodb_category_collection: Data<Collection<Category>>, modeFilter : &str, addbFilter : Vec<i32>, subbFilter : Vec<i32>, addiFilter : Vec<i32>, subiFilter : Vec<i32>) -> HttpResponse {
    // Search options
    let find_options = FindOptions::builder().projection(PrunedEvent::mongodb_projection()).build();



    // Controllo validità categorie di filtro
    let mut valid = checkCategories(addbFilter.clone(), &mongodb_category_collection).await;
    if !valid {
        return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
    }
    valid = checkCategories(subbFilter.clone(), &mongodb_category_collection).await;
    if !valid {
        return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
    }
    valid = checkCategories(addiFilter.clone(), &mongodb_category_collection).await;
    if !valid {
        return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
    }
    valid = checkCategories(subiFilter.clone(), &mongodb_category_collection).await;
    if !valid {
        return HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY);
    }

    // DB query
    let cursor = mongodb_event_collection
        .clone_with_type::<PrunedEvent>()
        .find(None, find_options)
        .await
        .unwrap();

    /*
    // Filtri locali, variabile vuota di esempio
    let mut local_filters = doc! {};

    // Query considerando le configurazioni degli utenti da mode
    // Ricavo del valore di mode
    let mode = modeFilter.get("mode").unwrap().as_str().unwrap();
    if (mode == "overwrite") {
        // Sovrascrittura preferenze locali con quelle remote
    } else if (mode == "combine") {
        // Combinazione preferenze locali e remote
    } else if (mode == "ifempty") {
        // Parametri remoti se non presenti locali
    } else {
        // Unuseful but necessary to avoid compiler warning
        return HttpResponse::BadRequest().body("Invalid mode value.");
    }
    */

    // Results
    let events: Vec<mongodb::error::Result<PrunedEvent>> = cursor.collect().await;
    let events: Vec<PrunedEvent> = events.into_iter().flatten().collect();

    return HttpResponse::Ok().json(events);
}

pub async fn query_full_event_single(mongodb_collection: Data<Collection<Event>>, event_id: u32) -> HttpResponse {

    let filter = doc! { "id": event_id};
    let result = mongodb_collection.find_one(filter, None).await;

    match result {
        Ok(o_event) => {
            return HttpResponse::Ok().json(o_event);
        }
        Err(error) => {
            return HttpResponse::NotFound().json(Error::NotFound("Event not Found"))
        }

    }
}

// HttpResponse::NotFound().json(Error::NotFound("Not Found"))