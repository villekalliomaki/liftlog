//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;


class ExerciseKind {
  /// Instantiate a new enum with the provided [value].
  const ExerciseKind._(this.value);

  /// The underlying value of this enum member.
  final String value;

  @override
  String toString() => value;

  String toJson() => value;

  static const dumbbell = ExerciseKind._(r'dumbbell');
  static const barbell = ExerciseKind._(r'barbell');
  static const cable = ExerciseKind._(r'cable');
  static const machine = ExerciseKind._(r'machine');
  static const bodyweight = ExerciseKind._(r'bodyweight');

  /// List of all possible values in this [enum][ExerciseKind].
  static const values = <ExerciseKind>[
    dumbbell,
    barbell,
    cable,
    machine,
    bodyweight,
  ];

  static ExerciseKind? fromJson(dynamic value) => ExerciseKindTypeTransformer().decode(value);

  static List<ExerciseKind> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <ExerciseKind>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = ExerciseKind.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }
}

/// Transformation class that can [encode] an instance of [ExerciseKind] to String,
/// and [decode] dynamic data back to [ExerciseKind].
class ExerciseKindTypeTransformer {
  factory ExerciseKindTypeTransformer() => _instance ??= const ExerciseKindTypeTransformer._();

  const ExerciseKindTypeTransformer._();

  String encode(ExerciseKind data) => data.value;

  /// Decodes a [dynamic value][data] to a ExerciseKind.
  ///
  /// If [allowNull] is true and the [dynamic value][data] cannot be decoded successfully,
  /// then null is returned. However, if [allowNull] is false and the [dynamic value][data]
  /// cannot be decoded successfully, then an [UnimplementedError] is thrown.
  ///
  /// The [allowNull] is very handy when an API changes and a new enum value is added or removed,
  /// and users are still using an old app with the old code.
  ExerciseKind? decode(dynamic data, {bool allowNull = true}) {
    if (data != null) {
      switch (data) {
        case r'dumbbell': return ExerciseKind.dumbbell;
        case r'barbell': return ExerciseKind.barbell;
        case r'cable': return ExerciseKind.cable;
        case r'machine': return ExerciseKind.machine;
        case r'bodyweight': return ExerciseKind.bodyweight;
        default:
          if (!allowNull) {
            throw ArgumentError('Unknown enum value to decode: $data');
          }
      }
    }
    return null;
  }

  /// Singleton [ExerciseKindTypeTransformer] instance.
  static ExerciseKindTypeTransformer? _instance;
}

