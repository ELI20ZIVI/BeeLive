mod errors;
mod test;

use std::convert::TryInto;
use geo::{BooleanOps, Geometry, MultiPolygon};
use geojson::{FeatureCollection, Feature};
use crate::dao::objects::{Event};
use crate::event_processor::errors::Error;

/// Processes an [event] in order to add self-calculated parameters.
pub fn process(event: &mut Event) -> Result<(), Error> {
    // Aggregates the features.
    // Features converted to geo::Geometry.
    let geometries : Vec<Geometry> = event.subevents
        .iter()
        .flat_map(|se| {
            se.polygons.features.iter().filter_map(|f| {
                f.geometry.as_ref().map(|g| {
                    g.try_into().ok()
                }).flatten()
            })
        }).collect();

    // Splits the various allowed geometries by type.
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

    // Performs the union of the polygons.
    let polygon = polygons
        .into_iter()
        .fold(MultiPolygon::new(vec![]), |r, p| {
            r.union(&MultiPolygon::new(vec![p]))
        });

    // TODO: add also the other geometry types.
    let features = vec![Feature::from(geojson::Geometry::from(&polygon))];

    event.polygons = FeatureCollection::from_iter(features.into_iter());

    Ok(())
}


