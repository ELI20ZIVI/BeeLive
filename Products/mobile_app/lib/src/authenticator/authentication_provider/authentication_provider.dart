import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:mobile_app/src/authenticator/tokens_manager.dart';

typedef Token = String;
typedef AuthorizationCode = String;
typedef Scope = String;

/// Class that abstracts a common JWT auth providers interface.
abstract interface class AuthenticationProvider {

  /// Checks if the token did already expire.
  bool isTokenExpired(final Token token);

  /// Requests the access tokens (and co) after obtaining the authorization
  /// code.
  FutureOr<Tokens> requestAccessToken(final AuthorizationCode code);

  /// Refreshes an expired access token.
  FutureOr<Tokens> refreshToken(
    final Token refreshToken, {
    final List<Scope> scopes = const [],
  });

  FutureOr<AuthorizationCode?> showLogin(
    final BuildContext context, {
    final List<Scope> scopes = const [],
  });

  Tokens tokenFromJson(final Map<String, dynamic> json);

  Map<String, dynamic> decodeToken(final Token accessToken);
}
