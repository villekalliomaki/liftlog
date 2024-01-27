//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;


class ExerciseInstanceApi {
  ExerciseInstanceApi([ApiClient? apiClient]) : apiClient = apiClient ?? defaultApiClient;

  final ApiClient apiClient;

  /// Performs an HTTP 'POST /api/exercise_instance/{exercise_instance_id}/comment' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being deleted
  ///
  /// * [CreateExerciseInstanceCommentInput] createExerciseInstanceCommentInput (required):
  Future<Response> addExerciseInstanceCommentWithHttpInfo(String exerciseInstanceId, CreateExerciseInstanceCommentInput createExerciseInstanceCommentInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise_instance/{exercise_instance_id}/comment'
      .replaceAll('{exercise_instance_id}', exerciseInstanceId);

    // ignore: prefer_final_locals
    Object? postBody = createExerciseInstanceCommentInput;

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
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being deleted
  ///
  /// * [CreateExerciseInstanceCommentInput] createExerciseInstanceCommentInput (required):
  Future<RouteSuccessExerciseInstance?> addExerciseInstanceComment(String exerciseInstanceId, CreateExerciseInstanceCommentInput createExerciseInstanceCommentInput,) async {
    final response = await addExerciseInstanceCommentWithHttpInfo(exerciseInstanceId, createExerciseInstanceCommentInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExerciseInstance',) as RouteSuccessExerciseInstance;
    
    }
    return null;
  }

  /// Performs an HTTP 'POST /api/exercise_instance' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [CreateExerciseInstanceInput] createExerciseInstanceInput (required):
  Future<Response> createExerciseInstanceWithHttpInfo(CreateExerciseInstanceInput createExerciseInstanceInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise_instance';

    // ignore: prefer_final_locals
    Object? postBody = createExerciseInstanceInput;

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
  /// * [CreateExerciseInstanceInput] createExerciseInstanceInput (required):
  Future<RouteSuccessExerciseInstance?> createExerciseInstance(CreateExerciseInstanceInput createExerciseInstanceInput,) async {
    final response = await createExerciseInstanceWithHttpInfo(createExerciseInstanceInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExerciseInstance',) as RouteSuccessExerciseInstance;
    
    }
    return null;
  }

  /// Performs an HTTP 'DELETE /api/exercise_instance/{exercise_instance_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being deleted
  Future<Response> deleteExerciseInstanceByIdWithHttpInfo(String exerciseInstanceId,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise_instance/{exercise_instance_id}'
      .replaceAll('{exercise_instance_id}', exerciseInstanceId);

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
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being deleted
  Future<RouteSuccessUuid?> deleteExerciseInstanceById(String exerciseInstanceId,) async {
    final response = await deleteExerciseInstanceByIdWithHttpInfo(exerciseInstanceId,);
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

  /// Performs an HTTP 'DELETE /api/exercise_instance/{exercise_instance_id}/comment/{comment_index}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being edited
  ///
  /// * [int] commentIndex (required):
  ///   Index of the comment in exercise instance
  Future<Response> deleteExerciseInstanceCommentWithHttpInfo(String exerciseInstanceId, int commentIndex,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise_instance/{exercise_instance_id}/comment/{comment_index}'
      .replaceAll('{exercise_instance_id}', exerciseInstanceId)
      .replaceAll('{comment_index}', commentIndex.toString());

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
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being edited
  ///
  /// * [int] commentIndex (required):
  ///   Index of the comment in exercise instance
  Future<RouteSuccessUsize?> deleteExerciseInstanceComment(String exerciseInstanceId, int commentIndex,) async {
    final response = await deleteExerciseInstanceCommentWithHttpInfo(exerciseInstanceId, commentIndex,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessUsize',) as RouteSuccessUsize;
    
    }
    return null;
  }

  /// Performs an HTTP 'PATCH /api/exercise_instance/{exercise_instance_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being edited
  ///
  /// * [EditExerciseInstanceInput] editExerciseInstanceInput (required):
  Future<Response> editExerciseInstanceWithHttpInfo(String exerciseInstanceId, EditExerciseInstanceInput editExerciseInstanceInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise_instance/{exercise_instance_id}'
      .replaceAll('{exercise_instance_id}', exerciseInstanceId);

    // ignore: prefer_final_locals
    Object? postBody = editExerciseInstanceInput;

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
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being edited
  ///
  /// * [EditExerciseInstanceInput] editExerciseInstanceInput (required):
  Future<RouteSuccessExerciseInstance?> editExerciseInstance(String exerciseInstanceId, EditExerciseInstanceInput editExerciseInstanceInput,) async {
    final response = await editExerciseInstanceWithHttpInfo(exerciseInstanceId, editExerciseInstanceInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExerciseInstance',) as RouteSuccessExerciseInstance;
    
    }
    return null;
  }

  /// Performs an HTTP 'GET /api/exercise_instance/{exercise_instance_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance
  Future<Response> getExerciseInstanceByIdWithHttpInfo(String exerciseInstanceId,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise_instance/{exercise_instance_id}'
      .replaceAll('{exercise_instance_id}', exerciseInstanceId);

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
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance
  Future<void> getExerciseInstanceById(String exerciseInstanceId,) async {
    final response = await getExerciseInstanceByIdWithHttpInfo(exerciseInstanceId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Performs an HTTP 'PATCH /api/exercise_instance/{exercise_instance_id}/comment/{comment_index}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being edited
  ///
  /// * [int] commentIndex (required):
  ///   Index of the comment in exercise instance
  ///
  /// * [SetExerciseInstanceCommentInput] setExerciseInstanceCommentInput (required):
  Future<Response> setExerciseInstanceCommentWithHttpInfo(String exerciseInstanceId, int commentIndex, SetExerciseInstanceCommentInput setExerciseInstanceCommentInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/exercise_instance/{exercise_instance_id}/comment/{comment_index}'
      .replaceAll('{exercise_instance_id}', exerciseInstanceId)
      .replaceAll('{comment_index}', commentIndex.toString());

    // ignore: prefer_final_locals
    Object? postBody = setExerciseInstanceCommentInput;

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
  /// * [String] exerciseInstanceId (required):
  ///   The ID of the exercise instance being edited
  ///
  /// * [int] commentIndex (required):
  ///   Index of the comment in exercise instance
  ///
  /// * [SetExerciseInstanceCommentInput] setExerciseInstanceCommentInput (required):
  Future<RouteSuccessExerciseInstance?> setExerciseInstanceComment(String exerciseInstanceId, int commentIndex, SetExerciseInstanceCommentInput setExerciseInstanceCommentInput,) async {
    final response = await setExerciseInstanceCommentWithHttpInfo(exerciseInstanceId, commentIndex, setExerciseInstanceCommentInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessExerciseInstance',) as RouteSuccessExerciseInstance;
    
    }
    return null;
  }
}
