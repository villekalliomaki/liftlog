//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class EditSetInput {
  /// Returns a new [EditSetInput] instance.
  EditSetInput({
    this.completed,
    this.reps,
    this.weight,
  });

  bool? completed;

  int? reps;

  double? weight;

  @override
  bool operator ==(Object other) => identical(this, other) || other is EditSetInput &&
    other.completed == completed &&
    other.reps == reps &&
    other.weight == weight;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (completed == null ? 0 : completed!.hashCode) +
    (reps == null ? 0 : reps!.hashCode) +
    (weight == null ? 0 : weight!.hashCode);

  @override
  String toString() => 'EditSetInput[completed=$completed, reps=$reps, weight=$weight]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    if (this.completed != null) {
      json[r'completed'] = this.completed;
    } else {
      json[r'completed'] = null;
    }
    if (this.reps != null) {
      json[r'reps'] = this.reps;
    } else {
      json[r'reps'] = null;
    }
    if (this.weight != null) {
      json[r'weight'] = this.weight;
    } else {
      json[r'weight'] = null;
    }
    return json;
  }

  /// Returns a new [EditSetInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static EditSetInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "EditSetInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "EditSetInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return EditSetInput(
        completed: mapValueOfType<bool>(json, r'completed'),
        reps: mapValueOfType<int>(json, r'reps'),
        weight: mapValueOfType<double>(json, r'weight'),
      );
    }
    return null;
  }

  static List<EditSetInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <EditSetInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = EditSetInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, EditSetInput> mapFromJson(dynamic json) {
    final map = <String, EditSetInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = EditSetInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of EditSetInput-objects as value to a dart map
  static Map<String, List<EditSetInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<EditSetInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = EditSetInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
  };
}

