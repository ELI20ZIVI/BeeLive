import 'dart:convert';

import 'package:desktop_app/src/client/client.dart';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;


import '../data_transfer_objects/event.dart';

// TODO: use Dio
// TODO: this client can attach to multiple endopoints. Only the base uri should be "hardcoded".
/// Client implementation for the real backend.
class ManagementWebServerClient implements Client {
  final String uriPath;

  static const _submitEventUriSegment = "insert_new_event";
  static const _getEventListUriSegment = "list_events";
  static const _deleteEventUriSegment = "delete_event";
  static const _modifyEventUriSegment = "modify_event";

  ManagementWebServerClient(this.uriPath);

  @override
  Future<http.Response> submitNewEvent(Event event) async {

    debugPrint(json.encode(event.toJson()));
    var uri = Uri.parse(uriPath + _submitEventUriSegment);

    return await http.post(uri, headers: {"Content-Type": "application/json", "Authorization": "Bearer ${getAuthToken()}"}, body: json.encode(event.toJson()));
  }

  @override
  Future<http.Response> deleteExistingEvent(int eventId) async {

    var uri = Uri.http(uriPath, "$_deleteEventUriSegment/$eventId");

    return await http.delete(uri, headers: {"Authorization": "Bearer ${getAuthToken()}"});
  }

  @override
  Future<(http.Response, List<Event>?)> getEventList() async {

    var uri = Uri.parse(uriPath + _getEventListUriSegment);

    var response = await http.get(uri, headers: {"Authorization": "Bearer ${getAuthToken()}",} );

    if (response.statusCode == 200) {
      debugPrint("${response.statusCode}");
      debugPrint("${response.body}");
      var json = jsonDecode(response.body) as Iterable;
      return (response, List<Event>.from(json.map((e) => {
        Event.fromJson(e)
      })));
    } else {
      return (response, null);
    }
  }

  @override
  Future<http.Response> modifyExistingEvent(Event event) async {
    debugPrint(json.encode(event.toJson()));
    var uri = Uri.http(uriPath, "$_modifyEventUriSegment/${event.id}");
    return await http.put(uri, headers: {"Content-Type": "application/json", "Authorization": "Bearer ${getAuthToken()}"}, body: json.encode(event.toJson()));
  }

  String getAuthToken() {
    return "asdfasdf";
  }

}
