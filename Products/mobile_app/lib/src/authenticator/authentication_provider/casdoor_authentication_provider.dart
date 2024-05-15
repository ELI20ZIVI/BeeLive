import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:casdoor_flutter_sdk/casdoor_flutter_sdk.dart';
import 'package:flutter/widgets.dart';
import 'package:http/http.dart';
import 'package:mobile_app/src/authenticator/authentication_provider/authentication_provider.dart';
import 'package:mobile_app/src/authenticator/authentication_provider/casdoor_token.dart';
import 'package:mobile_app/src/authenticator/tokens_manager.dart';

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
    return _casdoor.isTokenExpired(token);
  }

  @override
  FutureOr<Tokens> refreshToken(
    Token refreshToken, {
    List<Scope> scopes = const [],
  }) async {
    return _casdoor
        .refreshToken(refreshToken, _clientSecret)
        .then(_fromResponse);
  }

  @override
  FutureOr<Tokens> requestAccessToken(AuthorizationCode code) {
    return _casdoor.requestOauthAccessToken(code).then(_fromResponse);
  }

  Tokens _fromResponse(final Response response) {
    if (response.statusCode != HttpStatus.ok) {
      throw HttpException(response.reasonPhrase ?? "Unknown error");
    }

    debugPrint(response.body);

    final json = jsonDecode(response.body);

    // Splits scopes into list.
    json['scopes'] = (json['scope'] as String).split(RegExp(r'\s+'));

    return CasdoorTokens.fromJson(json);
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
    } on CasdoorAuthCancelledException catch (_) {
      debugPrint("Casdoor cancelled");
      return null;
    }

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
}
