import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/authenticator/authenticator.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/dtos/event.dart';

final class PublicWebServerClient implements Client {
  static final defaultUri = Uri(
    scheme: "http",
    host: "192.168.1.15",
    port: 8080,
  );
  static const _pathSegments = ['api', 'v3'];

  final Uri uri;

  final String apiKey;

  final Dio _client;

  factory PublicWebServerClient(String apiKey, [Uri? uri]) {
    return PublicWebServerClient._(apiKey, uri ?? defaultUri);
  }

  PublicWebServerClient._(this.apiKey, this.uri)
      : _client = Dio(BaseOptions(
          baseUrl: uri.toString(),
          queryParameters: {"api_key": apiKey},
        ));

  late final _eventList = FutureProvider<List<Event>>(
    (ref) => _fetchEventList(),
  );

  Future<List<Event>> _fetchEventList() async {
    final uri = this.uri.replace(
      pathSegments: [..._pathSegments, 'events'],
    );

    // Optional authorization token.
    String? token;
    try {
      token = await Authenticator().authorization;
    } catch (e) {
      token = null;
    }

    debugPrint("Authorization: $token");

    final response = await _client.getUri(uri,
        options: Options(headers: {
          if (token != null) "Authorization": token,
        }));

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
  }

  @override
  get eventList => _eventList;
}
