import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/client/dummy_client.dart';
import 'package:mobile_app/src/dtos/event.dart';

/// Client class for the system backend.
///
/// This is a class delegate to abstract the system APIs.
abstract interface class Client {
  static late final Client _implementation;

  static void override(Client implementation) {
    _implementation = implementation;
  }

  factory Client() {
    return _implementation;
  }

  FutureProvider<List<Event>> get eventList;

}
