import 'package:beelive/common/map/polygons.dart';
import 'package:beelive/common/map/tiles.dart';
import 'package:beelive/common/problem.dart';
import 'package:beelive/common/widgets/title_widget.dart';
import 'package:flutter/material.dart';
import 'package:beelive/common/map/map.dart';
import 'package:flutter_map/flutter_map.dart';

class Details extends StatelessWidget {
  final Problem problem;

  const Details({super.key, required this.problem});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(problem.title),
      ),
      body: _SubEventDetails(event: problem.events[3]),
    );
  }
}

class _SubEventDetails extends StatefulWidget {
  const _SubEventDetails({
    required this.event,
  });

  final Event event;

  @override
  State<StatefulWidget> createState() {
    return _SubEventDetailsState();
  }
}

class _SubEventDetailsState extends State<_SubEventDetails> {
  LabeledPolygonDto? _selected;

  @override
  void initState() {
    _selected = widget.event.polygons.firstOrNull;
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final polygons = widget.event.polygons.map((p) {
      return Polygon(
        points: p.points,
        isFilled: true,
        color: Colors.blue.withOpacity(0.2),
        borderColor: Colors.blue,
        borderStrokeWidth: 2,
        isDotted: true,
        rotateLabel: true,
        holePointsList: p.holes,
      );
    }).toList(growable: true);

    final map = BeeLiveMap(
      children: [
        openStreetMapTileLayer,
        PolygonLayer(
          polygonCulling: true,
          polygons: polygons,
        ),
        if (_selected != null) _PolygonDetails(polygon: _selected!),
      ],
    );

    final title = TitleWidget(
      title: widget.event.title,
      // TODO: dynamic
      caption: "8:15 - 10:00 15/03/2024",
    );

    final content = Padding(
      padding: const EdgeInsets.all(20),
      child: Text(
        widget.event.description ?? "",
        textAlign: TextAlign.justify,
      ),
    );

    return ListView(children: [
      AspectRatio(
        aspectRatio: 4 / 3,
        child: map,
      ),
      title,
      content,
    ]);
  }
}

class _PolygonDetails extends StatelessWidget {
  const _PolygonDetails({
    required this.polygon,
  });

  final LabeledPolygonDto polygon;

  @override
  Widget build(BuildContext context) {
    const indicator = Padding(
      padding: EdgeInsets.all(8),
      child: Icon(
        Icons.square,
        color: Colors.blue,
        size: 16,
      ),
    );

    final label = Container(
      margin: const EdgeInsets.all(4),
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(4),
        color: Theme.of(context).cardColor,
      ),
      constraints: const BoxConstraints(
        maxWidth: 200,
      ),
      child: Text(polygon.label),
    );

    return Row(children: [indicator, label]);
  }
}
