//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class RouteSuccessAccessToken {
  /// Returns a new [RouteSuccessAccessToken] instance.
  RouteSuccessAccessToken({
    required this.data,
    required this.msg,
  });

  AccessToken data;

  String msg;

  @override
  bool operator ==(Object other) => identical(this, other) || other is RouteSuccessAccessToken &&
    other.data == data &&
    other.msg == msg;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (data.hashCode) +
    (msg.hashCode);

  @override
  String toString() => 'RouteSuccessAccessToken[data=$data, msg=$msg]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'data'] = this.data;
      json[r'msg'] = this.msg;
    return json;
  }

  /// Returns a new [RouteSuccessAccessToken] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static RouteSuccessAccessToken? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "RouteSuccessAccessToken[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "RouteSuccessAccessToken[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return RouteSuccessAccessToken(
        data: AccessToken.fromJson(json[r'data'])!,
        msg: mapValueOfType<String>(json, r'msg')!,
      );
    }
    return null;
  }

  static List<RouteSuccessAccessToken> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <RouteSuccessAccessToken>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = RouteSuccessAccessToken.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, RouteSuccessAccessToken> mapFromJson(dynamic json) {
    final map = <String, RouteSuccessAccessToken>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = RouteSuccessAccessToken.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of RouteSuccessAccessToken-objects as value to a dart map
  static Map<String, List<RouteSuccessAccessToken>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<RouteSuccessAccessToken>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = RouteSuccessAccessToken.listFromJson(entry.value, growable: growable,);
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

