
import 'package:bson/bson.dart';

import 'package:json_annotation/json_annotation.dart';



@JsonSerializable()
final class Event with BsonSerializable {
  final String title;
  final String summary;
  final Uri? document;
  final NullableDateTimeRange validity;
  final NullableDateTimeRange visibility;
  final List<Category> categories;
  final List<SubEvent> events;
  final List<PolygonDto> polygons;

  Event._({
    required this.title,
    required this.summary,
    this.document,
    required this.validity,
    required this.visibility,
    this.categories = const [],
    this.events = const [],
    this.polygons = const [],
  });

  @override
  // TODO: implement toBson
  Map<String, dynamic> get toBson => throw UnimplementedError();

}

@JsonSerializable()
final class SubEvent {
  final String title;
  final String? description;
  final NullableDateTimeRange validity;
  final List<LabeledPolygonDto> polygons;

  const SubEvent._({
    required this.title,
    this.description,
    required this.validity,
    this.polygons = const [],
  });
}