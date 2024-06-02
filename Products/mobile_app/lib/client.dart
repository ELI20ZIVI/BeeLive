import 'dart:async';

import 'package:mobile_app/authenticator/errors.dart';
import 'package:mobile_app/dtos/event.dart';

export 'client/dummy_client.dart';
export 'client/public_webserver_client.dart';

/// Client class for the system backend.
///
/// This is a class delegate to abstract the system APIs.
abstract interface class Client {
  static late final Client _implementation;

  /// Overrides the default client for this session.
  static void override(Client implementation) {
    _implementation = implementation;
  }

  /// Creates the default client.
  factory Client() {
    return _implementation;
  }
  
  /// Fetches the list of events.
  ///
  /// #### Throws
  /// Throws [DioException] in case of networking errors.\
  /// Throws [JsonValidationError] in case of invalid json.\
  /// Throws [HttpStatusException] in case of status code
  /// different from [HttpStatus.ok].\
  /// Throws [TokenRefreshFailureException] in case of errors during token
  /// refreshing.\
  /// Throws [AuthenticationNotAskedException].
  FutureOr<List<Event>> getEventList();

  /// Fetches the details of a single event.
  ///
  /// #### Throws
  /// Throws [DioException] in case of networking errors.\
  /// Throws [JsonValidationError] in case of invalid json.\
  /// Throws [HttpStatusException] in case of status code
  /// different from [HttpStatus.ok].\
  /// Throws [TokenRefreshFailureException] in case of errors during
  /// token refreshing.\
  /// Throws [AuthenticationNotAskedException].
  FutureOr<Event> getEventDetails(EventId id);

}
