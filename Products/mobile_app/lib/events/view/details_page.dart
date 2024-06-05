import 'package:auto_route/annotations.dart';
import 'package:flutter/material.dart' hide ErrorWidget;
import 'package:flutter/scheduler.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:beelive_frontend_commons/beelive_frontend_commons.dart';
import 'package:mobile_app/events/controller/events_controller.dart';
import 'package:mobile_app/events/view/details/details_widget.dart';
import 'package:mobile_app/events/view/details/error_widget.dart';
import 'package:mobile_app/events/view/details/loading_widget.dart';
import 'package:mobile_app/dtos/event.dart';

@RoutePage()
class DetailsPage extends ConsumerWidget {
  final EventId eventId;

  const DetailsPage({super.key, required this.eventId});

  @override
  Widget build(final BuildContext context, final WidgetRef ref) {
    final eventListFuture = ref.watch(EventsController.instance().details(eventId));

    final body = eventListFuture.when(
      skipLoadingOnRefresh: false,
      data: (data) => DetailsWidget(event: data),
      error: (error, st) {
        if (error is! TokenRefreshFailureException &&
            error is! AuthenticationNotAskedException) {
          return ErrorWidget(error, st);
        } else {
          _asyncRequestLogin(context, ref);
        }
        return const LoadingWidget();
      },
      loading: () => const LoadingWidget(),
    );

    return Scaffold(
      body: SafeArea(child: body),
    );
  }
  
  void _asyncRequestLogin(final BuildContext context, final WidgetRef ref) {
    SchedulerBinding.instance.addPostFrameCallback((_) async {
      await Authenticator().authenticateIfAppropriate(context);
      ref.invalidate(EventsController.instance().list);
    });
  }

}

