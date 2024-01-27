# openapi.api.ExerciseApi

## Load the API package
```dart
import 'package:openapi/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createExercise**](ExerciseApi.md#createexercise) | **POST** /api/exercise | 
[**deleteExerciseById**](ExerciseApi.md#deleteexercisebyid) | **DELETE** /api/exercise/{exercise_id} | 
[**editExercise**](ExerciseApi.md#editexercise) | **PATCH** /api/exercise/{exercise_id} | 
[**getExerciseById**](ExerciseApi.md#getexercisebyid) | **GET** /api/exercise/{exercise_id} | 
[**getUserExercises**](ExerciseApi.md#getuserexercises) | **GET** /api/exercise/all | 
[**getUserExercisesByKind**](ExerciseApi.md#getuserexercisesbykind) | **GET** /api/exercise/all/{exercise_kind} | 


# **createExercise**
> RouteSuccessExercise createExercise(createExerciseInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseApi();
final createExerciseInput = CreateExerciseInput(); // CreateExerciseInput | 

try {
    final result = api_instance.createExercise(createExerciseInput);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseApi->createExercise: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createExerciseInput** | [**CreateExerciseInput**](CreateExerciseInput.md)|  | 

### Return type

[**RouteSuccessExercise**](RouteSuccessExercise.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteExerciseById**
> RouteSuccessUuid deleteExerciseById(exerciseId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseApi();
final exerciseId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise requested

try {
    final result = api_instance.deleteExerciseById(exerciseId);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseApi->deleteExerciseById: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseId** | **String**| The ID of the exercise requested | 

### Return type

[**RouteSuccessUuid**](RouteSuccessUuid.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editExercise**
> RouteSuccessExercise editExercise(exerciseId, editExerciseInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseApi();
final exerciseId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the edited exercise
final editExerciseInput = EditExerciseInput(); // EditExerciseInput | 

try {
    final result = api_instance.editExercise(exerciseId, editExerciseInput);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseApi->editExercise: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseId** | **String**| The ID of the edited exercise | 
 **editExerciseInput** | [**EditExerciseInput**](EditExerciseInput.md)|  | 

### Return type

[**RouteSuccessExercise**](RouteSuccessExercise.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getExerciseById**
> RouteSuccessExercise getExerciseById(exerciseId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseApi();
final exerciseId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise requested

try {
    final result = api_instance.getExerciseById(exerciseId);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseApi->getExerciseById: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseId** | **String**| The ID of the exercise requested | 

### Return type

[**RouteSuccessExercise**](RouteSuccessExercise.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUserExercises**
> RouteSuccessExerciseVec getUserExercises()



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseApi();

try {
    final result = api_instance.getUserExercises();
    print(result);
} catch (e) {
    print('Exception when calling ExerciseApi->getUserExercises: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**RouteSuccessExerciseVec**](RouteSuccessExerciseVec.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUserExercisesByKind**
> RouteSuccessExerciseVec getUserExercisesByKind(exerciseKind)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseApi();
final exerciseKind = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise requested

try {
    final result = api_instance.getUserExercisesByKind(exerciseKind);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseApi->getUserExercisesByKind: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseKind** | **String**| The ID of the exercise requested | 

### Return type

[**RouteSuccessExerciseVec**](RouteSuccessExerciseVec.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

