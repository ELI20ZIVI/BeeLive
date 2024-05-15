import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:mobile_app/src/authenticator/authentication_provider/authentication_provider.dart';
import 'package:mobile_app/src/authenticator/authenticator.dart';
import 'package:mobile_app/src/authenticator/authentication_manager.dart';
import 'package:mobile_app/src/authenticator/tokens_manager.dart';

/// Authenticator class to perform authentication of the user via Casdoor.
///
/// This is a wrapper class to allow usage of common interface for easy substitution.
final class JwtAuthenticator implements Authenticator {
  final TokensManager _tokensManager;
  final AuthenticationManager _authenticationManager;

  JwtAuthenticator(final AuthenticationProvider provider)
      : _tokensManager = TokensManager(provider),
        _authenticationManager = AuthenticationManager(provider);

  @override
  Future<bool> authenticate(final BuildContext context) {
    return _authenticationManager
        .authenticate(context)
        .then((code) => code != null);
  }

  @override
  FutureOr<bool> authenticateIfAppropriate(final BuildContext context) async {
    final token = await _tokensManager.tokens;

    if (!context.mounted) {
      debugPrint("Unmounted context");
      return false;
    }

    debugPrint("hasToken: ${token != null}");

    final code = await _authenticationManager.authenticateIfAppropriate(
      context,
      token != null,
    );

    return code != null;
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
