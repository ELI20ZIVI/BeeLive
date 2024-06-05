
import 'dart:async';

import 'package:flutter/widgets.dart';

export 'authenticator/jwt_authenticator.dart';
export 'authenticator/errors.dart';

/// Authenticator class to perform authentication of the user.
///
/// This is a class delegate to abstract the system APIs.
abstract interface class Authenticator {

  static late final Authenticator _implementation;

  /// Overrides the default client for this session.
  static void override(Authenticator implementation) {
    _implementation = implementation;
  }

  /// Creates the default client.
  factory Authenticator() {
    return _implementation;
  }

  /// Asks the user to sign in (or sign on).
  ///
  /// Returns if the operation has been successful.
  Future<bool> authenticate(final BuildContext context);

  Future<bool> authenticateIfAppropriate(final BuildContext context);

  /// The Authorization header payload.
  ///
  /// Returns null in absence of a token.
  ///
  /// #### Exceptions
  /// Throws [TokenRefreshFailureException] in case of errors during token
  /// refreshing.\
  /// Throws [AuthenticationNotAskedException].
  Future<String?> authorization();

  /// Invalidates the token.
  ///
  /// This method should be called in case of 401 and 403 errors
  /// in order to force re-authentication of the user.
  Future<void> invalidateToken();

}
