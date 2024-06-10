import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/dtos/event.dart';
import 'package:mobile_app/utils.dart';
import 'package:mobile_app/view/beelive_map.dart';

class EventMap extends ConsumerWidget {
  final List<Event>? events;

  const EventMap({this.events, super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final events = this.events ?? [];

    final polygons = events.expand((event) {
      final parser = defaultGeoJsonParser;
      parser.parseGeoJson(event.polygons.toMap());
      return parser.polygons;
    }).toList(growable: false);

    final map = BeeLiveMap(polygons: polygons);

    return map;
  }
}
