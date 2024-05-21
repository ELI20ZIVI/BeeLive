

import 'package:json_annotation/json_annotation.dart';

/// Risk level associated with a specific event.
@JsonEnum()
enum RiskLevel {

  /// This is a "long"-time planned event.
  ///
  /// For example the G7 event.
  @JsonValue(0)
  info,

  /// This is a sudden but not harmful event.
  ///
  /// For example a car crash.
  @JsonValue(50)
  warning,

  /// This is a sudden and harmful event.
  ///
  /// For example an armed person on the loose.
  @JsonValue(100)
  alert,
}