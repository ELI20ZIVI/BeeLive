
import 'package:desktop_app/client/client.dart';
import 'package:fluent_ui/fluent_ui.dart';

import '../../../../main.dart';
import '../../../data_transfer_objects/event.dart';

/// The screen for visualizing the management of a single event.
///
/// Can be used both for creation and modification.
class _EventListScreenState extends State<EventListScreen>{
  @override
  Widget build(BuildContext context) {

    Client.implementation.getEventList().then((v) {
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
    });


    return ListView(children: widget.eventList.map((e) {
      return EventListElementWidget(event: e, homePageState: widget.homePageState, refreshHomeScreen: widget.refreshHomeScreen);
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

  final Event event;
  final HomePageState homePageState;
  final Function() refreshHomeScreen;

  static const SizedBox separator = SizedBox(width: 20);

  const EventListElementWidget({super.key, required this.event, required this.homePageState, required this.refreshHomeScreen});

  @override
  Widget build(BuildContext context) {
    return ListTile(
        leading: Text("[ID: ${event.id}]"),
        title: Row(children: [
          Text(event.title),
          separator,
          FilledButton(child: const Icon(FluentIcons.edit), onPressed: () {
            homePageState.selectedEvent = event;
            homePageState.selectedPage = 1; // redirect to event management screen
            refreshHomeScreen();
          }),
          Button(child: const Icon(FluentIcons.delete), onPressed: () {}),
    ],),
        onPressed: () {},
      );
  }
}
