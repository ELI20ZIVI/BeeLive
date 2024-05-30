// Guarda preview


import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
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
  
  @override
  Widget build(final BuildContext context) {
    // TODO: layout
    // TODO: map
    final map = BeeLiveMap();
    /*
    AspectRatio(
        aspectRatio: 4 / 3,
        child: map,
      )
    */

    final subEventView = PageView(
      children: [
        // TODO: Widget overview
        ...widget.event.events.map((e) => _SubEventDetails(event: e))
      ],
    );

    return subEventView;
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
      title,
      content,
    ]);
  }
}

// TODO: Poligoni della mappa aggiornati muovendo tra i sottoeventi