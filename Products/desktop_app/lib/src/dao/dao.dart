

import '../data_transfer_objects/event.dart';

abstract interface class Dao {

  // TODO: FutureOr when state management is implemented
  List<Event> events();

}