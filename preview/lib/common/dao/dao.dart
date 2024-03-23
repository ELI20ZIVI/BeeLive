

import 'dart:async';

import 'package:BeeLive/common/problem.dart';

abstract interface class Dao {

  // TODO: FutureOr when state management is implemented
  List<Problem> problems();

}