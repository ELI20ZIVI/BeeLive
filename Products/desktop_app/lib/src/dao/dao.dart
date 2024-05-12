

import 'package:mongo_dart/mongo_dart.dart';

import '../data_transfer_objects/event.dart';

abstract interface class Dao {

  Future<WriteResult> insert_new_event(Event event);

  // TODO: FutureOr when state management is implemented
  List<Event> events();

}