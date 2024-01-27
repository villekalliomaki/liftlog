//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class ExerciseInstance {
  /// Returns a new [ExerciseInstance] instance.
  ExerciseInstance({
    this.comments = const [],
    required this.created,
    required this.exerciseId,
    required this.id,
    required this.sessionId,
    this.sets = const [],
    required this.userId,
  });

  List<String> comments;

  DateTime created;

  String exerciseId;

  String id;

  String sessionId;

  List<ModelSet> sets;

  String userId;

  @override
  bool operator ==(Object other) => identical(this, other) || other is ExerciseInstance &&
    _deepEquality.equals(other.comments, comments) &&
    other.created == created &&
    other.exerciseId == exerciseId &&
    other.id == id &&
    other.sessionId == sessionId &&
    _deepEquality.equals(other.sets, sets) &&
    other.userId == userId;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (comments.hashCode) +
    (created.hashCode) +
    (exerciseId.hashCode) +
    (id.hashCode) +
    (sessionId.hashCode) +
    (sets.hashCode) +
    (userId.hashCode);

  @override
  String toString() => 'ExerciseInstance[comments=$comments, created=$created, exerciseId=$exerciseId, id=$id, sessionId=$sessionId, sets=$sets, userId=$userId]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'comments'] = this.comments;
      json[r'created'] = this.created.toUtc().toIso8601String();
      json[r'exercise_id'] = this.exerciseId;
      json[r'id'] = this.id;
      json[r'session_id'] = this.sessionId;
      json[r'sets'] = this.sets;
      json[r'user_id'] = this.userId;
    return json;
  }

  /// Returns a new [ExerciseInstance] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static ExerciseInstance? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "ExerciseInstance[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "ExerciseInstance[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return ExerciseInstance(
        comments: json[r'comments'] is Iterable
            ? (json[r'comments'] as Iterable).cast<String>().toList(growable: false)
            : const [],
        created: mapDateTime(json, r'created', r'')!,
        exerciseId: mapValueOfType<String>(json, r'exercise_id')!,
        id: mapValueOfType<String>(json, r'id')!,
        sessionId: mapValueOfType<String>(json, r'session_id')!,
        sets: ModelSet.listFromJson(json[r'sets']),
        userId: mapValueOfType<String>(json, r'user_id')!,
      );
    }
    return null;
  }

  static List<ExerciseInstance> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <ExerciseInstance>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = ExerciseInstance.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, ExerciseInstance> mapFromJson(dynamic json) {
    final map = <String, ExerciseInstance>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = ExerciseInstance.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of ExerciseInstance-objects as value to a dart map
  static Map<String, List<ExerciseInstance>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<ExerciseInstance>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = ExerciseInstance.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'comments',
    'created',
    'exercise_id',
    'id',
    'session_id',
    'sets',
    'user_id',
  };
}

