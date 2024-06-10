





import 'package:flutter/material.dart';
import 'package:flutter_map_geojson/flutter_map_geojson.dart';

GeoJsonParser get defaultGeoJsonParser {
  return GeoJsonParser(
    defaultPolygonBorderColor: Colors.blue,
    defaultPolygonFillColor: Colors.blue.withOpacity(0.2),
    defaultPolygonBorderStroke: 2,
  );
}
