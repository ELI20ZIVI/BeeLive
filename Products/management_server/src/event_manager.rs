// Modulo che si frappone alle endpoint e il DAO, con lo scopo di fare eventuale caching e
// processamento deglie venti che passano tra i due.
// Per ora è solo un pass-through al DAO

use actix_web::HttpResponse;

use mongodb::results::InsertOneResult;

use crate::dao::objects::Event;
use crate::{event_processor, AppData, locker};
use crate::dao::objects::User;

/// Manages the addition of a new event. Uses the counter stored in 'data' as the event's ID and
/// then increments given counter by 1.
/// If 'data' is None, the function will call load_initial_counter in order to load the initial
/// counter value, which will panic if it fails to do so.
/// This function will panic if, for some reason, the counter contained in 'data' remains None
/// after trying to load it.
pub async fn insert_new_event(data: &AppData, mut event: Event, _user: &User) -> (mongodb::error::Result<InsertOneResult>, i32) {
    let _ = event_processor::process(&mut event);

    let mut counter = data.counter.lock().expect("Error: poisoned counter mutex");
    let available_id = match *counter {
        Some(c) => c,
        None => data.mongodb.available_event_id().await,
    };

    let result = data.mongodb.insert_new_event(event, available_id).await;
    *counter = Some(available_id + 1);

    (result, available_id)
}


// TODO: documentare la funzione
pub async fn check_user_event(_user: &User, _event_id: u32) -> bool {
    // Funzione non ancora implementata, per eventuali sprint futuri
    true
}

pub async fn list_events_by_id(data: &AppData, page: &Option<u64>, user: &User) -> HttpResponse {
    // Controllo numero pagina. Non può essere negativo.
    if page.map(|page| page < 1).unwrap_or(false) {
        return HttpResponse::UnprocessableEntity().finish();
    }

    data.mongodb.get_events_of_user(user).await
}

// TODO: documentation... checks what?
// I think this method should be put into [event_processor].
pub async fn check_event (_event: &Event) -> bool {
    return true
}

pub async fn modify_event(data: &AppData, event_id: u32, mut event: Event, user: &User) -> HttpResponse {
    // Check evento bloccato
    if locker::is_resource_locked(event_id).await {
        if !locker::is_resource_locked_by_user(event_id, user).await {
            // TODO: the message is not correct. In this branch the event hasn't been locked by
            // anyone. No action is taken to avoid anarchy.
            return HttpResponse::ImATeapot().body("Evento bloccato da un altro utente");
        }
    } else {
        return HttpResponse::Locked().body("Evento non bloccato");
    }

    // Processo dell'evento - Compilazione campi automatici
    let _ = event_processor::process(&mut event);

    // Controllo validità evento - Temporaneamente sempre valido
    let event_check_result = check_event(&event).await;
    if !event_check_result {
        // Se evento non valido, restituisco errore 422
        return HttpResponse::UnprocessableEntity().finish();
    }

    // Modifica evento
    data.mongodb.modify_event(event_id, &event).await
}

pub async fn delete_event(data: &AppData, event_id: u32, user: &User) -> HttpResponse {

    // Check evento di competenza
    if !check_user_event(user, event_id).await {
        return HttpResponse::Forbidden().body("User not authorized");
    }

    // Check evento bloccato
    if locker::is_resource_locked(event_id).await {
        if !locker::is_resource_locked_by_user(event_id, user).await {
            return HttpResponse::ImATeapot().body("Evento bloccato da un altro utente");
        }
    } else {
        return HttpResponse::Locked().body("Evento non bloccato");
    }

    data.mongodb.delete_event(event_id).await
}
