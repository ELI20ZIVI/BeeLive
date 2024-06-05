import 'dart:async';

import 'package:flutter/widgets.dart';
export './authentication_provider.dart';
import '../authenticator.dart';
import './jwt_authenticator/authentication_manager.dart';
import './jwt_authenticator/tokens_manager.dart';

/// Authenticator class to perform authentication of the user via a JWT provider.
///
/// This is a class that allows the management of JWT token with automatic refresh.
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
  Future<bool> authenticateIfAppropriate(final BuildContext context) async {
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

  /// #### Exceptions
  /// Throws [TokenRefreshFailureException] in case of errors during token
  /// refreshing.\
  /// Throws [AuthenticationNotAskedException].
  @override
  Future<String?> authorization() async {
    var tokens = await _tokensManager.tokens;

    debugPrint("got tokens: $tokens");

    if (tokens == null) {
      final code = _authenticationManager.code();
      if (code == null) {
        return null;
      }

      tokens = await _tokensManager.fromAuthorizationCode(code);
    }

    return "${tokens.tokenType} ${tokens.accessToken}";
  }
  

  /// Invalidates the token.
  ///
  /// This method should be called in case of 401 and 403 errors
  /// in order to force re-authentication of the user.
  Future<void> invalidateToken() {
    return _tokensManager.invalidate();
  }
}
