

final class HttpStatusException implements Exception {
  final Uri uri;

  final int statusCode;
  final String reasonPhrase;

  const HttpStatusException(this.uri, this.statusCode, this.reasonPhrase);
}
