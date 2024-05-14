import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:casdoor_flutter_sdk/casdoor_flutter_sdk.dart';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:shared_preferences/shared_preferences.dart';

part 'tokens_manager.g.dart';

/// Class that manages the OIDC tokens.
///
/// The tokens are stored, retrieved and refreshed automatically.
class TokensManager {
  /// The key in the Shared Preferences store for the tokens entry.
  ///
  /// The token is stored as a JSON.
  static const String _sharedPreferencesTokensKey = "authenticator.tokens";

  final Casdoor _casdoor;

  const TokensManager(this._casdoor);

  /// Retrieve a previously saved token.
  ///
  /// `null` is returned in case no tokens have been stored.
  /// Errors are thrown in case of corruption.
  Future<Tokens?> get _load async {
    final prefs = await SharedPreferences.getInstance();
    final rawTokens = prefs.getString(_sharedPreferencesTokensKey);

    if (rawTokens == null) return null;

    try {
      return Tokens.fromJson(jsonDecode(rawTokens));
    } catch (_) {
      await prefs.remove(_sharedPreferencesTokensKey);
      rethrow;
    }
  }

  Future<bool> _store(final Tokens tokens) async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.setString(_sharedPreferencesTokensKey, jsonEncode(tokens));
  }

  /// Returns a valid token.\
  /// Returns null in case of no token.
  Future<Tokens?> get tokens async {
    // Tries to retrieve it from shared preferences.
    var tokens = await _load;

    // Cannot do nothing without a previous token.
    if (tokens == null) return null;

    // Refreshes the token if expired
    tokens = await _refreshIfExpired(tokens);

    // NOTE: data races could actually trigger this assertion.
    assert(
      _casdoor.isTokenExpired(tokens.accessToken),
      "The just generated access token is already expired",
    );

    return tokens;
  }

  /// Requests and stores an access token obtained from the authorization [code].
  ///
  /// Returns the tokens.
  Future<Tokens> fromAuthorizationCode(final String code) async {
    final response = await _casdoor.requestOauthAccessToken(code);
    debugPrint(response.body);
    var tokens = _fromResponse(response);

    assert(
      _casdoor.isTokenExpired(tokens.accessToken),
      "The just generated access token is already expired",
    );

    final result = await _store(tokens);

    assert(result, "Unable to store the token");

    return tokens;
  }

  Future<Tokens> _refreshIfExpired(Tokens tokens) async {
    if (_casdoor.isTokenExpired(tokens.accessToken)) {
      final response = await _casdoor.refreshToken(
        tokens.refreshToken,
        // TODO: Why is it required? Should be secret.
        "8cd8dde871a54de9f5846b1b061e1040c160833f",
        scope: tokens.scope,
      );

      tokens = _fromResponse(response);

      _store(tokens);
    }
    return tokens;
  }

  Tokens _fromResponse(final Response response) {
    if (response.statusCode != HttpStatus.ok) {
      throw HttpException(response.reasonPhrase ?? "Unknown error");
    }

    final json = jsonDecode(response.body);
    return Tokens.fromJson(json);
  }
}

typedef Token = String;

@JsonSerializable(fieldRename: FieldRename.snake)
class Tokens {
  final Token accessToken;
  final Token idToken;
  final Token refreshToken;
  final String tokenType;
  final int expiresIn;
  final String scope;

  Tokens(
    this.accessToken,
    this.idToken,
    this.refreshToken,
    this.tokenType,
    this.expiresIn,
    this.scope,
  );

  factory Tokens.fromJson(final Map<String, dynamic> json) =>
      _$TokensFromJson(json);

  Map<String, dynamic> toJson() => _$TokensToJson(this);
}
