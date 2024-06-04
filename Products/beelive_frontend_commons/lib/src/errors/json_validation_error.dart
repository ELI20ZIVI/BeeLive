

final class JsonValidationError extends Error {
  final Uri uri;

  JsonValidationError(this.uri);
}