import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:mobile_app/authenticator/jwt_authenticator/tokens_manager.dart';
import 'package:result_dart/result_dart.dart';

export 'authentication_provider/casdoor_authentication_provider.dart';

typedef Token = String;
typedef AuthorizationCode = String;
typedef Scope = String;

/// Class that abstracts a common JWT auth providers interface.
abstract interface class AuthenticationProvider {

  /// Checks if the token did already expire.
  bool isTokenExpired(final Token token);

  /// Requests the access tokens after obtaining the authorization code.
  ///
  /// Throws [HttpStatusException] if the [response]
  /// doesn't have a [HttpStatus.ok] status.
  /// Throws [JsonValidationError] if the response doesn't have a valid body.
  Future<Tokens> requestAccessToken(final AuthorizationCode code);

  /// Refreshes an expired access token.
  ///
  /// Throws [HttpStatusException] if the [response]
  /// doesn't have a [HttpStatus.ok] status.\
  /// Throws [JsonValidationError] if the response doesn't have a valid body.
  Future<Tokens> refreshToken(
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
