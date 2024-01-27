//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class CreateExerciseInstanceInput {
  /// Returns a new [CreateExerciseInstanceInput] instance.
  CreateExerciseInstanceInput({
    required this.exerciseId,
    required this.sessionId,
  });

  String exerciseId;

  String sessionId;

  @override
  bool operator ==(Object other) => identical(this, other) || other is CreateExerciseInstanceInput &&
    other.exerciseId == exerciseId &&
    other.sessionId == sessionId;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (exerciseId.hashCode) +
    (sessionId.hashCode);

  @override
  String toString() => 'CreateExerciseInstanceInput[exerciseId=$exerciseId, sessionId=$sessionId]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'exercise_id'] = this.exerciseId;
      json[r'session_id'] = this.sessionId;
    return json;
  }

  /// Returns a new [CreateExerciseInstanceInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static CreateExerciseInstanceInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "CreateExerciseInstanceInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "CreateExerciseInstanceInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return CreateExerciseInstanceInput(
        exerciseId: mapValueOfType<String>(json, r'exercise_id')!,
        sessionId: mapValueOfType<String>(json, r'session_id')!,
      );
    }
    return null;
  }

  static List<CreateExerciseInstanceInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <CreateExerciseInstanceInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = CreateExerciseInstanceInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, CreateExerciseInstanceInput> mapFromJson(dynamic json) {
    final map = <String, CreateExerciseInstanceInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = CreateExerciseInstanceInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of CreateExerciseInstanceInput-objects as value to a dart map
  static Map<String, List<CreateExerciseInstanceInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<CreateExerciseInstanceInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = CreateExerciseInstanceInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'exercise_id',
    'session_id',
  };
}

