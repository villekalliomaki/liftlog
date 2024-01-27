//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class RouteSuccessExercise {
  /// Returns a new [RouteSuccessExercise] instance.
  RouteSuccessExercise({
    required this.data,
    required this.msg,
  });

  Exercise data;

  String msg;

  @override
  bool operator ==(Object other) => identical(this, other) || other is RouteSuccessExercise &&
    other.data == data &&
    other.msg == msg;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (data.hashCode) +
    (msg.hashCode);

  @override
  String toString() => 'RouteSuccessExercise[data=$data, msg=$msg]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'data'] = this.data;
      json[r'msg'] = this.msg;
    return json;
  }

  /// Returns a new [RouteSuccessExercise] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static RouteSuccessExercise? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "RouteSuccessExercise[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "RouteSuccessExercise[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return RouteSuccessExercise(
        data: Exercise.fromJson(json[r'data'])!,
        msg: mapValueOfType<String>(json, r'msg')!,
      );
    }
    return null;
  }

  static List<RouteSuccessExercise> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <RouteSuccessExercise>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = RouteSuccessExercise.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, RouteSuccessExercise> mapFromJson(dynamic json) {
    final map = <String, RouteSuccessExercise>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = RouteSuccessExercise.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of RouteSuccessExercise-objects as value to a dart map
  static Map<String, List<RouteSuccessExercise>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<RouteSuccessExercise>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = RouteSuccessExercise.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'data',
    'msg',
  };
}

