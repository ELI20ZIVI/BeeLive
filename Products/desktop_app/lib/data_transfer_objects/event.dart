
import 'package:bson/bson.dart';
import 'package:desktop_app/data_transfer_objects/category.dart';
import 'package:desktop_app/data_transfer_objects/risk_level.dart';
import 'package:geojson_vi/geojson_vi.dart';

import 'package:json_annotation/json_annotation.dart';
import 'package:desktop_app/data_transfer_objects/nullable_datetime_range.dart';


part 'event.g.dart';

extension type EventId(int _id) {
  factory EventId.fromJson(int id) => EventId(id);
  int toJson() => _id;
}

/// The DTO representing an Event.
@JsonSerializable(fieldRename: FieldRename.snake)
final class Event with BsonSerializable {
  final EventId id;
  String title;
  String summary;
  String description;

  /// A remote URL that contains additional information to this event.
  Uri? remoteDocument;
  NullableDateTimeRange validity;
  NullableDateTimeRange visibility;
  RiskLevel riskLevel;
  List<Category> categories;
  List<SubEvent> subevents;

  @_GeoJSONFeaturesToMap()
  final GeoJSONFeatureCollection polygons;

  Event({
    required this.id,
    required this.title,
    required this.summary,
    required this.description,
    this.remoteDocument,
    required this.validity,
    required this.visibility,
    required this.riskLevel,
    this.categories = const [],
    this.subevents = const [],
    required this.polygons,
  });

  Event.defaultNewEvent(int id) : this(
    id: EventId(id),
    title: "Nuovo Evento",
    summary: "",
    description: "",
    validity: NullableDateTimeRange(),
    visibility: NullableDateTimeRange(),
    riskLevel: RiskLevel.info,
    subevents: [],
    polygons: GeoJSONFeatureCollection([]),
  );

  @override
  Map<String, dynamic> get toBson => toJson();

  factory Event.fromJson(Map<String, dynamic> json) => _$EventFromJson(json);
  Map<String, dynamic> toJson() => _$EventToJson(this);
}

/// The DTO representing a SubEvent.
@JsonSerializable()
final class SubEvent {
  String title;
  String? description;
  NullableDateTimeRange validity;

  @_GeoJSONFeaturesToMap()
  GeoJSONFeatureCollection polygons;

  SubEvent({
    required this.title,
    this.description,
    required this.validity,
    required this.polygons,
  });

  SubEvent.defaultNewSubevent() : this(title: "Titolo", validity: NullableDateTimeRange(), polygons: GeoJSONFeatureCollection([]));

  factory SubEvent.fromJson(Map<String, dynamic> json) {
    return _$SubEventFromJson(json);
  }
  Map<String, dynamic> toJson() => _$SubEventToJson(this);
}

class _GeoJSONFeaturesToMap
    extends JsonConverter<GeoJSONFeatureCollection, Map<String, dynamic>> {
  const _GeoJSONFeaturesToMap();

  @override
  GeoJSONFeatureCollection fromJson(Map<String, dynamic> json) {
    return GeoJSONFeatureCollection.fromMap(json);
  }

  @override
  Map<String, dynamic> toJson(GeoJSONFeatureCollection object) {
    return object.toMap();
  }
}

