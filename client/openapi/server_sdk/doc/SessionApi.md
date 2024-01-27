# openapi.api.SessionApi

## Load the API package
```dart
import 'package:openapi/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createSession**](SessionApi.md#createsession) | **POST** /api/session | 
[**deleteSessionById**](SessionApi.md#deletesessionbyid) | **DELETE** /api/session/{session_id} | 
[**editSession**](SessionApi.md#editsession) | **PATCH** /api/session/{session_id} | 
[**finishSession**](SessionApi.md#finishsession) | **PATCH** /api/session/{session_id}/finish | 
[**getAllUserSessions**](SessionApi.md#getallusersessions) | **GET** /api/session | 
[**getSessionById**](SessionApi.md#getsessionbyid) | **GET** /api/session/{session_id} | 


# **createSession**
> RouteSuccessSession createSession(createSessionInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SessionApi();
final createSessionInput = CreateSessionInput(); // CreateSessionInput | 

try {
    final result = api_instance.createSession(createSessionInput);
    print(result);
} catch (e) {
    print('Exception when calling SessionApi->createSession: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createSessionInput** | [**CreateSessionInput**](CreateSessionInput.md)|  | 

### Return type

[**RouteSuccessSession**](RouteSuccessSession.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteSessionById**
> RouteSuccessUuid deleteSessionById(sessionId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SessionApi();
final sessionId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the session being deleted

try {
    final result = api_instance.deleteSessionById(sessionId);
    print(result);
} catch (e) {
    print('Exception when calling SessionApi->deleteSessionById: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String**| The ID of the session being deleted | 

### Return type

[**RouteSuccessUuid**](RouteSuccessUuid.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editSession**
> RouteSuccessSession editSession(sessionId, editSessionInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SessionApi();
final sessionId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the session edited
final editSessionInput = EditSessionInput(); // EditSessionInput | 

try {
    final result = api_instance.editSession(sessionId, editSessionInput);
    print(result);
} catch (e) {
    print('Exception when calling SessionApi->editSession: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String**| The ID of the session edited | 
 **editSessionInput** | [**EditSessionInput**](EditSessionInput.md)|  | 

### Return type

[**RouteSuccessSession**](RouteSuccessSession.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **finishSession**
> RouteSuccessSession finishSession(sessionId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SessionApi();
final sessionId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the finished session

try {
    final result = api_instance.finishSession(sessionId);
    print(result);
} catch (e) {
    print('Exception when calling SessionApi->finishSession: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String**| The ID of the finished session | 

### Return type

[**RouteSuccessSession**](RouteSuccessSession.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllUserSessions**
> getAllUserSessions()



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SessionApi();

try {
    api_instance.getAllUserSessions();
} catch (e) {
    print('Exception when calling SessionApi->getAllUserSessions: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getSessionById**
> getSessionById(sessionId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SessionApi();
final sessionId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the session requested

try {
    api_instance.getSessionById(sessionId);
} catch (e) {
    print('Exception when calling SessionApi->getSessionById: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String**| The ID of the session requested | 

### Return type

void (empty response body)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

