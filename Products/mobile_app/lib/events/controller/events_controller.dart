import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/client.dart';
import 'package:mobile_app/dtos/event.dart';

abstract interface class EventsController {
  static final EventsController _instance = _EventsController();

  EventsController._();
  factory EventsController.instance() => _instance;

  /// #### Exceptions
  /// The future throws [DioException] in case of networking errors.\
  /// The future throws [JsonValidationError] in case of invalid json.\
  /// The future throws [HttpStatusException] in case of status code
  /// different from [HttpStatus.ok].\
  /// The future throws [TokenRefreshFailureException] in case of errors during token
  /// refreshing.\
  /// The future throws [AuthenticationNotAskedException].
  FutureProvider<List<Event>> get list;

  /// #### Exceptions
  /// The future throws [DioException] in case of networking errors.\
  /// The future throws [JsonValidationError] in case of invalid json.\
  /// The future throws [HttpStatusException] in case of status code
  /// different from [HttpStatus.ok].\
  /// The future throws [TokenRefreshFailureException] in case of errors during token
  /// refreshing.\
  /// The future throws [AuthenticationNotAskedException].
  FutureProviderFamily<Event, EventId> get details;
}

final class _EventsController implements EventsController {
  @override
  final FutureProvider<List<Event>> list =
      FutureProvider((ref) => Client().getEventList());

  @override
  final FutureProviderFamily<Event, EventId> details =
      FutureProvider.family((ref, id) => Client().getEventDetails(id));
}
