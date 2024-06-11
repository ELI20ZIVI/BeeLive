// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'nullable_datetime_range.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NullableDateTimeRange _$NullableDateTimeRangeFromJson(
        Map<String, dynamic> json) =>
    NullableDateTimeRange(
      begin: _$JsonConverterFromJson<int, DateTime>(
          json['begin'], const _DateTimeToUnix().fromJson),
      end: _$JsonConverterFromJson<int, DateTime>(
          json['end'], const _DateTimeToUnix().fromJson),
    );

Map<String, dynamic> _$NullableDateTimeRangeToJson(
        NullableDateTimeRange instance) =>
    <String, dynamic>{
      'begin': _$JsonConverterToJson<int, DateTime>(
          instance.begin, const _DateTimeToUnix().toJson),
      'end': _$JsonConverterToJson<int, DateTime>(
          instance.end, const _DateTimeToUnix().toJson),
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
