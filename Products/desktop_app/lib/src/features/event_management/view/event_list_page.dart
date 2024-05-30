


import 'package:desktop_app/src/client/client.dart';
import 'package:desktop_app/src/data_transfer_objects/risk_level.dart';
import 'package:desktop_app/src/features/event_management/view/geojson_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:geojson_vi/geojson_vi.dart';
import 'package:http/http.dart' as http;
import 'package:latlong2/latlong.dart';

import '../../../data_transfer_objects/event.dart';
import '../map/map.dart';
import '../map/tiles.dart';
import 'category_picker.dart';
import 'datetime_range_picker.dart';
import 'event_manager_page.dart';

/// The screen for visualizing the management of a single event.
///
/// Can be used both for creation and modification.
class EventListScreen extends StatelessWidget {

  const EventListScreen({
    super.key,
  });

  @override
  Widget build(BuildContext context) {

    var event = Event.defaultNewEvent(0);

    return ListView(children: [
      EventListElementWidget(event: Event.defaultNewEvent(0)),
    ]);
  }
}

class EventListElementWidget extends StatelessWidget {

  final Event event;

  static const SizedBox separator = SizedBox(width: 20);

  const EventListElementWidget({super.key, required this.event});

  @override
  Widget build(BuildContext context) {
    return ListTile(
        leading: Text("[ID: ${event.id}]"),
        title: Row(children: [
          Text(event.title),
          separator,
          FilledButton(child: const Icon(FluentIcons.edit), onPressed: () {}),
          Button(child: const Icon(FluentIcons.delete), onPressed: () {}),
    ],),
        onPressed: () {},
      );
  }
}
