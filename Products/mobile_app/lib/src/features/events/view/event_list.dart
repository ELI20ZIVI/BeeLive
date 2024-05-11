import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_placeholder_textlines/flutter_placeholder_textlines.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/dtos/event.dart';

/// Widget class for visualizing an asynchronous list of events.
///
/// During loading a list of placeholders is shown.
/// In case of errors, the error is shown.
class EventList extends ConsumerWidget {
  final Client client;

  const EventList({super.key, required this.client});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final events = ref.watch(client.eventList.future);
    debugPrint(events.toString());

    // TODO: use riverpod
    /*return events.when(
      data: _EventList.new,
      error: _Error.new,
      loading: () => const _EventList(null),
    );*/

    return FutureBuilder(
      future: events,
      builder: (ctx, snap) {
        if (snap.hasError) {
          return _Error(snap.error!, snap.stackTrace!);
        } else {
          return _EventList(snap.data);
        }
      },
    );
  }
}

/// Widget class for visualizing a list of events.
///
/// When [list] is `null`, a list of text placeholders is shown.
class _EventList extends StatelessWidget {
  final List<Event>? list;

  const _EventList(this.list, {super.key});

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: list?.length,
      itemBuilder: (ctx, index) => _EventHeader(event: list?[index]),
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

    if (event == null) {
      // Placeholders
      title = const PlaceholderLines(count: 1);
      subtitle = const PlaceholderLines(count: 1);
    } else {
      // Effective values
      title = Text(event.title);
      subtitle = Text(event.summary);
    }

    return ListTile(
      title: title,
      subtitle: subtitle,
    );
  }
}

/// Widget class for visualizing a loading error.
class _Error extends StatelessWidget {
  final Object error;
  final StackTrace stacktrace;

  const _Error(this.error, this.stacktrace, {super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20),
      child: Text('${error.toString()}\n$stacktrace'),
    );
  }
}
