//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class Exercise {
  /// Returns a new [Exercise] instance.
  Exercise({
    this.description,
    required this.favourite,
    required this.id,
    required this.kind,
    required this.name,
    this.notes,
    required this.userId,
  });

  String? description;

  bool favourite;

  String id;

  ExerciseKind kind;

  String name;

  String? notes;

  String userId;

  @override
  bool operator ==(Object other) => identical(this, other) || other is Exercise &&
    other.description == description &&
    other.favourite == favourite &&
    other.id == id &&
    other.kind == kind &&
    other.name == name &&
    other.notes == notes &&
    other.userId == userId;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (description == null ? 0 : description!.hashCode) +
    (favourite.hashCode) +
    (id.hashCode) +
    (kind.hashCode) +
    (name.hashCode) +
    (notes == null ? 0 : notes!.hashCode) +
    (userId.hashCode);

  @override
  String toString() => 'Exercise[description=$description, favourite=$favourite, id=$id, kind=$kind, name=$name, notes=$notes, userId=$userId]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    if (this.description != null) {
      json[r'description'] = this.description;
    } else {
      json[r'description'] = null;
    }
      json[r'favourite'] = this.favourite;
      json[r'id'] = this.id;
      json[r'kind'] = this.kind;
      json[r'name'] = this.name;
    if (this.notes != null) {
      json[r'notes'] = this.notes;
    } else {
      json[r'notes'] = null;
    }
      json[r'user_id'] = this.userId;
    return json;
  }

  /// Returns a new [Exercise] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static Exercise? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "Exercise[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "Exercise[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return Exercise(
        description: mapValueOfType<String>(json, r'description'),
        favourite: mapValueOfType<bool>(json, r'favourite')!,
        id: mapValueOfType<String>(json, r'id')!,
        kind: ExerciseKind.fromJson(json[r'kind'])!,
        name: mapValueOfType<String>(json, r'name')!,
        notes: mapValueOfType<String>(json, r'notes'),
        userId: mapValueOfType<String>(json, r'user_id')!,
      );
    }
    return null;
  }

  static List<Exercise> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <Exercise>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = Exercise.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, Exercise> mapFromJson(dynamic json) {
    final map = <String, Exercise>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = Exercise.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of Exercise-objects as value to a dart map
  static Map<String, List<Exercise>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<Exercise>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = Exercise.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'favourite',
    'id',
    'kind',
    'name',
    'user_id',
  };
}

