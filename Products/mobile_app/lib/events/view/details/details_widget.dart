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
    // TODO: Deve dipendere da [controller.page].
    final polygons = widget.event.polygons?.toMap();

    final parser = GeoJsonParser();
    if (polygons != null) parser.parseGeoJson(polygons);

    // TODO: Ottenimento coordinate per la mappa
    final map = BeeLiveMap(
      polygons: parser.polygons,
    );

    final subEventView = PageView(
      children: [
        // TODO: Widget overview
        ...widget.event.events.map((e) => _SubEventDetails(event: e))
      ],
    );

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
    // DateFormat("HH:mm dd/MM/YY").format(date);

    final title = TitleWidget(
      title: widget.event.title,
      // TODO: Controllo valore orari e date
      caption: widget.event.validity.toString(),
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

// TODO: Poligoni della mappa aggiornati muovendo tra i sottoeventi