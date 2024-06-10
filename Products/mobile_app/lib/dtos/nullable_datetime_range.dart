
import 'package:intl/intl.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:unixtime/unixtime.dart';

part 'nullable_datetime_range.g.dart';

/// Represention of a range of nullable datetimes.
@JsonSerializable()
@_DateTimeToUnix()
class NullableDateTimeRange {

  DateTime? begin, end;

  NullableDateTimeRange({this.begin, this.end});

  factory NullableDateTimeRange.fromJson(Map<String, dynamic> json) {
    return _$NullableDateTimeRangeFromJson(json);
  }

  Map<String, dynamic> toJson() => _$NullableDateTimeRangeToJson(this);

  @override
  String toString() {
    final begin = this.begin;
    final end = this.end;

    final DateFormat formatter;
    final DateFormat suffixFormatter;
    if (begin == null || end == null || begin.year != end.year) {
      formatter = DateFormat("HH:mm dd/MM/yyyy");
      suffixFormatter = DateFormat("");
    } else if (begin.year == end.year && begin.month == end.month && begin.day == end.day) {
      // 14:00 - 16:00 15/03/2024
      formatter = DateFormat("HH:mm");
      suffixFormatter = DateFormat(" dd/MM/yyyy");
    } else {
      // 22:00 14/03 - 19:00 15/03/2024
      formatter = DateFormat("HH:mm dd/MM");
      suffixFormatter = DateFormat("/yyyy");
    }

    final fmtBegin = begin == null ? "..." : formatter.format(begin);
    final fmtEnd = end == null ? "..." : formatter.format(end);
    final commonDate = begin ?? end;
    final fmtSuffix = commonDate == null ? "" : suffixFormatter.format(commonDate);

    return "$fmtBegin - $fmtEnd$fmtSuffix";
  }
}


/// Converts a [DateTime] to an Unix time for transfer purposes.
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
