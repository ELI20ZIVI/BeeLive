import 'package:auto_route/annotations.dart';
import 'package:flutter/material.dart' hide ErrorWidget;
import 'package:flutter/scheduler.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/authenticator.dart';
import 'package:mobile_app/authenticator/errors.dart';
import 'package:mobile_app/events/controller/events_controller.dart';
import 'package:mobile_app/events/view/home/event_list.dart';
import 'package:mobile_app/events/view/home/event_map.dart';
import 'package:mobile_app/events/view/home/error_widget.dart';

@RoutePage()
class HomePage extends ConsumerWidget {
  const HomePage({super.key});

  @override
  Widget build(final BuildContext context, final WidgetRef ref) {
    final eventListFuture = ref.watch(EventsController.instance().eventList);

    final fab = FloatingActionButton(
      onPressed: () {},
      child: const Icon(Icons.settings),
    );

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

    final events = eventListFuture.valueOrNull;

    final list = EventList(events: events);
    final map = EventMap(events: events);

    // Inner body to show successful status.
    // Only shown if not errored.
    final innerBody = Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        AspectRatio(aspectRatio: 1.0, child: map),
        Expanded(child: list),
      ],
    );

    final body = eventListFuture.when(
      skipLoadingOnRefresh: false,
      data: (_) => innerBody,
      error: (error, st) {
        if (error is! TokenRefreshFailureException &&
            error is! AuthenticationNotAskedException) {
          return ErrorWidget(error, st);
        } else {
          _asyncRequestLogin(context, ref);
        }
        return innerBody;
      },
      loading: () => innerBody,
    );

    return Scaffold(
      floatingActionButton: fab,
      floatingActionButtonLocation: FloatingActionButtonLocation.miniEndFloat,
      body: SafeArea(child: body),
      bottomNavigationBar: navigationBar,
    );
  }

  void _asyncRequestLogin(final BuildContext context, final WidgetRef ref) {
    SchedulerBinding.instance.addPostFrameCallback((_) async {
      await Authenticator().authenticateIfAppropriate(context);
      ref.invalidate(EventsController.instance().eventList);
    });
  }
}

