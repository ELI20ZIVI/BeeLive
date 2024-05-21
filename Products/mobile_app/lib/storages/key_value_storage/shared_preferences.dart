import 'dart:async';
import 'dart:convert';

import 'package:mobile_app/storages/key_value_storage.dart';

import 'package:shared_preferences/shared_preferences.dart' as sp;

class SharedPreferences implements KeyValueStorage<String, Object> {
  final sp.SharedPreferences _underlying;

  const SharedPreferences(this._underlying);

  @override
  T? get<T>(String key) {
    return _underlying.get(key) as T?;
  }

  @override
  FutureOr<bool> set<T extends Object>(
    String key,
    T value, {
    bool jsonIfUnknown = false,
  }) {
    return switch (value) {
      String s => _underlying.setString(key, s),
      int i => _underlying.setInt(key, i),
      double d => _underlying.setDouble(key, d),
      bool b => _underlying.setBool(key, b),
      List<String> l => _underlying.setStringList(key, l),
      _ => jsonIfUnknown
          ? _underlying.setString(key, jsonEncode(value))
          : throw ArgumentError(
              "Cannot store a ${value.runtimeType} into SharedPreferences",
            ),
    };
  }

  @override
  FutureOr<bool> remove(final String key) {
    return _underlying.remove(key);
  }
}
