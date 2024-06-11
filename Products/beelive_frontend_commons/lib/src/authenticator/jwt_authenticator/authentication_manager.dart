import 'dart:async';

import 'package:flutter/widgets.dart';
import '../authentication_provider.dart';
import '../errors.dart';
import '../../storages/key_value_storage.dart';
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

  String? _code;

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

  /// Asks the user for authentication storing the expressed preference
  /// into [prefs].
  Future<String?> _authenticate(final BuildContext context) async {
    debugPrint("authenticating");
    final code = await _provider.showLogin(context, scopes: _scopes);
    debugPrint("authenticated");

    await prefs.set(_sharedPreferencesAuthenticatedKey, code != null);

    return code;
  }

  /// Asks the user for authentication.
  /// Updates the [code].
  ///
  /// Note: this method is synchronized to avoid concurrent authentication.
  Future<String?> authenticate(final BuildContext context) {
    debugPrint("Waiting for authentication lock");
    return synchronized(() async {
      debugPrint("Obtained authentication lock");

      final String? result;
      try {
        result = await _authenticate(context);
        debugPrint("Authorization code: $result");
      } catch (e) {
        debugPrint("Error during authentication: $e");
        rethrow;
      }
      _code = result;
      debugPrint("Authorization code: $result");
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
      return _code = null;
    }

    return authenticate(context);
  }

  /// #### Exceptions
  /// Throws [AuthenticationNotAskedException].
  String? code() {
    if (wasAuthenticationRefused()) {
      return null;
    }
    if (_code != null) {
      return _code;
    } else {
      throw AuthenticationNotAskedException();
    }
  }


  Future<void> invalidate() async {
    _code = null;
  }
}
