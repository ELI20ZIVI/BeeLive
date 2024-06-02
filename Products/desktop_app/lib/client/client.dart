import 'package:desktop_app/data_transfer_objects/event.dart';
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

  Future<Response> modifyExistingEvent(Event event);

  Future<Response> deleteExistingEvent(int eventId);

  /// tries to get the event list from the server. If the request succeeds (status == 200) the function tries to decode json values into a List<Event>, which will be null if it fails to do so.
  Future<(Response, List<Event>?)> getEventList();

  /// Creates the default client.
  factory Client() {
    return implementation;
  }
}

