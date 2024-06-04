use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::{results::InsertOneResult, Collection, Database};
use mongodb::bson::doc;
use crate::dao::objects::*;

pub mod objects;

/// Inserts a new event into the collection defined by the "mongobd_collection" handle.
/// The return is the result returned by the mongodb::Collection::insert_one call executed
/// in this function.
///
///
/// * `mongodb_collection` -  handle to the mongobd collection you want to insert this new event into.
/// * `event` -  event you want to insert into given collection 
async fn insert_new_event(mongodb_collection: &Collection<Event>, mut event: Event, id: i32) -> mongodb::error::Result<InsertOneResult> {
    event.id = id;
    mongodb_collection.insert_one(event, None).await
}

async fn get_events_by_id(mongodb_events_collection: &Collection<Event>, user_id: String) -> HttpResponse {

    /*let filter = bson::doc! { "id": user_id };
    let cursor = mongodb_events_collection
        .clone_with_type::<PrunedEvent>()
        .find(filter, None)
        .await
        .unwrap();

    let events: Vec<mongodb::error::Result<PrunedEvent>> = cursor.collect().await;
    let events: Vec<PrunedEvent> = events.into_iter().flatten().collect();*/

    let cursor = mongodb_events_collection
        .clone_with_type::<Event>()
        .find(None, None)
        .await
        .unwrap();

    let events: Vec<mongodb::error::Result<Event>> = cursor.collect().await;
    let events: Vec<Event> = events.into_iter().flatten().collect();

    HttpResponse::Ok().json(events)
}

async fn modify_event (mongodb_events_collection: &Collection<Event>, event_id: u32, event: &Event) -> HttpResponse{

    // L'evento da modificare viene passato per intero -> Viene sovrascritto nel database
    let filter = doc! { "id": event_id };
    let replacement = event;
    let res = mongodb_events_collection.replace_one(filter, replacement, None).await;

    // Controllo operazioni
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_event (mongodb_events_collection: &Collection<Event>, event_id: u32) -> HttpResponse{

    // L'evento da eliminare viene passato per intero -> Viene eliminato dal database
    let filter = doc! { "id": event_id };
    let res = mongodb_events_collection.delete_one(filter, None).await;

    // Controllo operazioni
    match res {
        // Se a buon fine -> 200 OK
        Ok(_) => HttpResponse::Ok().finish(),
        // Se errore durante l'operazione -> 500 Internal Server Error
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}



// TODO: composition
pub trait MongodbExtension {

    /// Gets the collection of events from the database.
    fn events(&self) -> Collection<Event>;
    
    /// Gets the collection of generic users from the database.
    fn users(&self) -> Collection<User>;
    
    /// Gets the collection of authorized users from the database.
    fn authorized_users(&self) -> Collection<User>;

    // TODO: in my opinion [id] is responsability of the database (DAO).
    // Shouldn't be passed from outside.
    /// Inserts the given [event] into the [events] collection.
    async fn insert_new_event(&self, event: Event, id: i32) -> mongodb::error::Result<InsertOneResult> {
        insert_new_event(&self.events(), event, id).await
    }

    /// Gets the list of events of competence of a given [user_id].
    async fn get_events_of_user(&self, user_id: &str) -> HttpResponse {
        get_events_by_id(&self.events(), user_id.to_string()).await
    }

    /// Updates the [event_id] with the data in [event].
    async fn modify_event(&self, event_id: u32, event: &Event) -> HttpResponse {
        modify_event(&self.events(), event_id, event).await
    }

    /// Deletes the event [event_id].
    async fn delete_event(&self, event_id: u32) -> HttpResponse {
        delete_event(&self.events(), event_id).await
    }

    /// Checks if the [user_id] corresponds to an authorized user.
    ///
    /// Further checks can be performed in order to ensure AC policies.
    async fn is_authorized(&self, user_id: &str) -> mongodb::error::Result<bool> {

        // Controllo esistenza utente nel db
        let user = self.authorized_users().find_one(doc! { "id": user_id }, None).await?;

        Ok(user.is_some())
    }
}


impl MongodbExtension for Database {

    fn events(&self) -> Collection<Event> {
        self.collection("events")
    }

    fn users(&self) -> Collection<User> {
        self.collection("users")
    }

    fn authorized_users(&self) -> Collection<User> {
        self.collection("authorized_users")
    }

}
