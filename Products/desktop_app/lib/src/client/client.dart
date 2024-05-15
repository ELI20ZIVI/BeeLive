import 'package:flutter_riverpod/flutter_riverpod.dart';
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

  Future<bool> submitNewEvent(Event event);

  /// Creates the default client.
  factory Client() {
    return implementation;
  }
}

