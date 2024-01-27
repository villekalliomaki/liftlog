//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class CreateSetInput {
  /// Returns a new [CreateSetInput] instance.
  CreateSetInput({
    required this.exerciseInstanceId,
  });

  String exerciseInstanceId;

  @override
  bool operator ==(Object other) => identical(this, other) || other is CreateSetInput &&
    other.exerciseInstanceId == exerciseInstanceId;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (exerciseInstanceId.hashCode);

  @override
  String toString() => 'CreateSetInput[exerciseInstanceId=$exerciseInstanceId]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'exercise_instance_id'] = this.exerciseInstanceId;
    return json;
  }

  /// Returns a new [CreateSetInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static CreateSetInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "CreateSetInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "CreateSetInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return CreateSetInput(
        exerciseInstanceId: mapValueOfType<String>(json, r'exercise_instance_id')!,
      );
    }
    return null;
  }

  static List<CreateSetInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <CreateSetInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = CreateSetInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, CreateSetInput> mapFromJson(dynamic json) {
    final map = <String, CreateSetInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = CreateSetInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of CreateSetInput-objects as value to a dart map
  static Map<String, List<CreateSetInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<CreateSetInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = CreateSetInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'exercise_instance_id',
  };
}

