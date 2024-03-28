


import 'package:beelive/common/problem.dart';

typedef Categories = Map<String, dynamic>;

abstract interface class Dao {

  // TODO: FutureOr when state management is implemented
  List<Problem> problems();

  Categories categories();

}