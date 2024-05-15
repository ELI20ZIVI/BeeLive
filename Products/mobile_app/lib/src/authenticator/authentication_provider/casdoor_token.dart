import 'package:json_annotation/json_annotation.dart';
import 'package:mobile_app/src/authenticator/tokens_manager.dart';

part 'casdoor_token.g.dart';

@JsonSerializable(fieldRename: FieldRename.snake)
final class CasdoorTokens extends Tokens {

  CasdoorTokens(
    super.accessToken,
    super.idToken,
    super.refreshToken,
    super.tokenType,
    super.expiresIn,
    super.scopes,
  );

  factory CasdoorTokens.fromJson(final Map<String, dynamic> json) {
    return _$CasdoorTokensFromJson(json);
  }

  @override
  Map<String, dynamic> toJson() => _$CasdoorTokensToJson(this);
}
