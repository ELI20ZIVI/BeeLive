// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'event.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Event _$EventFromJson(Map<String, dynamic> json) => Event(
      id: EventId.fromJson((json['id'] as num).toInt()),
      title: json['title'] as String,
      summary: json['summary'] as String,
      description: json['description'] as String,
      remoteDocument: json['remote_document'] == null
          ? null
          : Uri.parse(json['remote_document'] as String),
      validity: NullableDateTimeRange.fromJson(
          json['validity'] as Map<String, dynamic>),
      visibility: NullableDateTimeRange.fromJson(
          json['visibility'] as Map<String, dynamic>),
      riskLevel: $enumDecode(_$RiskLevelEnumMap, json['riskLevel']),
      categories: (json['categories'] as List<dynamic>?)
              ?.map((e) => Category.fromJson(e as Map<String, dynamic>))
              .toList() ??
          const [],
      subevents: (json['subevents'] as List<dynamic>?)
              ?.map((e) => SubEvent.fromJson(e as Map<String, dynamic>))
              .toList() ??
          const [],
      polygons: const _GeoJSONFeaturesToMap()
          .fromJson(json['polygons'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$EventToJson(Event instance) => <String, dynamic>{
      'id': instance.id,
      'title': instance.title,
      'summary': instance.summary,
      'description': instance.description,
      'remote_document': instance.remoteDocument?.toString(),
      'validity': instance.validity,
      'visibility': instance.visibility,
      'riskLevel': _$RiskLevelEnumMap[instance.riskLevel]!,
      'categories': instance.categories,
      'subevents': instance.subevents,
      'polygons': const _GeoJSONFeaturesToMap().toJson(instance.polygons),
    };

const _$RiskLevelEnumMap = {
  RiskLevel.info: 0,
  RiskLevel.warning: 50,
  RiskLevel.alert: 100,
};

SubEvent _$SubEventFromJson(Map<String, dynamic> json) => SubEvent(
      title: json['title'] as String,
      description: json['description'] as String?,
      validity: NullableDateTimeRange.fromJson(
          json['validity'] as Map<String, dynamic>),
      polygons: const _GeoJSONFeaturesToMap()
          .fromJson(json['polygons'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$SubEventToJson(SubEvent instance) => <String, dynamic>{
      'title': instance.title,
      'description': instance.description,
      'validity': instance.validity,
      'polygons': const _GeoJSONFeaturesToMap().toJson(instance.polygons),
    };
