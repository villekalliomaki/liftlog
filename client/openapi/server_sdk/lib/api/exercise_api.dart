//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;


class ExerciseApi {
  ExerciseApi([ApiClient? apiClient]) : apiClient = apiClient ?? defaultApiClient;

  final ApiClient apiClient;

  /// Performs an HTTP 'POST /api/exercise' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [CreateExerciseInput] createExerciseInput (required):
  Future<Response> createExerciseWithHttpInfo(CreateExerciseInput createExerciseInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise';

    // ignore: prefer_final_locals
    Object? postBody = createExerciseInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Parameters:
  ///
  /// * [CreateExerciseInput] createExerciseInput (required):
  Future<RouteSuccessExercise?> createExercise(CreateExerciseInput createExerciseInput,) async {
    final response = await createExerciseWithHttpInfo(createExerciseInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExercise',) as RouteSuccessExercise;
    
    }
    return null;
  }

  /// Performs an HTTP 'DELETE /api/exercise/{exercise_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseId (required):
  ///   The ID of the exercise requested
  Future<Response> deleteExerciseByIdWithHttpInfo(String exerciseId,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise/{exercise_id}'
      .replaceAll('{exercise_id}', exerciseId);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Parameters:
  ///
  /// * [String] exerciseId (required):
  ///   The ID of the exercise requested
  Future<RouteSuccessUuid?> deleteExerciseById(String exerciseId,) async {
    final response = await deleteExerciseByIdWithHttpInfo(exerciseId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessUuid',) as RouteSuccessUuid;
    
    }
    return null;
  }

  /// Performs an HTTP 'PATCH /api/exercise/{exercise_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseId (required):
  ///   The ID of the edited exercise
  ///
  /// * [EditExerciseInput] editExerciseInput (required):
  Future<Response> editExerciseWithHttpInfo(String exerciseId, EditExerciseInput editExerciseInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise/{exercise_id}'
      .replaceAll('{exercise_id}', exerciseId);

    // ignore: prefer_final_locals
    Object? postBody = editExerciseInput;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PATCH',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Parameters:
  ///
  /// * [String] exerciseId (required):
  ///   The ID of the edited exercise
  ///
  /// * [EditExerciseInput] editExerciseInput (required):
  Future<RouteSuccessExercise?> editExercise(String exerciseId, EditExerciseInput editExerciseInput,) async {
    final response = await editExerciseWithHttpInfo(exerciseId, editExerciseInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExercise',) as RouteSuccessExercise;
    
    }
    return null;
  }

  /// Performs an HTTP 'GET /api/exercise/{exercise_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseId (required):
  ///   The ID of the exercise requested
  Future<Response> getExerciseByIdWithHttpInfo(String exerciseId,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise/{exercise_id}'
      .replaceAll('{exercise_id}', exerciseId);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Parameters:
  ///
  /// * [String] exerciseId (required):
  ///   The ID of the exercise requested
  Future<RouteSuccessExercise?> getExerciseById(String exerciseId,) async {
    final response = await getExerciseByIdWithHttpInfo(exerciseId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExercise',) as RouteSuccessExercise;
    
    }
    return null;
  }

  /// Performs an HTTP 'GET /api/exercise/all' operation and returns the [Response].
  Future<Response> getUserExercisesWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise/all';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  Future<RouteSuccessExerciseVec?> getUserExercises() async {
    final response = await getUserExercisesWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExerciseVec',) as RouteSuccessExerciseVec;
    
    }
    return null;
  }

  /// Performs an HTTP 'GET /api/exercise/all/{exercise_kind}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseKind (required):
  ///   The ID of the exercise requested
  Future<Response> getUserExercisesByKindWithHttpInfo(String exerciseKind,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise/all/{exercise_kind}'
      .replaceAll('{exercise_kind}', exerciseKind);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Parameters:
  ///
  /// * [String] exerciseKind (required):
  ///   The ID of the exercise requested
  Future<RouteSuccessExerciseVec?> getUserExercisesByKind(String exerciseKind,) async {
    final response = await getUserExercisesByKindWithHttpInfo(exerciseKind,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExerciseVec',) as RouteSuccessExerciseVec;
    
    }
    return null;
  }
}
