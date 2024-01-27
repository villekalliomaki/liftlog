//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class RouteSuccessSessionVec {
  /// Returns a new [RouteSuccessSessionVec] instance.
  RouteSuccessSessionVec({
    this.data = const [],
    required this.msg,
  });

  List<Session> data;

  String msg;

  @override
  bool operator ==(Object other) => identical(this, other) || other is RouteSuccessSessionVec &&
    _deepEquality.equals(other.data, data) &&
    other.msg == msg;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (data.hashCode) +
    (msg.hashCode);

  @override
  String toString() => 'RouteSuccessSessionVec[data=$data, msg=$msg]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'data'] = this.data;
      json[r'msg'] = this.msg;
    return json;
  }

  /// Returns a new [RouteSuccessSessionVec] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static RouteSuccessSessionVec? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "RouteSuccessSessionVec[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "RouteSuccessSessionVec[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return RouteSuccessSessionVec(
        data: Session.listFromJson(json[r'data']),
        msg: mapValueOfType<String>(json, r'msg')!,
      );
    }
    return null;
  }

  static List<RouteSuccessSessionVec> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <RouteSuccessSessionVec>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = RouteSuccessSessionVec.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, RouteSuccessSessionVec> mapFromJson(dynamic json) {
    final map = <String, RouteSuccessSessionVec>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = RouteSuccessSessionVec.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of RouteSuccessSessionVec-objects as value to a dart map
  static Map<String, List<RouteSuccessSessionVec>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<RouteSuccessSessionVec>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = RouteSuccessSessionVec.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'data',
    'msg',
  };
}

