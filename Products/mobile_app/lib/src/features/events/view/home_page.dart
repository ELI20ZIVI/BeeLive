import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/features/events/view/event_list.dart';

class HomePage extends StatelessWidget {
  final Client client;

  HomePage({super.key}) : client = Client();

  @override
  Widget build(final BuildContext context) {
    final fab = FloatingActionButton(
      onPressed: () {},
      child: const Icon(Icons.settings),
    );

    const map = SizedBox.expand();
    final list = EventList(client: client);

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
      /*floatingActionButton: fab,
      floatingActionButtonLocation: FloatingActionButtonLocation.miniEndTop,*/
      // bottomSheet: list,
      body: SafeArea(child: list),
      bottomNavigationBar: navigationBar,
    );
  }
}
