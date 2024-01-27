//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class SingleRouteError {
  /// Returns a new [SingleRouteError] instance.
  SingleRouteError({
    this.field,
    required this.msg,
  });

  String? field;

  String msg;

  @override
  bool operator ==(Object other) => identical(this, other) || other is SingleRouteError &&
    other.field == field &&
    other.msg == msg;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (field == null ? 0 : field!.hashCode) +
    (msg.hashCode);

  @override
  String toString() => 'SingleRouteError[field=$field, msg=$msg]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    if (this.field != null) {
      json[r'field'] = this.field;
    } else {
      json[r'field'] = null;
    }
      json[r'msg'] = this.msg;
    return json;
  }

  /// Returns a new [SingleRouteError] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static SingleRouteError? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "SingleRouteError[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "SingleRouteError[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return SingleRouteError(
        field: mapValueOfType<String>(json, r'field'),
        msg: mapValueOfType<String>(json, r'msg')!,
      );
    }
    return null;
  }

  static List<SingleRouteError> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <SingleRouteError>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = SingleRouteError.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, SingleRouteError> mapFromJson(dynamic json) {
    final map = <String, SingleRouteError>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = SingleRouteError.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of SingleRouteError-objects as value to a dart map
  static Map<String, List<SingleRouteError>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<SingleRouteError>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = SingleRouteError.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'msg',
  };
}

