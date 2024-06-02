// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use std::cmp::max;
use std::ptr::null;
use actix_web::HttpResponse;

use actix_web::web::Data;
use futures::StreamExt;
use mongodb::{bson, Collection};
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;

use crate::dao::{self, objects::Event};
use crate::{event_processor, ManagementServerData, EventQueryData, locker};
use crate::dao::objects::User;

/// Manages the addition of a new event. Uses the counter stored in 'data' as the event's ID and
/// then increments given counter by 1.
/// If 'data' is None, the function will call load_initial_counter in order to load the initial
/// counter value, which will panic if it fails to do so.
/// This function will panic if, for some reason, the counter contained in 'data' remains None
/// after trying to load it.
pub async fn insert_new_event(mut data: Data<ManagementServerData>, mut event: Event) -> (mongodb::error::Result<InsertOneResult>, i32) {

    let _ = event_processor::process(&mut event);

    if data.counter.get().is_none() {
        load_initial_counter(&mut data).await;
    }

    let current_counter: i32 = data.counter.get().unwrap();

    let result = dao::insert_new_event(data.mongodb_events_collection.clone(), event, data.counter.get().unwrap()).await;
    data.counter.set(Some(current_counter+1));

    (result, data.counter.get().unwrap())
}

pub async fn load_initial_counter(data: &mut Data<ManagementServerData>) {

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

pub async fn check_user_event(user: User, user_id: &str) -> bool {
    // Funzione non ancora implementata, per eventuali sprint futuri
    true
}

pub async fn list_events_by_id(data: Data<ManagementServerData>, page: &Option<u64>, user_id: &str) -> HttpResponse {

    // Controllo numero pagina
    if page < &Some(1) {
        return HttpResponse::UnprocessableEntity().finish();
    }

    // Ottenimento collezione eventi e utenti
    let mongodb_events_collection = data.mongodb_events_collection.clone();
    let mongodb_users_collection = data.mongodb_users_collection.clone();

    // Ccontrollo esistenza utente nel db
    let user = mongodb_users_collection.find_one(doc! { "id": user_id.clone() }, None).await.unwrap();
    if user.is_none() {
        return HttpResponse::Forbidden().body("User not authorized");
    }

    dao::get_events_by_id(mongodb_events_collection, user_id.to_string()).await
}

pub async fn check_event (event: Event) -> bool {
    return true
}

pub async fn modify_event(data: Data<ManagementServerData>, event_id: u32, mut event: Event, user_id: &str) -> HttpResponse {

    let mongodb_events_collection = data.mongodb_events_collection.clone();
    let mongodb_users_collection = data.mongodb_users_collection.clone();

    // Controllo esistenza utente nel db
    let user = mongodb_users_collection.find_one(doc! { "id": user_id.clone() }, None).await.unwrap();
    if user.is_none() {
        return HttpResponse::Forbidden().body("User not authorized");
    }

    // Check evento di competenza
    if !check_user_event(user.unwrap(), user_id).await {
        return HttpResponse::Forbidden().body("User not authorized");
    }

    // Check evento bloccatoù
    if !locker::is_resource_locked(event_id).await {
        if !locker::is_resource_locked_by_user(event_id, user_id).await {
            return HttpResponse::ImATeapot().body("Evento bloccato da un altro utente");
        }
    } else {
        return HttpResponse::Locked().body("Evento bloccato da altro utente");
    }

    // Processo dell'evento - Compilazione campi automatici
    let _ = event_processor::process(&mut event);

    // Controllo validità evento - Temporaneamente sempre valido
    let event_check_result = check_event(event).await;
    if !event_check_result {
        // Se evento non valido, restituisco errore 422
        return HttpResponse::UnprocessableEntity().finish();
    }

    // Controllo esistenza evento nel db
    let filter = doc! { "id": event_id.clone() };
    let event = mongodb_events_collection.find_one(filter, None).await.unwrap();
    if event.is_none() {
        // Se evento non esistente impossibile la modifica - Restituzione errore 404
        return HttpResponse::NotFound().finish();
    }

    // Modifica evento
    let result = dao::modify_event(mongodb_events_collection, event_id.clone(), event.unwrap()).await;
    result
}

pub async fn delete_event(data: Data<ManagementServerData>, event_id: u32, user_id: &str) -> HttpResponse {

    // Ccontrollo esistenza utente nel db
    let user = data.mongodb_users_collection.find_one(doc! { "id": user_id.clone() }, None).await.unwrap();
    if user.is_none() {
        return HttpResponse::Forbidden().body("User not authorized");
    }

    let mongodb_events_collection = data.mongodb_events_collection.clone();

    // Controllo esistenza evento nel db
    let filter = doc! { "id": event_id.clone() };
    let event = mongodb_events_collection.find_one(filter.clone(), None).await.unwrap();
    if event.is_none() {
        // Se evento non esistente impossibile la modifica - Restituzione errore 404
        return HttpResponse::NotFound().body("Evento non trovato all'interno del database");
    }

    // Check evento di competenza
    if !check_user_event(user.unwrap(), user_id).await {
        return HttpResponse::Forbidden().body("User not authorized");
    }

    // Check evento bloccato
    if !locker::is_resource_locked(event_id).await {
        if !locker::is_resource_locked_by_user(event_id, user_id).await {
            return HttpResponse::ImATeapot().body("Evento bloccato da un altro utente");
        }
    } else {
        return HttpResponse::Locked().body("Evento bloccato da altro utente");
    }

    dao::delete_event(mongodb_events_collection, event_id.clone()).await
}
