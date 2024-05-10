use std::iter;
use geo::{BooleanOps, Geometry, GeometryCollection, MultiPolygon, Polygon};
use crate::dao::objects::Event;

pub trait EventProcessor {

    fn process(&self, event: &mut Event) -> ();

}

pub struct EventProcessorImpl {}

impl EventProcessorImpl {
    
    pub const fn new() -> Self {
        EventProcessorImpl{}
    }

    fn polygons_union(geometries: Vec<&Geometry>) -> GeometryCollection {
        let polygons = geometries
            .iter()
            .filter_map(|g| match g {
                Geometry::Polygon(p) => Some(p),
                _ => None
            });

        let multi_polygons = geometries
            .iter()
            .filter_map(|g| match g {
                Geometry::MultiPolygon(p) => Some(p),
                _ => None
            });

        let others = geometries
            .iter()
            .filter_map(|g| match g {
                Geometry::Polygon(_) | Geometry::MultiPolygon(_) => None,
                g => Some(g)
            });

        let polygons = polygons
            .fold(MultiPolygon::new(vec![]), |p1, p2| {
                // TODO: try to avoid clone (Requires BooleanOps to allow Multipolygon.union(&Polygon))
                p1.union(&MultiPolygon::new(vec![p2.clone()]))
            });

        let multi_polygons = multi_polygons
            .fold(MultiPolygon::new(vec![]), |r, p| {
                // TODO: check if internal multipolygon union is performed
                r.union(p)
            });

        let polygons = polygons.union(&multi_polygons);

        let mut result = GeometryCollection::from_iter(others.map(|g| (*g).clone()));

        if !polygons.0.is_empty() {
            result.0.push(Geometry::MultiPolygon(polygons));
        }

        result
    }

}

impl EventProcessor for EventProcessorImpl {
    fn process(&self, event: &mut Event) -> () {
        let geometries = event.subevents
            .iter()
            .flat_map(|se| {
                se.geometry.iter()
            })
            .collect();

        event.geojson_geometry = Self::polygons_union(geometries);
    }
}