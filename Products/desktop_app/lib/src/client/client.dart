import 'package:desktop_app/src/data_transfer_objects/event.dart';
import 'package:http/http.dart';

/// Client class for the system backend.
///
/// This is a class delegate to abstract the system APIs.
abstract interface class Client {

  static late final Client implementation;

  /// Overrides the default client for this session.
  static void override(Client implementation_) {
    implementation = implementation_;
  }

  /// Submits a new event to the backend to perform insertion.
  ///
  /// Returns an integer representing the HTTP response's status code.
  Future<Response> submitNewEvent(Event event);

  /// Creates the default client.
  factory Client() {
    return implementation;
  }
}

