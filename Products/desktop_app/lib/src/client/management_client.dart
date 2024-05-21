import 'dart:convert';

import 'package:desktop_app/src/client/client.dart';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;


import '../data_transfer_objects/event.dart';

// TODO: use Dio
// TODO: this client can attach to multiple endopoints. Only the base uri should be "hardcoded".
/// Client implementation for the real backend.
class ManagementWebServerClient implements Client {
  final Uri uri;
  ManagementWebServerClient(this.uri);

  @override
  Future<http.Response> submitNewEvent(Event event) async {

    debugPrint(json.encode(event.toJson()));

    return await http.post(uri, headers: {"Content-Type": "application/json"}, body: json.encode(event.toJson()));
  }

}
