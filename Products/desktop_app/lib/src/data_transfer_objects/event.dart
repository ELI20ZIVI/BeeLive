import 'dart:convert';

import 'package:bson/bson.dart';
import 'package:desktop_app/src/data_transfer_objects/category.dart';
import 'package:desktop_app/src/data_transfer_objects/risk_level.dart';
import 'package:geojson_vi/geojson_vi.dart';

import 'package:json_annotation/json_annotation.dart';
import 'package:desktop_app/src/data_transfer_objects/category.dart';
import 'package:desktop_app/src/data_transfer_objects/nullable_datetime_range.dart';
import 'package:desktop_app/src/data_transfer_objects/risk_level.dart';

import 'nullable_datetime_range.dart';

part 'event.g.dart';

extension type EventId(int _id) {
  factory EventId.fromJson(int id) => EventId(id);
  int toJson() => _id;
}

@JsonSerializable()
final class Event with BsonSerializable {
  final EventId id;
  final String title;
  final String summary;
  final Uri? document;
  final NullableDateTimeRange validity;
  final NullableDateTimeRange visibility;
  RiskLevel riskLevel;
  final List<Category> categories;
  final List<SubEvent> subevents;

  @_GeoJSONGeometriesToMap()
  final GeoJSONGeometryCollection? polygons;

  Event({
    required this.id,
    required this.title,
    required this.summary,
    this.document,
    required this.validity,
    required this.visibility,
    required this.riskLevel,
    this.categories = const [],
    this.subevents = const [],
    this.polygons,
  });

  Event.defaultNewEvent(int id) : this(
    id: EventId(id),
    title: "Nuovo Evento",
    summary: "",
    validity: NullableDateTimeRange(),
    visibility: NullableDateTimeRange(),
    riskLevel: RiskLevel.info,
    subevents: [],
  );

  @override
  Map<String, dynamic> get toBson => toJson();

  factory Event.fromJson(Map<String, dynamic> json) => _$EventFromJson(json);
  Map<String, dynamic> toJson() => _$EventToJson(this);
}

@JsonSerializable()
final class SubEvent {
  String title;
  String? description;
  NullableDateTimeRange validity;

  @_GeoJSONFeaturesToMap()
  final GeoJSONFeatureCollection? polygons;

  SubEvent({
    required this.title,
    this.description,
    required this.validity,
    this.polygons,
  });

  SubEvent.defaultNewSubevent() : this(title: "Titolo", validity: NullableDateTimeRange());

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

class _GeoJSONGeometriesToMap
    extends JsonConverter<GeoJSONGeometryCollection, Map<String, dynamic>> {
  const _GeoJSONGeometriesToMap();

  @override
  GeoJSONGeometryCollection fromJson(Map<String, dynamic> json) {
    return GeoJSONGeometryCollection.fromMap(json);
  }

  @override
  Map<String, dynamic> toJson(GeoJSONGeometryCollection object) {
    return object.toMap();
  }
}