import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_placeholder_textlines/flutter_placeholder_textlines.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/client.dart';
import 'package:mobile_app/dtos/event.dart';
import 'package:mobile_app/events/controller/events_controller.dart';
import 'package:mobile_app/routes/routes.gr.dart';

/// Widget class for visualizing an asynchronous list of events.
///
/// During loading a list of placeholders is shown.
/// In case of errors, the error is shown.
class EventList extends StatelessWidget {
  final List<Event>? events;

  const EventList({this.events, super.key});

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: events?.length,
      itemBuilder: (ctx, index) => _EventHeader(event: events?[index]),
    );
  }
}

/// Widget class for visualizing the header of an event.
///
/// When [event] is `null`, a text placeholder is shown.
class _EventHeader extends StatelessWidget {
  final Event? event;

  _EventHeader({required this.event})
      : super(key: event == null ? null : ValueKey(event));

  @override
  Widget build(BuildContext context) {
    final Event? event = this.event;

    final Widget title, subtitle;

    final Function()? onTap;

    if (event == null) {
      // Placeholders
      title = const PlaceholderLines(count: 1);
      subtitle = const PlaceholderLines(count: 1);
      onTap = null;
    } else {
      // Effective values
      title = Text(event.title);
      subtitle = Text(event.summary);
      onTap = () => context.router.push(DetailsRoute(eventId: event.id));
    }

    return ListTile(
      title: title,
      subtitle: subtitle,
      onTap: onTap,
    );
  }
}
