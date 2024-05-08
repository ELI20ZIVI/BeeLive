use std::default;

use chrono::{DateTime, Local};
use geojson::Geometry;
use mongodb::bson::{doc, Document};
use serde::{Serialize, Deserialize};

type ODate = Option<DateTime<chrono::Local>>;
type Date = DateTime<chrono::Local>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    id: i32,
    name: String,
    modifiable: bool,
    supercategory_id: Option<i32>,
    subcategories_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubEvent {
    id: i32,
    title: String,
    description: String,
    geometry: Geometry,
    start_validity: DateTime<chrono::Local>,
    end_validity: DateTime<chrono::Local>,

}

/// This struct represents the deserializable JSON data that the server sends to the client when
/// the client requests the list of events, aka `/events`.
/// It is a compact representation of an Event, thus 'pruning' some fields like subcategories and
/// locks. 
/// The purpose of the pruning is to save bandwidt when sending data to the client. 
#[derive(Serialize, Deserialize, Debug)]
pub struct PrunedEvent {
    id: i32,
    title: String,
    summary: String,
    start_validity: ODate,
    end_validity: ODate,
    start_visibility: ODate,
    end_visibility: ODate,
    category_ids: Vec<i32>,
    geojson_geometry: Geometry,
}

impl PrunedEvent {

    /// Returns a MongoDB projection document with fields corresponding to the PrunedEvent fields.
    /// It is used in crate::dao::query_pruned_events in order to only read given fields out of the
    /// Events database.
    pub fn mongobd_projection() -> Document {
        doc! { 
            "id": 1,
            "title": 1,
            "summary": 1,
            "start_validity": 1,
            "end_validity": 1,
            "start_visibility": 1,
            "end_visibility": 1,
            "category_ids": 1,
            "geojson_geometry": 1,
        }
    }
}

/// This struct represents the deserializable JSON data that the server sends to the client when
/// the client requests information about a single event, aka `/events/{event}`
/// It is the full representation of an Event, with internal fields like 'creator_id' and
/// `locked_by` that are not de/serializable, thus are not sent nor received to / from the client. 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    id: i32,
    title: String,
    summary: String,
    description: String,
    remote_document: Option<String>,
    start_validity: Date,
    end_validity: Date,
    start_visibility: ODate,
    end_visibility: ODate,
    category_ids: Vec<i32>,
    geojson_geometry: Geometry,
    subevents: Vec<SubEvent>,
    #[serde(skip)]
    locked_by: Option<i32>,
    #[serde(skip)]
    creator_id: i32,
}

impl Event {
    pub fn test_event() -> Event {

        let now = Local::now();

        Event {
            id: 0,
            title: "test".to_string(),
            description: "an amazing description".to_string(),
            remote_document: Some("test remote document . org".to_string()),
            summary: "a long summary".to_string(),
            start_validity: now,
            end_validity: now,
            start_visibility: Some(now),
            end_visibility: Some(now),
            locked_by: None,
            geojson_geometry: Geometry::new(geojson::Value::Point(vec![10.0, 6.0])),
            creator_id: 0,
            subevents: vec![],
            category_ids: vec![],
        }
    }
}
