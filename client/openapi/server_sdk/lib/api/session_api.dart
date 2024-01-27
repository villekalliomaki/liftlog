//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;


class SessionApi {
  SessionApi([ApiClient? apiClient]) : apiClient = apiClient ?? defaultApiClient;

  final ApiClient apiClient;

  /// Performs an HTTP 'POST /api/session' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [CreateSessionInput] createSessionInput (required):
  Future<Response> createSessionWithHttpInfo(CreateSessionInput createSessionInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/session';

    // ignore: prefer_final_locals
    Object? postBody = createSessionInput;

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
  /// * [CreateSessionInput] createSessionInput (required):
  Future<RouteSuccessSession?> createSession(CreateSessionInput createSessionInput,) async {
    final response = await createSessionWithHttpInfo(createSessionInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessSession',) as RouteSuccessSession;
    
    }
    return null;
  }

  /// Performs an HTTP 'DELETE /api/session/{session_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] sessionId (required):
  ///   The ID of the session being deleted
  Future<Response> deleteSessionByIdWithHttpInfo(String sessionId,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/session/{session_id}'
      .replaceAll('{session_id}', sessionId);

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
  /// * [String] sessionId (required):
  ///   The ID of the session being deleted
  Future<RouteSuccessUuid?> deleteSessionById(String sessionId,) async {
    final response = await deleteSessionByIdWithHttpInfo(sessionId,);
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

  /// Performs an HTTP 'PATCH /api/session/{session_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] sessionId (required):
  ///   The ID of the session edited
  ///
  /// * [EditSessionInput] editSessionInput (required):
  Future<Response> editSessionWithHttpInfo(String sessionId, EditSessionInput editSessionInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/session/{session_id}'
      .replaceAll('{session_id}', sessionId);

    // ignore: prefer_final_locals
    Object? postBody = editSessionInput;

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
  /// * [String] sessionId (required):
  ///   The ID of the session edited
  ///
  /// * [EditSessionInput] editSessionInput (required):
  Future<RouteSuccessSession?> editSession(String sessionId, EditSessionInput editSessionInput,) async {
    final response = await editSessionWithHttpInfo(sessionId, editSessionInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessSession',) as RouteSuccessSession;
    
    }
    return null;
  }

  /// Performs an HTTP 'PATCH /api/session/{session_id}/finish' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] sessionId (required):
  ///   The ID of the finished session
  Future<Response> finishSessionWithHttpInfo(String sessionId,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/session/{session_id}/finish'
      .replaceAll('{session_id}', sessionId);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


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
  /// * [String] sessionId (required):
  ///   The ID of the finished session
  Future<RouteSuccessSession?> finishSession(String sessionId,) async {
    final response = await finishSessionWithHttpInfo(sessionId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessSession',) as RouteSuccessSession;
    
    }
    return null;
  }

  /// Performs an HTTP 'GET /api/session' operation and returns the [Response].
  Future<Response> getAllUserSessionsWithHttpInfo() async {
    // ignore: prefer_const_declarations
    final path = r'/api/session';

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

  Future<void> getAllUserSessions() async {
    final response = await getAllUserSessionsWithHttpInfo();
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }

  /// Performs an HTTP 'GET /api/session/{session_id}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] sessionId (required):
  ///   The ID of the session requested
  Future<Response> getSessionByIdWithHttpInfo(String sessionId,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/session/{session_id}'
      .replaceAll('{session_id}', sessionId);

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
  /// * [String] sessionId (required):
  ///   The ID of the session requested
  Future<void> getSessionById(String sessionId,) async {
    final response = await getSessionByIdWithHttpInfo(sessionId,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
  }
}
