import 'dart:convert';
import 'dart:io';

import 'package:beelive_frontend_commons/beelive_frontend_commons.dart';
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
  static const _getEventListUriSegment = "list_events";
  static const _deleteEventUriSegment = "delete_event";
  static const _modifyEventUriSegment = "modify_event";

  ManagementWebServerClient(this.uriPath);

  
  /// Invalidates the token when the [res] has status code 401 or 403.
  ///
  /// This will force tthe application to request the use to reauthenticate.
  Future<void> _invalidateTokenOnError(final http.Response res) async {
    if (res.statusCode == HttpStatus.unauthorized || res.statusCode == HttpStatus.forbidden) {
      await Authenticator().logout();
    }
  } 

  @override
  Future<http.Response> submitNewEvent(Event event) async {

    debugPrint(json.encode(event.toJson()));
    var uri = Uri.parse("$uriPath/$_submitEventUriSegment");
    debugPrint(uri.toString());

    final response = await http.post(uri, headers: {
      "Content-Type": "application/json",
      "Authorization": await getAuthToken(),
    }, body: json.encode(event.toJson()));

    await _invalidateTokenOnError(response);
    return response;
  }

  @override
  Future<http.Response> deleteExistingEvent(int eventId) async {

    var uri = Uri.parse("$uriPath/$_deleteEventUriSegment/$eventId");

    final response = await http.delete(uri, headers: {"Authorization": await getAuthToken()});
    await _invalidateTokenOnError(response);
    return response;
  }

  @override
  Future<(http.Response, List<Event>?)> getEventList() async {

    var uri = Uri.parse("$uriPath/$_getEventListUriSegment");
    debugPrint(uri.toString());

    var response = await http.get(uri, headers: {"Authorization": await getAuthToken(),} );

    if (response.statusCode == 200) {
      debugPrint("${response.statusCode}");
      debugPrint(response.body);
      var json = jsonDecode(response.body) as Iterable;
      return (response, json.map((e) => Event.fromJson(e)).toList());
    } else {
      await _invalidateTokenOnError(response);
      return (response, null);
    }
  }

  @override
  Future<http.Response> modifyExistingEvent(Event event) async {
    debugPrint("Modifying");
    debugPrint(json.encode(event.toJson()));

    var uri = Uri.parse("$uriPath/$_modifyEventUriSegment/${event.id}");
    final response = await http.put(
      uri,
      headers: {"Content-Type": "application/json", "Authorization": await getAuthToken()},
      body: json.encode(event.toJson()),
    );

    await _invalidateTokenOnError(response);

    return response;
  }

  Future<String> getAuthToken() {
    return Authenticator().authorization().then((str) => str ?? "");
  }


}
