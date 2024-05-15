
import 'package:json_annotation/json_annotation.dart';
import 'package:unixtime/unixtime.dart';

part 'nullable_datetime_range.g.dart';

@JsonSerializable()
@_DateTimeToUnix()
class NullableDateTimeRange {

  DateTime? begin, end;

  NullableDateTimeRange({this.begin, this.end});

  factory NullableDateTimeRange.fromJson(Map<String, dynamic> json) {
    return _$NullableDateTimeRangeFromJson(json);
  }

  Map<String, dynamic> toJson() => _$NullableDateTimeRangeToJson(this);
}

class _DateTimeToUnix
    extends JsonConverter<DateTime, int> {
  const _DateTimeToUnix();

  @override
  DateTime fromJson(int value) {
    return value.toUnixTime();

  }

  @override
  int toJson(DateTime object) {
    return object.unixtime;
  }
}