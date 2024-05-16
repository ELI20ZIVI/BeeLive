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
  Future<bool> submitNewEvent(Event event) async {

    debugPrint(json.encode(event.toJson()));

    var response = await http.post(uri, headers: {"Content-Type": "application/json"}, body: json.encode(event.toJson()));
    if (response.statusCode == 200) {
      return true;
    }
    else {
      debugPrint(response.body);
      return false;
    }
  }

}
