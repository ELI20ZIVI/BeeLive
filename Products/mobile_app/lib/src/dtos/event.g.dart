// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'event.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Event _$EventFromJson(Map<String, dynamic> json) => Event(
      id: EventId.fromJson((json['id'] as num).toInt()),
      title: json['title'] as String,
      summary: json['summary'] as String,
      document: json['document'] == null
          ? null
          : Uri.parse(json['document'] as String),
      validity: NullableDateTimeRange.fromJson(
          json['validity'] as Map<String, dynamic>),
      visibility: NullableDateTimeRange.fromJson(
          json['visibility'] as Map<String, dynamic>),
      riskLevel: $enumDecode(_$RiskLevelEnumMap, json['riskLevel']),
      categories: (json['categories'] as List<dynamic>?)
              ?.map((e) => Category.fromJson(e as Map<String, dynamic>))
              .toList() ??
          const [],
      events: (json['events'] as List<dynamic>?)
              ?.map((e) => SubEvent.fromJson(e as Map<String, dynamic>))
              .toList() ??
          const [],
      polygons: _$JsonConverterFromJson<Map<String, dynamic>,
              GeoJSONGeometryCollection>(
          json['polygons'], const _GeoJSONGeometriesToMap().fromJson),
    );

Map<String, dynamic> _$EventToJson(Event instance) => <String, dynamic>{
      'id': instance.id,
      'title': instance.title,
      'summary': instance.summary,
      'document': instance.document?.toString(),
      'validity': instance.validity,
      'visibility': instance.visibility,
      'riskLevel': _$RiskLevelEnumMap[instance.riskLevel]!,
      'categories': instance.categories,
      'events': instance.events,
      'polygons': _$JsonConverterToJson<Map<String, dynamic>,
              GeoJSONGeometryCollection>(
          instance.polygons, const _GeoJSONGeometriesToMap().toJson),
    };

const _$RiskLevelEnumMap = {
  RiskLevel.info: 0,
  RiskLevel.warning: 50,
  RiskLevel.alert: 100,
};

Value? _$JsonConverterFromJson<Json, Value>(
  Object? json,
  Value? Function(Json json) fromJson,
) =>
    json == null ? null : fromJson(json as Json);

Json? _$JsonConverterToJson<Json, Value>(
  Value? value,
  Json? Function(Value value) toJson,
) =>
    value == null ? null : toJson(value);

SubEvent _$SubEventFromJson(Map<String, dynamic> json) => SubEvent(
      title: json['title'] as String,
      description: json['description'] as String?,
      validity: NullableDateTimeRange.fromJson(
          json['validity'] as Map<String, dynamic>),
      polygons: _$JsonConverterFromJson<Map<String, dynamic>,
              GeoJSONFeatureCollection>(
          json['polygons'], const _GeoJSONFeaturesToMap().fromJson),
    );

Map<String, dynamic> _$SubEventToJson(SubEvent instance) => <String, dynamic>{
      'title': instance.title,
      'description': instance.description,
      'validity': instance.validity,
      'polygons':
          _$JsonConverterToJson<Map<String, dynamic>, GeoJSONFeatureCollection>(
              instance.polygons, const _GeoJSONFeaturesToMap().toJson),
    };
