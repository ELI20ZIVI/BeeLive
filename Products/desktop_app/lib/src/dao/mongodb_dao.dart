import 'package:desktop_app/src/dao/dao.dart';
import 'package:desktop_app/src/data_transfer_objects/event.dart';
import 'package:flutter/foundation.dart';
import 'package:mongo_dart/mongo_dart.dart';

class MongodbDao implements Dao {

  late Db db;

  static Future<MongodbDao> create() async {
    var mgb_dao = MongodbDao();
    mgb_dao.db = (kDebugMode) ? Db("mongodb://localhost:27017/beelive_develop") : Db("mongodb://localhost:27017/beelive_production");
    await mgb_dao.db.open();
    return mgb_dao;
  }


  @override
  List<Event> events() {
    throw UnimplementedError();
  }

  @override
  Future<WriteResult> insert_new_event(Event event) {
    var events_collection = db.collection("events");
    return events_collection.insertOne(event.toJson());
  }



}