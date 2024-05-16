

import 'package:flutter/cupertino.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/dtos/event.dart';
import 'package:mobile_app/src/features/events/view/beelive_map.dart';

class EventMap extends ConsumerWidget {
  final Client client;

  const EventMap({super.key, required this.client});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final List<Event> events = ref.watch(client.eventList).valueOrNull ?? [];

    final polygons = <Polygon>[]; /*events.expand((p) => p.polygons).map((p) {
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