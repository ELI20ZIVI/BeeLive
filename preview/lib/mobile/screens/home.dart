import 'package:beelive/common/dao/dao.dart';
import 'package:beelive/common/map/map.dart';
import 'package:beelive/common/map/tiles.dart';
import 'package:beelive/mobile/widgets/events.dart';
import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';

class HomePage extends StatefulWidget {
  final Dao dao;
  final String title;

  const HomePage({super.key, required this.title, required this.dao});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    final fab = FloatingActionButton(
      onPressed: () {},
      child: const Icon(Icons.settings),
    );

    final polygons = widget.dao.problems().expand((p) => p.polygons).map((p) {
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
