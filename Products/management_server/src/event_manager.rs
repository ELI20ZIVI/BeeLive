// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use std::cmp::max;
use actix_web::HttpResponse;

use actix_web::web::Data;
use futures::StreamExt;
use mongodb::{bson, Collection};
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;

use crate::dao::{self, objects::Event};
use crate::{event_processor, InsertNewEventEndpointData};
use crate::dao::objects::User;

/// Manages the addition of a new event. Uses the counter stored in 'data' as the event's ID and
/// then increments given counter by 1.
/// If 'data' is None, the function will call load_initial_counter in order to load the initial
/// counter value, which will panic if it fails to do so.
/// This function will panic if, for some reason, the counter contained in 'data' remains None
/// after trying to load it.
pub async fn insert_new_event(mut data: Data<InsertNewEventEndpointData>, mut event: Event) -> (mongodb::error::Result<InsertOneResult>, i32) {
    let _ = event_processor::process(&mut event);

    if data.counter.get().is_none() {
        load_initial_counter(&mut data).await;
    }

    let current_counter: i32 = data.counter.get().unwrap();

    let result = dao::insert_new_event(data.mongodb_events_collection.clone(), event, data.counter.get().unwrap()).await;
    data.counter.set(Some(current_counter+1));

    (result, data.counter.get().unwrap())
}

// TODO: document
pub async fn load_initial_counter(data: &mut Data<InsertNewEventEndpointData>) {

    let mut max_ = -1;

    match data.mongodb_events_collection.find(None, None).await { 
        Ok(mut cursor) => {
            while let Some(Ok(event)) = cursor.next().await {
                max_ = max(max_, event.id);
            }
        }
        Err(error) => {
            panic!("There was an error while trying to load the initial ID counter's value. Error: {}", error);
        }
    }
    data.counter.set(Some(max_+1));

}

pub async fn check_id (id: u32, mongodb_id_collection: Data<Collection<User>>) -> bool {
    let filter = doc! { "id": id};
    let result = mongodb_id_collection.find_one(filter, None).await;

    match result {
        Ok(o_id) => {
            match o_id {
                Some(_) => {
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
    true
}

pub async fn list_events_by_id(mongodb_events_collection: Data<Collection<Event>>, mongodb_id_collection: Data<Collection<User>>, user_id: u32) -> HttpResponse {

    let result;

    // Controllo esistenza utente
    let userCheck = check_id(user_id, mongodb_id_collection).await;


    if !userCheck {
        result = dao::get_events_by_id(mongodb_events_collection, user_id).await;
        return result
    } else {
        return HttpResponse::UnprocessableEntity().finish();
    }
}