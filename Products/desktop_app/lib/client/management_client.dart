import 'dart:convert';

import 'package:desktop_app/client/client.dart';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;


import '../data_transfer_objects/event.dart';

// TODO: use Dio
// TODO: this client can attach to multiple endopoints. Only the base uri should be "hardcoded".
/// Client implementation for the real backend.
class ManagementWebServerClient implements Client {
  final String uriPath;

  static const _submitEventUriSegment = "insert_new_event";
  static const _getEventListUriSegment = "events";

  ManagementWebServerClient(this.uriPath);

  @override
  Future<http.Response> submitNewEvent(Event event) async {

    debugPrint(json.encode(event.toJson()));
    var uri = Uri.http(uriPath, _submitEventUriSegment);
    
    return await http.post(uri, headers: {"Content-Type": "application/json"}, body: json.encode(event.toJson()));
  }

  @override
  Future<http.Response> deleteExistingEvent(int eventId) {
    // TODO: implement deleteExistingEvent
    throw UnimplementedError();
  }

  @override
  Future<(http.Response, List<Event>?)> getEventList() async {

    var uri = Uri.parse(uriPath + _getEventListUriSegment);

    var response = await http.get(uri);
    if (response.statusCode == 200) {
      var json = jsonDecode(response.body) as Iterable;
      return (response, List<Event>.from(json.map((e) => {
        Event.fromJson(e)
      })));
    } else {
      return (response, null);
    }
  }

  @override
  Future<http.Response> modifyExistingEvent(Event event) {
    // TODO: implement modifyExistingEvent
    throw UnimplementedError();
  }

}
