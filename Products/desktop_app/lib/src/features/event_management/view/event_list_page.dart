


import 'dart:async';
import 'dart:io';

import 'package:desktop_app/src/client/client.dart';
import 'package:desktop_app/src/data_transfer_objects/risk_level.dart';
import 'package:desktop_app/src/features/event_management/view/geojson_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:geojson_vi/geojson_vi.dart';
import 'package:http/http.dart' as http;
import 'package:latlong2/latlong.dart';

import '../../../../main.dart';
import '../../../data_transfer_objects/event.dart';
import '../map/map.dart';
import '../map/tiles.dart';
import 'category_picker.dart';
import 'datetime_range_picker.dart';
import 'event_manager_page.dart';
import 'no_connection_infobar.dart';

/// The screen for visualizing the management of a single event.
///
/// Can be used both for creation and modification.
class _EventListScreenState extends State<EventListScreen>{


  @override
  void initState() {

    try {
      Client().getEventList().then((v) {
        var (response, list) = v;
        if (list != null) {
          setState(() {
            widget.eventList.clear();
            widget.eventList.addAll(list);
          });
        }
        else {
          debugPrint("[${response.statusCode}]\n${response.reasonPhrase}\n${response.body}");
        }
      }).catchError((err) {
        debugPrint(err.toString());
        displayInfoBar(context, builder: NoConnectionInfobar.noConnectionInfobarBuilder);
      });

      super.initState();
    } catch (e) {
      displayInfoBar(context, builder: NoConnectionInfobar.noConnectionInfobarBuilder);
    }
  }

  refreshEventList() {
    setState(() {
    });
  }

  @override
  Widget build(BuildContext context) {

    return ListView(children: widget.eventList.map((e) {
      return EventListElementWidget(event: e, homePageState: widget.homePageState, refreshHomeScreen: widget.refreshHomeScreen, eventList: widget.eventList, refreshEventList: refreshEventList);
    }).toList());
  }
}

class EventListScreen extends StatefulWidget {

  final HomePageState homePageState;
  final List<Event> eventList = [];

  final Function() refreshHomeScreen;

  EventListScreen({
    super.key, required this.homePageState, required this.refreshHomeScreen,
  });

  @override
  State<StatefulWidget> createState() => _EventListScreenState();

}

class EventListElementWidget extends StatelessWidget {

  final Function() refreshEventList;
  final List<Event> eventList;
  final Event event;
  final HomePageState homePageState;
  final Function() refreshHomeScreen;

  static const SizedBox separator = SizedBox(width: 20);

  const EventListElementWidget({super.key, required this.event, required this.homePageState, required this.refreshHomeScreen, required this.eventList, required this.refreshEventList});

  @override
  Widget build(BuildContext context) {

    bool visible = true;
    late Visibility eventlistelementwidget;

    eventlistelementwidget = Visibility(visible: visible, child: ListTile(
        leading: Text("[ID: ${event.id}]"),
        title: Row(children: [
          Text(event.title),
          separator,
          FilledButton(child: const Icon(FluentIcons.edit), onPressed: () {
            homePageState.selectedEvent = event;
            homePageState.selectedPage = 1; // redirect to event management screen
            refreshHomeScreen();
          }),
          Button(child: const Icon(FluentIcons.delete), onPressed: () async {
            var response = await Client().deleteExistingEvent(event.id as int);
            debugPrint("Deleting event ${event.id} [${event.title}]");
            debugPrint("${response.statusCode}");
            debugPrint("${response.body}");
            eventList.remove(event);
            refreshEventList();
          }),
    ],),
        onPressed: () {},
      ));
    return eventlistelementwidget;
  }
}
