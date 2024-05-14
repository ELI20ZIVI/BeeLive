
import 'dart:async';

import 'package:casdoor_flutter_sdk/casdoor_flutter_sdk.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:synchronized/extension.dart';

/// Class that manages the authentication phase.
class AuthenticationManager {
  /// The key in the Shared Preferences store for the tokens entry.
  ///
  /// The token is stored as a JSON.
  static const String _sharedPreferencesAuthenticatedKey =
      "authenticator.didAuthenticate";

  /// The OAuth scopes.
  static const String _scope = "openid profile";

  final Casdoor _casdoor;

  Completer<String?> _codeCompleter = Completer();

  AuthenticationManager(this._casdoor);

  Future<bool> wasAuthenticationRefused() async {
    final prefs = await SharedPreferences.getInstance();
    final didAuthenticate = prefs.getBool(_sharedPreferencesAuthenticatedKey);

    debugPrint("didAuthenticate: $didAuthenticate");

    return didAuthenticate == false;
  }

  Future<bool> shouldAuthenticate() async {
    final prefs = await SharedPreferences.getInstance();
    final didAuthenticate = prefs.getBool(_sharedPreferencesAuthenticatedKey);

    return didAuthenticate == null;
  }

  Future<String?> _authenticate(final BuildContext context) async {
    final callback = await _casdoor.showFullscreen(context, scope: _scope);

    final prefs = await SharedPreferences.getInstance();

    String? result;
    try {
      final String code = Uri
          .parse(callback)
          .queryParameters['code'] as String;

      result = code;
    } on CasdoorAuthCancelledException catch (_) {
      result = null;
    }

    await prefs.setBool(_sharedPreferencesAuthenticatedKey, result != null);

    return result;
  }

  /// Asks the user for authentication.
  ///
  /// Note: this method is synchronized to avoid concurrent authentication.
  Future<String?> authenticate(final BuildContext context) {
    return synchronized(() async {
      if (_codeCompleter.isCompleted) {
        _codeCompleter = Completer();
      }

      final String? result;
      try {
        result = await _authenticate(context);
      } catch (e) {
        _codeCompleter.completeError(e);
        rethrow;
      }
      _codeCompleter.complete(result);
      return result;
    });
  }

  Future<String?> authenticateIfAppropriate(final BuildContext context) async {
    if (!await shouldAuthenticate()) {
      if (!_codeCompleter.isCompleted) {
        _codeCompleter.complete(null);
      }
      return null;
    }

    if (context.mounted) {
      return authenticate(context);
    } else {
      return null;
    }
  }

  Future<String?> get code async {
    if (await wasAuthenticationRefused()) {
      return null;
    }
    return await _codeCompleter.future;
  }

}