//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class ModelSet {
  /// Returns a new [ModelSet] instance.
  ModelSet({
    required this.completed,
    required this.created,
    required this.exerciseInstanceId,
    required this.id,
    this.reps,
    required this.userId,
    this.weight,
  });

  bool completed;

  DateTime created;

  String exerciseInstanceId;

  String id;

  int? reps;

  String userId;

  double? weight;

  @override
  bool operator ==(Object other) => identical(this, other) || other is ModelSet &&
    other.completed == completed &&
    other.created == created &&
    other.exerciseInstanceId == exerciseInstanceId &&
    other.id == id &&
    other.reps == reps &&
    other.userId == userId &&
    other.weight == weight;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (completed.hashCode) +
    (created.hashCode) +
    (exerciseInstanceId.hashCode) +
    (id.hashCode) +
    (reps == null ? 0 : reps!.hashCode) +
    (userId.hashCode) +
    (weight == null ? 0 : weight!.hashCode);

  @override
  String toString() => 'ModelSet[completed=$completed, created=$created, exerciseInstanceId=$exerciseInstanceId, id=$id, reps=$reps, userId=$userId, weight=$weight]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'completed'] = this.completed;
      json[r'created'] = this.created.toUtc().toIso8601String();
      json[r'exercise_instance_id'] = this.exerciseInstanceId;
      json[r'id'] = this.id;
    if (this.reps != null) {
      json[r'reps'] = this.reps;
    } else {
      json[r'reps'] = null;
    }
      json[r'user_id'] = this.userId;
    if (this.weight != null) {
      json[r'weight'] = this.weight;
    } else {
      json[r'weight'] = null;
    }
    return json;
  }

  /// Returns a new [ModelSet] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static ModelSet? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "ModelSet[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "ModelSet[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return ModelSet(
        completed: mapValueOfType<bool>(json, r'completed')!,
        created: mapDateTime(json, r'created', r'')!,
        exerciseInstanceId: mapValueOfType<String>(json, r'exercise_instance_id')!,
        id: mapValueOfType<String>(json, r'id')!,
        reps: mapValueOfType<int>(json, r'reps'),
        userId: mapValueOfType<String>(json, r'user_id')!,
        weight: mapValueOfType<double>(json, r'weight'),
      );
    }
    return null;
  }

  static List<ModelSet> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <ModelSet>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = ModelSet.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, ModelSet> mapFromJson(dynamic json) {
    final map = <String, ModelSet>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = ModelSet.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of ModelSet-objects as value to a dart map
  static Map<String, List<ModelSet>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<ModelSet>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = ModelSet.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'completed',
    'created',
    'exercise_instance_id',
    'id',
    'user_id',
  };
}

