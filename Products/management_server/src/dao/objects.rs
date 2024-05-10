use std::iter;
use chrono::{DateTime, Local};
use geojson::{FeatureCollection, Feature};
use geojson::Value::Point;
use mongodb::bson::{doc, Document};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    id: i32,
    name: String,
    modifiable: bool,
    supercategory_id: Option<i32>,
    subcategories_ids: Vec<i32>,
}

type NullableDateTime = Option<DateTime<Local>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NullableDateTimeRange {
    pub start : NullableDateTime,
    pub end : NullableDateTime,
}

impl NullableDateTimeRange {

    pub fn new(start : NullableDateTime, end : NullableDateTime) -> NullableDateTimeRange {
        NullableDateTimeRange{start, end}
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubEvent {
    pub title: String,
    pub description: String,
    pub geometry: FeatureCollection,
    pub validity : NullableDateTimeRange,
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
    validity : NullableDateTimeRange,
    visibility: NullableDateTimeRange,
    category_ids: Vec<i32>,
    geojson_geometry: FeatureCollection,
}

impl PrunedEvent {

    /// Returns a MongoDB projection document with fields corresponding to the PrunedEvent fields.
    /// It is used in crate::dao::query_pruned_events in order to only read given fields out of the
    /// Events database.
    pub fn mongodb_projection() -> Document {
        doc! {
            "id": 1,
            "title": 1,
            "summary": 1,
            "validity": 1,
            "visibility": 1,
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
    pub id: i32,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub remote_document: Option<String>,
    pub validity: NullableDateTimeRange,
    pub visibility: NullableDateTimeRange,
    pub category_ids: Vec<i32>,
    pub geojson_geometry: FeatureCollection,
    pub subevents: Vec<SubEvent>,
    #[serde(skip)]
    pub locked_by: Option<i32>,
    #[serde(skip)]
    pub creator_id: i32,
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
            validity: NullableDateTimeRange::new(Some(now), Some(now)),
            visibility: NullableDateTimeRange::new(Some(now), Some(now)),
            locked_by: None,
            geojson_geometry: FeatureCollection::from_iter(iter::once(Feature::from(Point(vec![10.0, 6.0])))),
            creator_id: 0,
            subevents: vec![],
            category_ids: vec![],
        }
    }
}
