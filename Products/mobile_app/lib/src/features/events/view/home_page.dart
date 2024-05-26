import 'package:auto_route/annotations.dart';
import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:mobile_app/src/authenticator/authenticator.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/features/events/view/event_list.dart';
import 'package:mobile_app/src/features/events/view/event_map.dart';

@RoutePage()
class HomePage extends StatefulWidget {
  final Client client;

  HomePage({super.key}) : client = Client();

  @override
  createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  void initState() {
    super.initState();

    SchedulerBinding.instance.addPostFrameCallback((_) {
      Authenticator().authenticateIfAppropriate(context);
    });
  }

  @override
  Widget build(final BuildContext context) {
    final fab = FloatingActionButton(
      onPressed: () {},
      child: const Icon(Icons.settings),
    );

    final map = EventMap(client: widget.client);
    final list = EventList(client: widget.client);

    final navigationBar = BottomNavigationBar(
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
    );

    return Scaffold(
      floatingActionButton: fab,
      floatingActionButtonLocation: FloatingActionButtonLocation.miniEndFloat,
      body: SafeArea(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            AspectRatio(aspectRatio: 1.0, child: map),
            Expanded(child: list),
          ],
        ),
      ),
      bottomNavigationBar: navigationBar,
    );
  }
}