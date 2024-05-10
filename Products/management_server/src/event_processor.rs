mod errors;
mod test;

use std::convert::TryInto;
use geo::{BooleanOps, Geometry, MultiPolygon};
use geojson::{FeatureCollection, Feature};
use crate::dao::objects::{Event};
use crate::event_processor::errors::Error;

pub fn process(event: &mut Event) -> Result<(), Error> {
    // Aggregates the features.
    // Features converted to geo::Geometry.
    let geometries : Vec<Geometry> = event.subevents
        .iter()
        .flat_map(|se| {
            se.geometry.features.iter().filter_map(|f| {
                f.geometry.as_ref().map(|g| {
                    g.try_into().ok()
                }).flatten()
            })
        }).collect();

    let mut polygons : Vec<geo::Polygon> = Vec::new();
    let mut lines : Vec<geo::Line> = Vec::new();
    let mut points : Vec<geo::Point> = Vec::new();

    for geometry in geometries {
        match geometry {
            Geometry::Polygon(p) => polygons.push(p),
            Geometry::MultiPolygon(mp) => polygons.extend(mp.0),
            Geometry::Line(l) => lines.push(l),
            Geometry::Point(p) => points.push(p),
            Geometry::MultiPoint(mp) => points.extend(mp.0),
            _ => return Err(Error::JSONTypeException("Only polygons lines and points are allowed".to_string())),
        }
    }

    let polygon = polygons
        .into_iter()
        .fold(MultiPolygon::new(vec![]), |r, p| {
            r.union(&MultiPolygon::new(vec![p]))
        });

    let features = vec![Feature::from(geojson::Geometry::from(&polygon))];

    event.geojson_geometry = FeatureCollection::from_iter(features.into_iter());

    Ok(())
}


