// Guarda preview


import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:flutter_map_geojson/flutter_map_geojson.dart';
import 'package:intl/intl.dart';
import 'package:mobile_app/dtos/event.dart';
import 'title_widget.dart';

import '../../../view/beelive_map.dart';

class DetailsWidget extends StatefulWidget {
  final Event event;

  const DetailsWidget({super.key, required this.event});

  @override
  State<DetailsWidget> createState() => _DetailsWidgetState();

}

class _DetailsWidgetState extends State<DetailsWidget> {
  final PageController controller = PageController();
  
  @override
  Widget build(final BuildContext context) {
    final polygons;
    final page = controller.page?.toInt()??0;

    if (page == 0) {
      polygons = widget.event.polygons?.toMap();
    } else {
      polygons = widget.event.events[page-1].polygons;
    }

    final parser = GeoJsonParser();
    if (polygons != null) parser.parseGeoJson(polygons);

    // Ottenimento coordinate della mappa
    final map = BeeLiveMap(
      polygons: parser.polygons,
    );

    final subEventView = PageView(
      children: [
        // Widget dell'evento generale seguito da tutti i sottoeventi
        _EventDetails(event: widget.event),
        ...widget.event.events.map((e) => _SubEventDetails(event: e))
      ],
    );

    // Definizione del layout
    return Column(
      children: [
        AspectRatio(
          aspectRatio: 4 / 3,
          child: map,
        ),
        Expanded(
          child: subEventView,
        ),
      ],
    );
  }

}

class _SubEventDetails extends StatefulWidget {
  const _SubEventDetails({
    required this.event,
  });

  final SubEvent event;

  @override
  State<StatefulWidget> createState() {
    return _SubEventDetailsState();
  }
}

class _SubEventDetailsState extends State<_SubEventDetails> {

  @override
  Widget build(BuildContext context) {
    final begin = widget.event.validity.begin;
    DateFormat("HH:mm dd/MM/YY").format(begin!);
    final end = widget.event.validity.end;
    DateFormat("HH:mm dd/MM/YY").format(end!);

    String date = "Inizio evento: $begin\n Fine evento: $end";

    final title = TitleWidget(
      title: widget.event.title,
      caption: date,
    );

    final content = Padding(
      padding: const EdgeInsets.all(20),
      child: Text(
        widget.event.description ?? "",
        textAlign: TextAlign.justify,
      ),
    );

    return ListView(
      children: [
        title,
        content,
      ],
    );
  }
}

class _EventDetails extends StatefulWidget {
  const _EventDetails({
    required this.event,
  });

  final Event event;

  @override
  State<StatefulWidget> createState() {
    return _EventDetailsState();
  }
}

class _EventDetailsState extends State<_EventDetails>{
  @override
  Widget build(BuildContext context) {
    final begin = widget.event.validity.begin;
    DateFormat("HH:mm dd/MM/YY").format(begin!);
    final end = widget.event.validity.end;
    DateFormat("HH:mm dd/MM/YY").format(end!);

    String date = "Inizio evento: $begin\n Fine evento: $end";

    final title = TitleWidget(
      title: widget.event.title,
      caption: date,
    );

    final content = Padding(
      padding: const EdgeInsets.all(20),
      child: Text(
        widget.event.summary ?? "",
        textAlign: TextAlign.justify,
      ),
    );

    return ListView(
      children: [
        title,
        content,
      ],
    );
  }
}

