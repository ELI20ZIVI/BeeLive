import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:casdoor_flutter_sdk/casdoor_flutter_sdk.dart';
import 'package:flutter/widgets.dart';
import 'package:http/http.dart';
import './casdoor_authentication_provider/casdoor_token.dart';
import '../jwt_authenticator/tokens_manager.dart';
import 'package:beelive_frontend_commons/beelive_frontend_commons.dart';

/// AuthenticationProvider wrapping Casdoor.
final class CasdoorAuthenticationProvider implements AuthenticationProvider {
  final Casdoor _casdoor;
  final String _clientSecret;

  CasdoorAuthenticationProvider(
    final AuthConfig config,
    final String clientSecret,
  )   : _casdoor = Casdoor(config: config),
        _clientSecret = clientSecret;

  CasdoorAuthenticationProvider.defaultConfig({
    required final String clientSecret,
    final String? clientId,
    final Uri? serverUrl,
    final String? organizationName,
    final String? appName,
    final String? redirectUri,
    final String? callbackUrlScheme,
  }) : this(
          AuthConfig(
            clientId: clientId ?? "712b8aaffd9c4c71ab7a",
            serverUrl: serverUrl?.toString() ?? "casdoor.beelive.it",
            organizationName: organizationName ?? "beelive",
            appName: appName ?? "public_beelive",
            redirectUri: redirectUri ?? "casdoor://callback",
            callbackUrlScheme: callbackUrlScheme ?? "casdoor",
          ),
          clientSecret,
        );

  @override
  bool isTokenExpired(Token token) {
    try {
      return _casdoor.isTokenExpired(token);
    } on FormatException catch (_) {
      return true;
    }
  }

  @override
  Future<Tokens> refreshToken(
    Token refreshToken, {
    List<Scope> scopes = const [],
  }) async {
    // TODO: check exceptions
    final response = await _casdoor.refreshToken(
      refreshToken,
      _clientSecret,
      scope: scopes.join(" "),
    );

    return _fromResponse(response);
  }

  @override
  Future<Tokens> requestAccessToken(
    final AuthorizationCode code,
  ) async {
    // TODO: check exceptions
    final response = await _casdoor.requestOauthAccessToken(code);

    return _fromResponse(response);
  }

  /// Parses a [Response] in order to extract the [Tokens].
  ///
  /// Throws [HttpStatusException] if the [response]
  /// doesn't have a [HttpStatus.ok] status.
  ///
  /// Throws [JsonValidationError] if the response doesn't have a valid body.
  Tokens _fromResponse(final Response response) {
    if (response.statusCode != HttpStatus.ok) {
      debugPrint(response.body);
      throw HttpStatusException(
        response.request?.url ?? Uri(),
        response.statusCode,
        response.reasonPhrase ?? "Unknown error",
      );
    }

    debugPrint(response.body);

    final Map<String, dynamic> json;
    try {
      json = jsonDecode(response.body);
    } on FormatException catch (_) {
      throw JsonValidationError(response.request?.url ?? Uri());
    } on TypeError catch (_) {
      throw JsonValidationError(response.request?.url ?? Uri());
    }

    // Splits scopes into list.
    json['scopes'] = (json['scope'] as String).split(RegExp(r'\s+'));

    try {
      return CasdoorTokens.fromJson(json);
    } on TypeError catch (_) {
      throw JsonValidationError(response.request?.url ?? Uri());
    }
  }

  @override
  FutureOr<AuthorizationCode?> showLogin(
    final BuildContext context, {
    final List<Scope> scopes = const [],
  }) async {
    final scope = scopes.join(" ");

    final String callback;
    try {
      debugPrint("Casdoor authentication");
      callback = await _casdoor.showFullscreen(context, scope: scope);
      debugPrint("Casdoor authenticated");
    } on Type catch (t) {
      // NOTE: casdoor lib throws the type [CasdoorAuthCancelledException] instead of an instance.
      if (t == CasdoorAuthCancelledException) {
        debugPrint("Casdoor cancelled");
        return null;
      } else {
        rethrow;
      }
    }

    // [TypeError] in case of validation error.
    return Uri.parse(callback).queryParameters['code'] as String;
  }

  @override
  Tokens tokenFromJson(Map<String, dynamic> json) {
    return CasdoorTokens.fromJson(json);
  }

  @override
  Map<String, dynamic> decodeToken(final Token accessToken) {
    return _casdoor.decodedToken(accessToken);
  }

  @override
  Future<void> logout(final Tokens tokens) async {
    // Creates a new random state to prevent CSRF attacks.
    final randomState = _randomString(4);

    final _ = await _casdoor.tokenLogout(
      tokens.idToken,
      _casdoor.config.redirectUri,
      randomState,
      clearCache: true,
    );
  }

  /// Generates a random string as concatenation of random chunks.\
  /// A chunk is the hex representation of an integer.
  String _randomString([int chunks = 4]) {
    const int maxInt = 1 << 32;
    final rng = Random.secure();

    return Iterable
      .generate(chunks, (_) => rng.nextInt(maxInt))
      .map((n) => n.toRadixString(16).padLeft(8, '0'))
      .join("-");
  }
}
