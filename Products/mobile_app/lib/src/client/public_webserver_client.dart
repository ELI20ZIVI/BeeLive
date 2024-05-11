import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/dtos/event.dart';

class PublicWebServerClient implements Client {
  static final defaultUri = Uri(
    scheme: "http",
    host: "192.168.1.15",
    port: 8080,
  );
  static const _pathSegments = ['api', 'v3'];

  final Uri uri;

  final Dio _client;

  factory PublicWebServerClient([Uri? uri]) {
    return PublicWebServerClient._(uri ?? defaultUri);
  }

  PublicWebServerClient._(this.uri)
      : _client = Dio(BaseOptions(
          baseUrl: uri.toString(),
        ));

  @override
  get eventList => FutureProvider<List<Event>>((ref) async {
        final uri = this.uri.replace(
          pathSegments: [..._pathSegments, 'events'],
        );

        final response = await _client.getUri(uri);

        if (response.statusCode == HttpStatus.ok) {
          final data = response.data;

          if (data is! List) {
            throw const HttpException('Cannot validate the response');
          }

          return data
              .cast<Map<String, dynamic>>()
              .map<Event>(Event.fromJson)
              .toList(growable: false);
        } else {
          throw HttpException(response.statusMessage ?? 'Unknown error');
        }
      });
}
