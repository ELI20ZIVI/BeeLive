use actix_web::HttpResponse;
use futures::StreamExt;
use mongodb::{results::InsertOneResult, Collection, Database};
use mongodb::bson::doc;
use crate::dao::objects::*;
use std::cmp::max;

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

async fn get_events_by_id(mongodb_events_collection: &Collection<Event>, _user: &User) -> HttpResponse {

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
        Ok(res) => {
            if res.modified_count > 0 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        },
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
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().body(format!("Event with ID={} was not found", event_id))
            }
        },
        // Se errore durante l'operazione -> 500 Internal Server Error
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}


pub struct Dao {
    /// The collection of events of the database.
    pub events: Collection<Event>,
    /// The collection of generic users of the database.
    pub users: Collection<User>,
    /// The collection of authorized users of the database.
    pub authorized_users: Collection<User>,
}


// TODO: composition
impl Dao {

    // TODO: in my opinion [id] is responsability of the database (DAO).
    // Shouldn't be passed from outside.
    /// Inserts the given [event] into the [events] collection.
    pub async fn insert_new_event(&self, event: Event, id: i32) -> mongodb::error::Result<InsertOneResult> {
        insert_new_event(&self.events, event, id).await
    }

    /// Gets the list of events of competence of a given [user_id].
    pub async fn get_events_of_user(&self, user: &User) -> HttpResponse {
        get_events_by_id(&self.events, user).await
    }

    /// Updates the [event_id] with the data in [event].
    pub async fn modify_event(&self, event_id: u32, event: &Event) -> HttpResponse {
        modify_event(&self.events, event_id, event).await
    }

    /// Deletes the event [event_id].
    pub async fn delete_event(&self, event_id: u32) -> HttpResponse {
        delete_event(&self.events, event_id).await
    }

    /// Checks if the [user_id] corresponds to an authorized user.
    ///
    /// Further checks can be performed in order to ensure AC policies.
    pub async fn is_authorized(&self, user_id: &str) -> mongodb::error::Result<Option<User>> {

        // Controllo esistenza utente nel db
        let user = self.authorized_users.find_one(doc! { "id": user_id }, None).await?;

        Ok(user)
    }

    pub async fn available_event_id(&self) -> i32 {
        let id = match self.events.find(None, None).await {
            Ok(cursor) => {
                cursor
                    .filter_map(|e| async {e.ok().map(|e| e.id)})
                    .fold(-1, |res, id| async move {max(res, id)}).await
            }
            Err(error) => {
                panic!("There was an error while trying to load the initial ID counter's value. Error: {}", error);
            }
        };
        return id+1
    }
}


impl From<Database> for Dao {

    fn from(db: Database) -> Self {
        Self {
            events: db.collection("events"),
            users: db.collection("users"),
            authorized_users: db.collection("authorized_users"),
        }
    }

}
