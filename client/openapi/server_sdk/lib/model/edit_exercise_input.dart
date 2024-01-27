//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class EditExerciseInput {
  /// Returns a new [EditExerciseInput] instance.
  EditExerciseInput({
    this.description,
    this.favourite,
    this.kind,
    this.name,
    this.notes,
  });

  String? description;

  bool? favourite;

  ExerciseKind? kind;

  String? name;

  String? notes;

  @override
  bool operator ==(Object other) => identical(this, other) || other is EditExerciseInput &&
    other.description == description &&
    other.favourite == favourite &&
    other.kind == kind &&
    other.name == name &&
    other.notes == notes;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (description == null ? 0 : description!.hashCode) +
    (favourite == null ? 0 : favourite!.hashCode) +
    (kind == null ? 0 : kind!.hashCode) +
    (name == null ? 0 : name!.hashCode) +
    (notes == null ? 0 : notes!.hashCode);

  @override
  String toString() => 'EditExerciseInput[description=$description, favourite=$favourite, kind=$kind, name=$name, notes=$notes]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    if (this.description != null) {
      json[r'description'] = this.description;
    } else {
      json[r'description'] = null;
    }
    if (this.favourite != null) {
      json[r'favourite'] = this.favourite;
    } else {
      json[r'favourite'] = null;
    }
    if (this.kind != null) {
      json[r'kind'] = this.kind;
    } else {
      json[r'kind'] = null;
    }
    if (this.name != null) {
      json[r'name'] = this.name;
    } else {
      json[r'name'] = null;
    }
    if (this.notes != null) {
      json[r'notes'] = this.notes;
    } else {
      json[r'notes'] = null;
    }
    return json;
  }

  /// Returns a new [EditExerciseInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static EditExerciseInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "EditExerciseInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "EditExerciseInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return EditExerciseInput(
        description: mapValueOfType<String>(json, r'description'),
        favourite: mapValueOfType<bool>(json, r'favourite'),
        kind: ExerciseKind.fromJson(json[r'kind']),
        name: mapValueOfType<String>(json, r'name'),
        notes: mapValueOfType<String>(json, r'notes'),
      );
    }
    return null;
  }

  static List<EditExerciseInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <EditExerciseInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = EditExerciseInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, EditExerciseInput> mapFromJson(dynamic json) {
    final map = <String, EditExerciseInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = EditExerciseInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of EditExerciseInput-objects as value to a dart map
  static Map<String, List<EditExerciseInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<EditExerciseInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = EditExerciseInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
  };
}

