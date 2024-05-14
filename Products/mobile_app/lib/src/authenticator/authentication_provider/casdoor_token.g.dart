// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'casdoor_token.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CasdoorTokens _$CasdoorTokensFromJson(Map<String, dynamic> json) =>
    CasdoorTokens(
      json['accessToken'] as String,
      json['idToken'] as String,
      json['refreshToken'] as String,
      json['tokenType'] as String,
      (json['expiresIn'] as num).toInt(),
      (json['scopes'] as List<dynamic>).map((e) => e as String).toList(),
    );

Map<String, dynamic> _$CasdoorTokensToJson(CasdoorTokens instance) =>
    <String, dynamic>{
      'accessToken': instance.accessToken,
      'idToken': instance.idToken,
      'refreshToken': instance.refreshToken,
      'tokenType': instance.tokenType,
      'expiresIn': instance.expiresIn,
      'scopes': instance.scopes,
    };
