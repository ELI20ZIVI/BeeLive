import 'package:desktop_app/src/data_transfer_objects/event.dart';

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
  /// Returns a boolean representing if the event has been accepted or not.
  Future<bool> submitNewEvent(Event event);

  /// Creates the default client.
  factory Client() {
    return implementation;
  }
}

