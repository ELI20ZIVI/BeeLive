import 'dart:async';

export 'key_value_storage/shared_preferences.dart';

/// A key-value based storage.
///
/// [K] is the type of the keys.\
/// [V] is a supertype of the values.
abstract interface class KeyValueStorage<K, V> {
  static late final KeyValueStorage _implementation;

  /// Overrides the default storage for this session.
  static void override(final KeyValueStorage implementation) {
    _implementation = implementation;
  }

  /// Access the default storage.
  factory KeyValueStorage() {
    return _implementation as KeyValueStorage<K, V>;
  }

  /// Gets the value corresponding to [key] assuming it is of type [T].
  T? get<T>(final K key);

  FutureOr<bool> set<T extends V>(
    final K key,
    T value, {
    bool jsonIfUnknown = false,
  });

  FutureOr<bool> remove(final K key);
}
