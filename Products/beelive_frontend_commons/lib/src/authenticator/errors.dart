
// TODO: more granularity:
//  - failure due to refresh token expiration.
//  - failure due to connection errors.
//  - failure due to provider configuration changes.
//  - ...
/// Exception thrown during token refreshing if is not possible to
/// perform the action.
final class TokenRefreshFailureException implements Exception {
  const TokenRefreshFailureException();
}

/// Exception thrown if the user was never asked to authenticate.
final class AuthenticationNotAskedException implements Exception {
  AuthenticationNotAskedException();
}