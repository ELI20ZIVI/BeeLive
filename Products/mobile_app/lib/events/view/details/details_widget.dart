import 'package:flutter/material.dart';
import 'package:flutter_map_geojson/flutter_map_geojson.dart';
import 'package:mobile_app/dtos/event.dart';
import 'package:mobile_app/dtos/nullable_datetime_range.dart';
import 'title_widget.dart';

import '../../../view/beelive_map.dart';

// Main widget that displays the details of an event
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
    final Map<String, dynamic> polygons;
    final page = controller.hasClients ? controller.page?.toInt() ?? 0 : 0;

    // Determine which polygons to display based on the current page
    if (page == 0) {
      polygons = widget.event.polygons.toMap();
    } else {
      polygons = widget.event.subevents![page - 1].polygons.toMap();
    }

    // Parse the polygons using GeoJsonParser
    final parser = GeoJsonParser();
    parser.parseGeoJson(polygons);

    // Create a map widget with the parsed polygons
    final map = BeeLiveMap(
      polygons: parser.polygons,
    );

    // Create a PageView to display the main event and its sub-events
    final subEventView = PageView(
      controller: controller,
      children: [
        // Main event details
        _EventDetails(event: widget.event),
        // Sub-event details
        ...widget.event.subevents?.map((e) => _SubEventDetails(event: e)) ?? []
      ],
    );

    // Layout for the DetailsWidget
    return Column(
      children: [
        // Display the map with a 4:3 aspect ratio
        AspectRatio(
          aspectRatio: 4 / 3,
          child: map,
        ),
        // Display the PageView containing event details
        Expanded(
          child: subEventView,
        ),
      ],
    );
  }
}

// Widget to display details of a sub-event
class _SubEventDetails extends StatelessWidget {
  const _SubEventDetails({required this.event});

  final SubEvent event;

  @override
  Widget build(BuildContext context) {
    // Use the shared function to build the event details
    return buildEventDetails(
      title: event.title,
      validity: event.validity,
      description: event.description,
    );
  }
}

// Widget to display details of the main event
class _EventDetails extends StatelessWidget {
  const _EventDetails({required this.event});

  final Event event;

  @override
  Widget build(BuildContext context) {
    // Use the shared function to build the event details
    return buildEventDetails(
      title: event.title,
      validity: event.validity,
      description: event.description ?? "",
    );
  }
}

// Function to build the event details widget
Widget buildEventDetails({
  required String title,
  required NullableDateTimeRange validity,
  String? description,
}) {

  String titleDateTime = validity.toString();

  // Build and return the event details widget
  return ListView(
    children: [
      // Title and time information
      TitleWidget(
        title: title,
        caption: titleDateTime,
      ),
      // Event description
      Padding(
        padding: const EdgeInsets.all(20),
        child: Text(
          description ?? "",
          textAlign: TextAlign.justify,
        ),
      ),
    ],
  );
}
