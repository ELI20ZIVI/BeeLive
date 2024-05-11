
import 'package:json_annotation/json_annotation.dart';

part 'nullable_datetime_range.g.dart';

@JsonSerializable()
class NullableDateTimeRange {

  DateTime? begin, end;

  NullableDateTimeRange({this.begin, this.end});

  factory NullableDateTimeRange.fromJson(Map<String, dynamic> json) {
    return _$NullableDateTimeRangeFromJson(json);
  }
  Map<String, dynamic> toJson() => _$NullableDateTimeRangeToJson(this);
}