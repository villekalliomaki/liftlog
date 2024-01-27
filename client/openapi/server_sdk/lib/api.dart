//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

library openapi.api;

import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:collection/collection.dart';
import 'package:http/http.dart';
import 'package:intl/intl.dart';
import 'package:meta/meta.dart';

part 'api_client.dart';
part 'api_helper.dart';
part 'api_exception.dart';
part 'auth/authentication.dart';
part 'auth/api_key_auth.dart';
part 'auth/oauth.dart';
part 'auth/http_basic_auth.dart';
part 'auth/http_bearer_auth.dart';

part 'api/access_token_api.dart';
part 'api/exercise_api.dart';
part 'api/exercise_instance_api.dart';
part 'api/ping_api.dart';
part 'api/session_api.dart';
part 'api/set_api.dart';
part 'api/user_api.dart';

part 'model/access_token.dart';
part 'model/change_password_input.dart';
part 'model/change_username_input.dart';
part 'model/create_access_token_input.dart';
part 'model/create_exercise_input.dart';
part 'model/create_exercise_instance_comment_input.dart';
part 'model/create_exercise_instance_input.dart';
part 'model/create_session_input.dart';
part 'model/create_set_input.dart';
part 'model/create_user_input.dart';
part 'model/edit_exercise_input.dart';
part 'model/edit_exercise_instance_input.dart';
part 'model/edit_session_input.dart';
part 'model/edit_set_input.dart';
part 'model/exercise.dart';
part 'model/exercise_instance.dart';
part 'model/exercise_kind.dart';
part 'model/model_set.dart';
part 'model/route_error.dart';
part 'model/route_success_access_token.dart';
part 'model/route_success_exercise.dart';
part 'model/route_success_exercise_instance.dart';
part 'model/route_success_exercise_instance_vec.dart';
part 'model/route_success_exercise_vec.dart';
part 'model/route_success_session.dart';
part 'model/route_success_session_vec.dart';
part 'model/route_success_set.dart';
part 'model/route_success_string.dart';
part 'model/route_success_user.dart';
part 'model/route_success_usize.dart';
part 'model/route_success_uuid.dart';
part 'model/session.dart';
part 'model/set_exercise_instance_comment_input.dart';
part 'model/single_route_error.dart';
part 'model/user.dart';


/// An [ApiClient] instance that uses the default values obtained from
/// the OpenAPI specification file.
var defaultApiClient = ApiClient();

const _delimiters = {'csv': ',', 'ssv': ' ', 'tsv': '\t', 'pipes': '|'};
const _dateEpochMarker = 'epoch';
const _deepEquality = DeepCollectionEquality();
final _dateFormatter = DateFormat('yyyy-MM-dd');
final _regList = RegExp(r'^List<(.*)>$');
final _regSet = RegExp(r'^Set<(.*)>$');
final _regMap = RegExp(r'^Map<String,(.*)>$');

bool _isEpochMarker(String? pattern) => pattern == _dateEpochMarker || pattern == '/$_dateEpochMarker/';
