import 'dart:async';

import 'package:casdoor_flutter_sdk/casdoor_flutter_sdk.dart';
import 'package:flutter/widgets.dart';
import 'package:mobile_app/src/authenticator/authentication_provider/authentication_provider.dart';
import 'package:mobile_app/src/storage/key_value_storage.dart';
import 'package:synchronized/extension.dart';

/// Class that manages the authentication phase.
class AuthenticationManager {
  /// The key in the Shared Preferences store for the tokens entry.
  ///
  /// The token is stored as a JSON.
  static const String _sharedPreferencesAuthenticatedKey =
      "authenticator.didAuthenticate";

  /// The OAuth scopes.
  final List<Scope> _scopes;

  final AuthenticationProvider _provider;
  final KeyValueStorage prefs = KeyValueStorage();

  Completer<String?> _codeCompleter = Completer();

  AuthenticationManager(
    this._provider, [
    this._scopes = const ["openid"],
  ]);

  bool wasAuthenticationRefused() {
    final didAuthenticate = prefs.get<bool>(_sharedPreferencesAuthenticatedKey);

    debugPrint("didAuthenticate: $didAuthenticate");

    return didAuthenticate == false;
  }

  bool shouldAuthenticate() {
    final didAuthenticate = prefs.get<bool>(_sharedPreferencesAuthenticatedKey);

    return didAuthenticate == null;
  }

  Future<String?> _authenticate(final BuildContext context) async {
    debugPrint("authenticating");
    final code = await _provider.showLogin(context, scopes: _scopes);
    debugPrint("authenticated");

    await prefs.set(_sharedPreferencesAuthenticatedKey, code != null);

    return code;
  }

  /// Asks the user for authentication.
  ///
  /// Note: this method is synchronized to avoid concurrent authentication.
  Future<String?> authenticate(final BuildContext context) {
    debugPrint("Waiting for authentication lock");
    return synchronized(() async {
      debugPrint("Obtained authentication lock");
      if (_codeCompleter.isCompleted) {
        _codeCompleter = Completer();
      }

      final String? result;
      try {
        result = await _authenticate(context);
      } catch (e) {
        debugPrint("Error during authentication: $e");
        _codeCompleter.completeError(e);
        rethrow;
      }
      _codeCompleter.complete(result);
      return result;
    });
  }

  Future<String?> authenticateIfAppropriate(
    final BuildContext context, [
    bool hasValidToken = true,
  ]) async {
    final isAppropriate =
        shouldAuthenticate() || (!hasValidToken && !wasAuthenticationRefused());

    debugPrint("Appropriate authentication: $isAppropriate");

    if (!isAppropriate) {
      if (!_codeCompleter.isCompleted) {
        _codeCompleter.complete(null);
      }
      return null;
    }

    if (context.mounted) {
      return authenticate(context);
    } else {
      debugPrint("Unmounted context");
      return null;
    }
  }

  Future<String?> get code async {
    if (wasAuthenticationRefused()) {
      return null;
    }
    return await _codeCompleter.future;
  }
}
