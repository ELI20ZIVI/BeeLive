

final class HttpStatusException implements Exception {
  final Uri uri;

  final int statusCode;
  final String reasonPhrase;

  const HttpStatusException(this.uri, this.statusCode, this.reasonPhrase);

  @override
    String toString() {
      return "Unsuccessful request to $uri: $statusCode $reasonPhrase";
    }
}
