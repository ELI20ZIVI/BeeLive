import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:beelive_frontend_commons/beelive_frontend_commons.dart';
import 'package:mobile_app/client.dart';
import 'package:mobile_app/dtos/event.dart';

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

  /// Throws [DioException] in case of networking errors.\
  /// Throws [JsonValidationError] in case of invalid json.\
  /// Throws [HttpStatusException] in case of status code
  /// different from [HttpStatus.ok].\
  /// Throws [TokenRefreshFailureException] in case of errors during token
  /// refreshing.\
  /// Throws [AuthenticationNotAskedException].
  @override
  getEventList() async {
    final uri = this.uri.replace(
      pathSegments: [..._pathSegments, 'events'],
    );

    // Optional authorization token.
    String? token = await Authenticator().authorization();

    debugPrint("Authorization: $token");

    // TODO: convert DioException
    final response = await _client.getUri(uri,
        options: Options(headers: {
          if (token != null) "Authorization": token,
        }));

    if (response.statusCode == HttpStatus.ok) {
      final data = response.data;

      if (data is! List) {
        throw JsonValidationError(uri);
      }

      return data
          .cast<Map<String, dynamic>>()
          .map<Event>(Event.fromJson)
          .toList(growable: false);
    } else {
      throw HttpStatusException(
        uri,
        response.statusCode ?? 0,
        response.statusMessage ?? "Unknown error",
      );
    }
  }
  
  /// Fetches the details of a single event.
  ///
  /// #### Throws
  /// Throws [DioException] in case of networking errors.\
  /// Throws [JsonValidationError] in case of invalid json.\
  /// Throws [HttpStatusException] in case of status code
  /// different from [HttpStatus.ok].\
  /// Throws [TokenRefreshFailureException] in case of errors during
  /// token refreshing.\
  /// Throws [AuthenticationNotAskedException].
  @override
  Future<Event> getEventDetails(EventId id) async {
    final uri = this.uri.replace(
      pathSegments: [..._pathSegments, 'events', id.toString()],
    );

    // Optional authorization token.
    String? token = await Authenticator().authorization();

    debugPrint("Authorization: $token");

    debugPrint("Requesting event #$id to $uri");

    // TODO: convert DioException
    final response = await _client.getUri(
      uri,
      options: Options(headers: {
        if (token != null) "Authorization": token,
      }),
    );

    if (response.statusCode == HttpStatus.ok) {
      final data = response.data;

      if (data is! Map<String, dynamic>) {
        throw JsonValidationError(uri);
      }

      return Event.fromJson(data);
    } else {
      throw HttpStatusException(
        uri,
        response.statusCode ?? 0,
        response.statusMessage ?? "Unknown error",
      );
    }
  }
}
