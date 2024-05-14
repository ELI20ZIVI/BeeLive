// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'nullable_datetime_range.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NullableDateTimeRange _$NullableDateTimeRangeFromJson(
        Map<String, dynamic> json) =>
    NullableDateTimeRange(
      begin: json['begin'] == null
          ? null
          : DateTime.parse(json['begin'] as String),
      end: json['end'] == null ? null : DateTime.parse(json['end'] as String),
    );

Map<String, dynamic> _$NullableDateTimeRangeToJson(
        NullableDateTimeRange instance) =>
    <String, dynamic>{
      'begin': instance.begin?.toIso8601String(),
      'end': instance.end?.toIso8601String(),
    };
