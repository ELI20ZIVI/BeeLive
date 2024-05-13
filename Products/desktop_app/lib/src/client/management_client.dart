import 'dart:convert';

import 'package:desktop_app/src/client/client.dart';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;


import '../data_transfer_objects/event.dart';

class ManagementWebServerClient implements Client {
  final Uri uri;
  ManagementWebServerClient(this.uri);

  @override
  Future<bool> submitNewEvent(Event event) async {
    var response = await http.post(uri, headers: {"Content-Type": "application/json"}, body: json.encode(event.toJson()));
    if (response.statusCode == 200) {
      return true;
    }
    else {
      if (kDebugMode) {
        print(response.body);
      }
      return false;
    }
  }

}