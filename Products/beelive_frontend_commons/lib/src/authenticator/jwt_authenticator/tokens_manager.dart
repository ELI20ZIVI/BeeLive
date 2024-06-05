import 'dart:async';
import 'dart:convert';

import 'package:flutter/foundation.dart';
import '../authentication_provider.dart';
import '../errors.dart';
import '../../storages/key_value_storage.dart';

/// Class that manages the OIDC tokens.
///
/// The tokens are stored, retrieved and refreshed automatically.
class TokensManager {
  /// The key in the Shared Preferences store for the tokens entry.
  ///
  /// The token is stored as a JSON.
  static const String _sharedPreferencesTokensKey = "authenticator.tokens";

  final AuthenticationProvider _provider;
  final KeyValueStorage prefs = KeyValueStorage();

  TokensManager(this._provider);

  /// Retrieve a previously saved token.
  ///
  /// `null` is returned in case no tokens have been stored.
  /// Errors are thrown in case of corruption.
  Future<Tokens?> get _load async {
    final rawTokens = prefs.get<String>(_sharedPreferencesTokensKey);

    debugPrint("shared preference token: $rawTokens");

    if (rawTokens == null) return null;

    try {
      return _provider.tokenFromJson(jsonDecode(rawTokens));
    } catch (_) {
      await _store(null);
      rethrow;
    }
  }

  FutureOr<bool> _store(final Tokens? tokens) async {
    final bool result;
    if (tokens == null) {
      result = await prefs.remove(_sharedPreferencesTokensKey);
    } else {
      result = await prefs.set(_sharedPreferencesTokensKey, jsonEncode(tokens));
    }
    return result;
  }

  /// Returns a valid token.\
  /// Returns null in case of no token.
  ///
  /// #### Exceptions
  /// Throws [TokenRefreshFailureException] in case of errors during token
  /// refreshing.
  Future<Tokens?> get tokens async {
    // Tries to retrieve it from shared preferences.
    var tokens = await _load;

    debugPrint("loaded token: $tokens");

    // Cannot do nothing without a previous token.
    if (tokens == null) return null;

    // Refreshes the token if expired
    tokens = await _refreshIfExpired(tokens);

    return tokens;
  }

  /// Requests and stores an access token obtained from the authorization [code].
  ///
  /// Returns the tokens.
  Future<Tokens> fromAuthorizationCode(final String code) async {
    final token = await _provider.requestAccessToken(code);

    assert(
      !_provider.isTokenExpired(token.accessToken),
      "The just generated access token is already expired.",
    );

    final result = await _store(token);

    assert(result, "Unable to store the token");

    return token;
  }

  /// Refreshes a token only if expired.
  ///
  /// #### Exceptions
  /// Throws [TokenRefreshFailureException] in case of errors during token
  /// refreshing.
  Future<Tokens?> _refreshIfExpired(final Tokens tokens) async {
    Tokens? result = tokens;
    if (_provider.isTokenExpired(tokens.accessToken)) {
      debugPrint("Expired token.");
      try {
        result = await _provider.refreshToken(
          tokens.refreshToken,
          scopes: tokens.scopes,
        );
      } catch (e) { // TODO: do no catch generic exception
        throw const TokenRefreshFailureException();
      }

      if (_provider.isTokenExpired(result.accessToken)) {
        throw const TokenRefreshFailureException();
      }

      final response = await _store(result);
      assert(response, "Could not store token after refresh");
    }
    return result;
  }
}

abstract base class Tokens {
  final Token accessToken;
  final Token idToken;
  final Token refreshToken;
  final String tokenType;
  final int expiresIn;
  final List<Scope> scopes;

  Tokens(
    this.accessToken,
    this.idToken,
    this.refreshToken,
    this.tokenType,
    this.expiresIn,
    this.scopes,
  );

  Map<String, dynamic> toJson();
}
