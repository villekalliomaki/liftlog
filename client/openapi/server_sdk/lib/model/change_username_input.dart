//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class ChangeUsernameInput {
  /// Returns a new [ChangeUsernameInput] instance.
  ChangeUsernameInput({
    required this.newUsername,
  });

  String newUsername;

  @override
  bool operator ==(Object other) => identical(this, other) || other is ChangeUsernameInput &&
    other.newUsername == newUsername;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (newUsername.hashCode);

  @override
  String toString() => 'ChangeUsernameInput[newUsername=$newUsername]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'new_username'] = this.newUsername;
    return json;
  }

  /// Returns a new [ChangeUsernameInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static ChangeUsernameInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "ChangeUsernameInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "ChangeUsernameInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return ChangeUsernameInput(
        newUsername: mapValueOfType<String>(json, r'new_username')!,
      );
    }
    return null;
  }

  static List<ChangeUsernameInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <ChangeUsernameInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = ChangeUsernameInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, ChangeUsernameInput> mapFromJson(dynamic json) {
    final map = <String, ChangeUsernameInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = ChangeUsernameInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of ChangeUsernameInput-objects as value to a dart map
  static Map<String, List<ChangeUsernameInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<ChangeUsernameInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = ChangeUsernameInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'new_username',
  };
}

