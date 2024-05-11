import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:desktop_app/src/client/client.dart';
import 'package:desktop_app/src/data_transfer_objects/event.dart';

class ManagementWebServerClient implements Client {
  static final publicWebServerURL = Uri(
    scheme: "https",
    host: "manage.beelive.it",
  );
  static const _pathSegments = ['api', 'v3'];

  final _client = Dio(BaseOptions(
    baseUrl: publicWebServerURL.path,
  ));
}