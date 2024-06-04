

final class HttpNoSuchHostError extends Error {
  final Uri uri;

  HttpNoSuchHostError(this.uri);
}