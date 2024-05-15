// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'casdoor_token.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CasdoorTokens _$CasdoorTokensFromJson(Map<String, dynamic> json) =>
    CasdoorTokens(
      json['access_token'] as String,
      json['id_token'] as String,
      json['refresh_token'] as String,
      json['token_type'] as String,
      (json['expires_in'] as num).toInt(),
      (json['scopes'] as List<dynamic>).map((e) => e as String).toList(),
    );

Map<String, dynamic> _$CasdoorTokensToJson(CasdoorTokens instance) =>
    <String, dynamic>{
      'access_token': instance.accessToken,
      'id_token': instance.idToken,
      'refresh_token': instance.refreshToken,
      'token_type': instance.tokenType,
      'expires_in': instance.expiresIn,
      'scopes': instance.scopes,
    };
