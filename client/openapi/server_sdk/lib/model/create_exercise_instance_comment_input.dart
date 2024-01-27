//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class CreateExerciseInstanceCommentInput {
  /// Returns a new [CreateExerciseInstanceCommentInput] instance.
  CreateExerciseInstanceCommentInput({
    required this.newComment,
  });

  String newComment;

  @override
  bool operator ==(Object other) => identical(this, other) || other is CreateExerciseInstanceCommentInput &&
    other.newComment == newComment;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (newComment.hashCode);

  @override
  String toString() => 'CreateExerciseInstanceCommentInput[newComment=$newComment]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'new_comment'] = this.newComment;
    return json;
  }

  /// Returns a new [CreateExerciseInstanceCommentInput] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static CreateExerciseInstanceCommentInput? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "CreateExerciseInstanceCommentInput[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "CreateExerciseInstanceCommentInput[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return CreateExerciseInstanceCommentInput(
        newComment: mapValueOfType<String>(json, r'new_comment')!,
      );
    }
    return null;
  }

  static List<CreateExerciseInstanceCommentInput> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <CreateExerciseInstanceCommentInput>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = CreateExerciseInstanceCommentInput.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, CreateExerciseInstanceCommentInput> mapFromJson(dynamic json) {
    final map = <String, CreateExerciseInstanceCommentInput>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = CreateExerciseInstanceCommentInput.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of CreateExerciseInstanceCommentInput-objects as value to a dart map
  static Map<String, List<CreateExerciseInstanceCommentInput>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<CreateExerciseInstanceCommentInput>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = CreateExerciseInstanceCommentInput.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'new_comment',
  };
}

