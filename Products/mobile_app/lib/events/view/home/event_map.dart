import 'package:flutter/cupertino.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/dtos/event.dart';
import 'package:mobile_app/view/beelive_map.dart';
import 'package:mobile_app/client.dart';
import 'package:mobile_app/events/controller/events_controller.dart';

class EventMap extends ConsumerWidget {
  final List<Event>? events;

  const EventMap({this.events, super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final events = this.events ?? [];

    final polygons =
        <Polygon>[]; /*events.expand((p) => p.polygons).map((p) {
      return Polygon(
        points: p.points,
        isFilled: true,
        color: Colors.blue.withOpacity(0.4),
        borderColor: Colors.blue,
        borderStrokeWidth: 2,
        isDotted: false,
        rotateLabel: true,
      );
    }).toList(growable: false);*/

    final map = BeeLiveMap();

    return map;
  }
}
