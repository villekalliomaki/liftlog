//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class CreateExerciseInput {
  /// Returns a new [CreateExerciseInput] instance.
  CreateExerciseInput({
    this.description,
    this.favourite,
    required this.kind,
    required this.name,
    this.notes,
  });

  String? description;

  ///
  /// Please note: This property should have been non-nullable! Since the specification file
  /// does not include a default value (using the "default:" property), however, the generated
  /// source code must fall back to having a nullable type.
  /// Consider adding a "default:" property in the specification file to hide this note.
  ///
  bool? favourite;

  ExerciseKind kind;

  String name;

  String? notes;

  @override
  bool operator ==(Object other) => identical(this, other) || other is CreateExerciseInput &&
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
    (kind.hashCode) +
    (name.hashCode) +
    (notes == null ? 0 : notes!.hashCode);

  @override
  String toString() => 'CreateExerciseInput[description=$description, favourite=$favourite, kind=$kind, name=$name, notes=$notes]';

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
      json[r'kind'] = this.kind;
      json[r'name'] = this.name;
    if (this.notes != null) {
      json[r'notes'] = this.notes;
    } else {
      json[r'notes'] = null;
    }
    return json;
  }

  /// Returns a new [CreateExerciseInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static CreateExerciseInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "CreateExerciseInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "CreateExerciseInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return CreateExerciseInput(
        description: mapValueOfType<String>(json, r'description'),
        favourite: mapValueOfType<bool>(json, r'favourite'),
        kind: ExerciseKind.fromJson(json[r'kind'])!,
        name: mapValueOfType<String>(json, r'name')!,
        notes: mapValueOfType<String>(json, r'notes'),
      );
    }
    return null;
  }

  static List<CreateExerciseInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <CreateExerciseInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = CreateExerciseInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, CreateExerciseInput> mapFromJson(dynamic json) {
    final map = <String, CreateExerciseInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = CreateExerciseInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of CreateExerciseInput-objects as value to a dart map
  static Map<String, List<CreateExerciseInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<CreateExerciseInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = CreateExerciseInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'kind',
    'name',
  };
}

