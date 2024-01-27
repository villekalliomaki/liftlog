# openapi.api.ExerciseInstanceApi

## Load the API package
```dart
import 'package:openapi/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addExerciseInstanceComment**](ExerciseInstanceApi.md#addexerciseinstancecomment) | **POST** /api/exercise_instance/{exercise_instance_id}/comment | 
[**createExerciseInstance**](ExerciseInstanceApi.md#createexerciseinstance) | **POST** /api/exercise_instance | 
[**deleteExerciseInstanceById**](ExerciseInstanceApi.md#deleteexerciseinstancebyid) | **DELETE** /api/exercise_instance/{exercise_instance_id} | 
[**deleteExerciseInstanceComment**](ExerciseInstanceApi.md#deleteexerciseinstancecomment) | **DELETE** /api/exercise_instance/{exercise_instance_id}/comment/{comment_index} | 
[**editExerciseInstance**](ExerciseInstanceApi.md#editexerciseinstance) | **PATCH** /api/exercise_instance/{exercise_instance_id} | 
[**getExerciseInstanceById**](ExerciseInstanceApi.md#getexerciseinstancebyid) | **GET** /api/exercise_instance/{exercise_instance_id} | 
[**setExerciseInstanceComment**](ExerciseInstanceApi.md#setexerciseinstancecomment) | **PATCH** /api/exercise_instance/{exercise_instance_id}/comment/{comment_index} | 


# **addExerciseInstanceComment**
> RouteSuccessExerciseInstance addExerciseInstanceComment(exerciseInstanceId, createExerciseInstanceCommentInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseInstanceApi();
final exerciseInstanceId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise instance being deleted
final createExerciseInstanceCommentInput = CreateExerciseInstanceCommentInput(); // CreateExerciseInstanceCommentInput | 

try {
    final result = api_instance.addExerciseInstanceComment(exerciseInstanceId, createExerciseInstanceCommentInput);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseInstanceApi->addExerciseInstanceComment: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseInstanceId** | **String**| The ID of the exercise instance being deleted | 
 **createExerciseInstanceCommentInput** | [**CreateExerciseInstanceCommentInput**](CreateExerciseInstanceCommentInput.md)|  | 

### Return type

[**RouteSuccessExerciseInstance**](RouteSuccessExerciseInstance.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createExerciseInstance**
> RouteSuccessExerciseInstance createExerciseInstance(createExerciseInstanceInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseInstanceApi();
final createExerciseInstanceInput = CreateExerciseInstanceInput(); // CreateExerciseInstanceInput | 

try {
    final result = api_instance.createExerciseInstance(createExerciseInstanceInput);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseInstanceApi->createExerciseInstance: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createExerciseInstanceInput** | [**CreateExerciseInstanceInput**](CreateExerciseInstanceInput.md)|  | 

### Return type

[**RouteSuccessExerciseInstance**](RouteSuccessExerciseInstance.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteExerciseInstanceById**
> RouteSuccessUuid deleteExerciseInstanceById(exerciseInstanceId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseInstanceApi();
final exerciseInstanceId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise instance being deleted

try {
    final result = api_instance.deleteExerciseInstanceById(exerciseInstanceId);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseInstanceApi->deleteExerciseInstanceById: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseInstanceId** | **String**| The ID of the exercise instance being deleted | 

### Return type

[**RouteSuccessUuid**](RouteSuccessUuid.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteExerciseInstanceComment**
> RouteSuccessUsize deleteExerciseInstanceComment(exerciseInstanceId, commentIndex)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseInstanceApi();
final exerciseInstanceId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise instance being edited
final commentIndex = 56; // int | Index of the comment in exercise instance

try {
    final result = api_instance.deleteExerciseInstanceComment(exerciseInstanceId, commentIndex);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseInstanceApi->deleteExerciseInstanceComment: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseInstanceId** | **String**| The ID of the exercise instance being edited | 
 **commentIndex** | **int**| Index of the comment in exercise instance | 

### Return type

[**RouteSuccessUsize**](RouteSuccessUsize.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editExerciseInstance**
> RouteSuccessExerciseInstance editExerciseInstance(exerciseInstanceId, editExerciseInstanceInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseInstanceApi();
final exerciseInstanceId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise instance being edited
final editExerciseInstanceInput = EditExerciseInstanceInput(); // EditExerciseInstanceInput | 

try {
    final result = api_instance.editExerciseInstance(exerciseInstanceId, editExerciseInstanceInput);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseInstanceApi->editExerciseInstance: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseInstanceId** | **String**| The ID of the exercise instance being edited | 
 **editExerciseInstanceInput** | [**EditExerciseInstanceInput**](EditExerciseInstanceInput.md)|  | 

### Return type

[**RouteSuccessExerciseInstance**](RouteSuccessExerciseInstance.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getExerciseInstanceById**
> getExerciseInstanceById(exerciseInstanceId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseInstanceApi();
final exerciseInstanceId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise instance

try {
    api_instance.getExerciseInstanceById(exerciseInstanceId);
} catch (e) {
    print('Exception when calling ExerciseInstanceApi->getExerciseInstanceById: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseInstanceId** | **String**| The ID of the exercise instance | 

### Return type

void (empty response body)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **setExerciseInstanceComment**
> RouteSuccessExerciseInstance setExerciseInstanceComment(exerciseInstanceId, commentIndex, setExerciseInstanceCommentInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = ExerciseInstanceApi();
final exerciseInstanceId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise instance being edited
final commentIndex = 56; // int | Index of the comment in exercise instance
final setExerciseInstanceCommentInput = SetExerciseInstanceCommentInput(); // SetExerciseInstanceCommentInput | 

try {
    final result = api_instance.setExerciseInstanceComment(exerciseInstanceId, commentIndex, setExerciseInstanceCommentInput);
    print(result);
} catch (e) {
    print('Exception when calling ExerciseInstanceApi->setExerciseInstanceComment: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **exerciseInstanceId** | **String**| The ID of the exercise instance being edited | 
 **commentIndex** | **int**| Index of the comment in exercise instance | 
 **setExerciseInstanceCommentInput** | [**SetExerciseInstanceCommentInput**](SetExerciseInstanceCommentInput.md)|  | 

### Return type

[**RouteSuccessExerciseInstance**](RouteSuccessExerciseInstance.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

