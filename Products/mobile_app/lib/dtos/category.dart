
import 'package:json_annotation/json_annotation.dart';

part 'category.g.dart';

// TODO: decide how to represent ids.
extension type const CategoryId(int _id) {
  factory CategoryId.fromJson(int id) => CategoryId(id);
  int toJson() => _id;
}

// TODO: complete
/// Category tag to be associated to events.
///
/// Necessary in order to build filters and clearances.
@JsonSerializable()
class Category {
  final CategoryId id;

  const Category({
    required this.id,
  });

  factory Category.fromJson(Map<String, dynamic> json) {
    return _$CategoryFromJson(json);
  }
  Map<String, dynamic> toJson() => _$CategoryToJson(this);
}
