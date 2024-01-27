//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class Session {
  /// Returns a new [Session] instance.
  Session({
    this.description,
    this.exerciseInstances = const [],
    this.finished,
    required this.id,
    required this.name,
    required this.started,
    required this.userId,
  });

  String? description;

  List<ExerciseInstance> exerciseInstances;

  DateTime? finished;

  String id;

  String name;

  DateTime started;

  String userId;

  @override
  bool operator ==(Object other) => identical(this, other) || other is Session &&
    other.description == description &&
    _deepEquality.equals(other.exerciseInstances, exerciseInstances) &&
    other.finished == finished &&
    other.id == id &&
    other.name == name &&
    other.started == started &&
    other.userId == userId;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (description == null ? 0 : description!.hashCode) +
    (exerciseInstances.hashCode) +
    (finished == null ? 0 : finished!.hashCode) +
    (id.hashCode) +
    (name.hashCode) +
    (started.hashCode) +
    (userId.hashCode);

  @override
  String toString() => 'Session[description=$description, exerciseInstances=$exerciseInstances, finished=$finished, id=$id, name=$name, started=$started, userId=$userId]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    if (this.description != null) {
      json[r'description'] = this.description;
    } else {
      json[r'description'] = null;
    }
      json[r'exercise_instances'] = this.exerciseInstances;
    if (this.finished != null) {
      json[r'finished'] = this.finished!.toUtc().toIso8601String();
    } else {
      json[r'finished'] = null;
    }
      json[r'id'] = this.id;
      json[r'name'] = this.name;
      json[r'started'] = this.started.toUtc().toIso8601String();
      json[r'user_id'] = this.userId;
    return json;
  }

  /// Returns a new [Session] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static Session? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "Session[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "Session[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return Session(
        description: mapValueOfType<String>(json, r'description'),
        exerciseInstances: ExerciseInstance.listFromJson(json[r'exercise_instances']),
        finished: mapDateTime(json, r'finished', r''),
        id: mapValueOfType<String>(json, r'id')!,
        name: mapValueOfType<String>(json, r'name')!,
        started: mapDateTime(json, r'started', r'')!,
        userId: mapValueOfType<String>(json, r'user_id')!,
      );
    }
    return null;
  }

  static List<Session> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <Session>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = Session.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, Session> mapFromJson(dynamic json) {
    final map = <String, Session>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = Session.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of Session-objects as value to a dart map
  static Map<String, List<Session>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<Session>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = Session.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'exercise_instances',
    'id',
    'name',
    'started',
    'user_id',
  };
}

