import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/client/client.dart';

class HomePage extends ConsumerWidget {
  final Client client;

  HomePage({super.key})
    : client = Client();

  @override
  Widget build(final BuildContext context, final WidgetRef ref) {
    final fab = FloatingActionButton(
      onPressed: () {},
      child: const Icon(Icons.settings),
    );

    final events = ref.watch(client.eventList);

    events.when(data: data, error: error, loading: loading);

    final polygons = events.expand((p) => p.polygons).map((p) {
      return Polygon(
        points: p.points,
        isFilled: true,
        color: Colors.blue.withOpacity(0.4),
        borderColor: Colors.blue,
        borderStrokeWidth: 2,
        isDotted: false,
        rotateLabel: true,
      );
    }).toList(growable: false);

    final map = BeeLiveMap(
      children: [
        openStreetMapTileLayer,
        PolygonLayer(
          polygonCulling: true,
          polygons: polygons,
        ),
      ],
    );

    return Scaffold(
      floatingActionButton: fab,
      floatingActionButtonLocation: FloatingActionButtonLocation.miniEndTop,
      body: map,
      bottomSheet: ProblemsList(dao: widget.dao),
      bottomNavigationBar: BottomNavigationBar(
        items: const [
          BottomNavigationBarItem(
            icon: Icon(Icons.list),
            activeIcon: Icon(Icons.list),
            label: "Eventi",
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.archive),
            label: "Archivio",
          ),
        ],
      ),
    );
  }
}