
import 'dart:async';

import 'package:flutter/widgets.dart';

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
  FutureOr<bool> authenticate(final BuildContext context);

  FutureOr<bool> authenticateIfAppropriate(final BuildContext context);

  /// The Authorization header payload.
  ///
  /// Returns null in absence of a token.
  Future<String?> get authorization;

}
