

final class HttpStatusException implements Exception {
  final Uri uri;

  final int statusCode;
  final String reasonPhrase;

  HttpStatusException(this.uri, this.statusCode, this.reasonPhrase);
}