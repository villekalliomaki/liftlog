//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;


class AccessTokenApi {
  AccessTokenApi([ApiClient? apiClient]) : apiClient = apiClient ?? defaultApiClient;

  final ApiClient apiClient;

  /// Performs an HTTP 'POST /api/access_token' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [CreateAccessTokenInput] createAccessTokenInput (required):
  Future<Response> createAccessTokenWithHttpInfo(CreateAccessTokenInput createAccessTokenInput,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/access_token';

    // ignore: prefer_final_locals
    Object? postBody = createAccessTokenInput;

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
  /// * [CreateAccessTokenInput] createAccessTokenInput (required):
  Future<RouteSuccessAccessToken?> createAccessToken(CreateAccessTokenInput createAccessTokenInput,) async {
    final response = await createAccessTokenWithHttpInfo(createAccessTokenInput,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessAccessToken',) as RouteSuccessAccessToken;
    
    }
    return null;
  }

  /// Performs an HTTP 'DELETE /api/access_token/{token}' operation and returns the [Response].
  /// Parameters:
  ///
  /// * [String] token (required):
  ///   The token being deleted
  Future<Response> deleteTokenWithHttpInfo(String token,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/access_token/{token}'
      .replaceAll('{token}', token);

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
  /// * [String] token (required):
  ///   The token being deleted
  Future<RouteSuccessString?> deleteToken(String token,) async {
    final response = await deleteTokenWithHttpInfo(token,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RouteSuccessString',) as RouteSuccessString;
    
    }
    return null;
  }
}
