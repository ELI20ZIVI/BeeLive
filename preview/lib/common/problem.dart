import 'package:BeeLive/common/map/polygons.dart';
import 'package:BeeLive/common/model/nullable_datetime_range.dart';
import 'package:latlong2/latlong.dart';
import 'package:polybool/polybool.dart';

enum Category {
  viability,
  accident,
  publicTransport,
  parking;

  @override
  String toString() {
    return switch (this) {
      viability => "viabilità",
      accident => "incidente",
      publicTransport => "trasporto pubblico",
      parking => "parcheggi",
    };
  }
}

enum RiskLevel {
  info, warning, alert;
}

final class Event {
  final String title;
  final String? description;
  final NullableDateTimeRange validity;
  final List<LabeledPolygonDto> polygons;

  const Event({
    required this.title,
    this.description,
    required this.validity,
    this.polygons = const [],
  });
}

final class Problem {
  final String title;
  final String summary;
  final Uri? document;
  final NullableDateTimeRange validity;
  final NullableDateTimeRange visibility;
  final List<Category> categories;
  final List<Event> events;
  final List<PolygonDto> polygons;

  Problem({
    required this.title,
    required this.summary,
    this.document,
    required this.validity,
    required this.visibility,
    this.categories = const [],
    this.events = const [],
  }) : polygons = _union(events);

  static Coordinate _latlngToCoord(final LatLng latlng) {
    return Coordinate(latlng.latitude, latlng.longitude);
  }

  static LatLng _coordToLatlng(final Coordinate coord) {
    return LatLng(coord.x, coord.y);
  }

  // TODO: implement
  static List<PolygonDto> _union(final List<Event> events) {
    return events
        .expand((event) => event.polygons)
        .take(1)
        .toList(growable: false);

    final Polygon union = events
        .expand((event) => event.polygons)
        .map(
          (p) => Polygon(
            regions: [
              p.points.map(_latlngToCoord).toList(growable: false),
              ...p.holes
                  .map((h) => h.map(_latlngToCoord).toList(growable: false)),
            ],
          ),
        )
        .reduce((p1, p2) => p1.union(p2));

    throw UnimplementedError("Più avanti.");
  }
}
