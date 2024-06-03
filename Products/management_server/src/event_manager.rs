// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use std::cmp::max;
use actix_web::HttpResponse;

use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;

use crate::dao::{objects::Event, MongodbExtension};
use crate::{event_processor, AppData, locker};
use crate::dao::objects::User;

/// Manages the addition of a new event. Uses the counter stored in 'data' as the event's ID and
/// then increments given counter by 1.
/// If 'data' is None, the function will call load_initial_counter in order to load the initial
/// counter value, which will panic if it fails to do so.
/// This function will panic if, for some reason, the counter contained in 'data' remains None
/// after trying to load it.
pub async fn insert_new_event(mut data: &AppData, mut event: Event) -> (mongodb::error::Result<InsertOneResult>, i32) {

    let _ = event_processor::process(&mut event);

    if data.counter.get().is_none() {
        load_initial_counter(&mut data).await;
    }

    let current_counter: i32 = data.counter.get().unwrap();

    let result = data.mongodb.insert_new_event(event, data.counter.get().unwrap()).await;
    data.counter.set(Some(current_counter+1));

    (result, data.counter.get().unwrap())
}

pub async fn load_initial_counter(data: &mut &AppData) {

    let mut max_ = -1;

    match data.mongodb.events().find(None, None).await { 
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

pub async fn list_events_by_id(data: &AppData, page: &Option<u64>, user_id: &str) -> HttpResponse {
    // Controllo numero pagina. Non può essere negativo.
    if page.map(|page| page < 1).unwrap_or(false) {
        return HttpResponse::UnprocessableEntity().finish();
    }

    data.mongodb.get_events_of_user(user_id).await
}

// TODO: documentation... checks what?
// I think this method should be put into [event_processor].
pub async fn check_event (event: Event) -> bool {
    return true
}

pub async fn modify_event(data: &AppData, event_id: u32, mut event: Event, user_id: &str) -> HttpResponse {

    // Check evento bloccato
    if !locker::is_resource_locked(event_id).await {
        if !locker::is_resource_locked_by_user(event_id, user_id).await {
            // TODO: the message is not correct. In this branch the event hasn't been locked by
            // anyone. No action is taken to avoid anarchy.
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
    // TODO: DAO!
    let filter = doc! { "id": event_id.clone() };
    let event = match data.mongodb.events().find_one(filter, None).await.unwrap() {
        None => return HttpResponse::NotFound().finish(),
        Some(event) => event,
    };

    // Modifica evento
    data.mongodb.modify_event(event_id, &event).await
}

pub async fn delete_event(data: &AppData, event_id: u32, user_id: &str) -> HttpResponse {
    // Ccontrollo esistenza utente nel db
    let user = data.mongodb.authorized_users().find_one(doc! { "id": user_id.clone() }, None).await.unwrap();
    if user.is_none() {
        return HttpResponse::Forbidden().body("User not authorized");
    }

    // Controllo esistenza evento nel db
    // TODO: DAO!
    let filter = doc! { "id": event_id.clone() };
    let event = match data.mongodb.events().find_one(filter.clone(), None).await.unwrap() {
        // Se evento non esistente impossibile la modifica - Restituzione errore 404
        None => return HttpResponse::NotFound().body("Evento non trovato all'interno del database"),
        Some(event) => event,
    };

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

    data.mongodb.delete_event(event_id).await
}
