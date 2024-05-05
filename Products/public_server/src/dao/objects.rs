use std::default;

use chrono::{DateTime, Local};
use geojson::Geometry;
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct Category {
    id: i32,
    name: String,
    modifiable: bool,
    supercategory_id: Option<i32>,
    subcategories_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    id: i32,
    title: String,
    summary: String,
    remote_document: Option<String>,
    start_validity: DateTime<chrono::Local>,
    end_validity: DateTime<chrono::Local>,
    start_visibility: DateTime<chrono::Local>,
    end_visibility: DateTime<chrono::Local>,
    category_ids: Vec<i32>,
    creator_id: i32,
    geojson_geometry: Geometry,
    locked_by: Option<i32>,
}

impl Event {
    pub fn test_event() -> Event {

        let now = Local::now();

        Event {
            id: 0,
            title: "test".to_string(),
            remote_document: Some("test remote document . org".to_string()),
            summary: "a long summary".to_string(),
            start_validity: now,
            end_validity: now,
            start_visibility: now,
            end_visibility: now,
            locked_by: None,
            geojson_geometry: Geometry::new(geojson::Value::Point(vec![10.0, 6.0])),
            creator_id: 0,
            category_ids: vec![],
        }
    }
}
