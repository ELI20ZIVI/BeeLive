import 'dart:async';

import 'package:casdoor_flutter_sdk/casdoor_flutter_sdk.dart';
import 'package:flutter/widgets.dart';
import 'package:mobile_app/src/authenticator/authenticator.dart';
import 'package:mobile_app/src/authenticator/authentication_manager.dart';
import 'package:mobile_app/src/authenticator/tokens_manager.dart';

/// Authenticator class to perform authentication of the user via Casdoor.
///
/// This is a wrapper class to allow usage of common interface for easy substitution.
final class CasdoorAuthenticator implements Authenticator {

  final TokensManager _tokensManager;
  final AuthenticationManager _authenticationManager;

  CasdoorAuthenticator(final AuthConfig config)
      : this._(Casdoor(config: config));

  CasdoorAuthenticator._(final Casdoor casdoor)
      : _casdoor = casdoor,
        _tokensManager = TokensManager(casdoor),
        _authenticationManager = AuthenticationManager(casdoor);

  CasdoorAuthenticator.defaultConfig({
    String? clientId,
    Uri? serverUrl,
    String? organizationName,
    String? appName,
    String? redirectUri,
    String? callbackUrlScheme,
  }) : this(AuthConfig(
          clientId: clientId ?? "712b8aaffd9c4c71ab7a",
          serverUrl: serverUrl?.toString() ?? "casdoor.beelive.it",
          organizationName: organizationName ?? "beelive",
          appName: appName ?? "public_beelive",
          redirectUri: redirectUri ?? "casdoor://callback",
          callbackUrlScheme: callbackUrlScheme ?? "casdoor",
        ));

  @override
  Future<bool> authenticate(final BuildContext context) {
    return _authenticationManager
        .authenticate(context)
        .then((code) => code != null);
  }

  @override
  FutureOr<bool> authenticateIfAppropriate(
    final BuildContext context, {
    bool hasToken = true,
  }) {
    return _authenticationManager
        .authenticateIfAppropriate(context, hasToken)
        .then((code) => code != null);
  }

  @override
  Future<String?> get authorization async {
    var tokens = await _tokensManager.tokens;

    if (tokens == null) {
      final code = await _authenticationManager.code;
      if (code == null) {
        return null;
      }

      tokens = await _tokensManager.fromAuthorizationCode(code);
    }

    return "${tokens.tokenType} ${tokens.accessToken}";
  }
}
