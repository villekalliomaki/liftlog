# openapi.api.SetApi

## Load the API package
```dart
import 'package:openapi/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createSet**](SetApi.md#createset) | **POST** /api/set | 
[**deleteSet**](SetApi.md#deleteset) | **DELETE** /api/set/{set_id} | 
[**editSet**](SetApi.md#editset) | **PATCH** /api/set/{set_id} | 
[**getSetById**](SetApi.md#getsetbyid) | **GET** /api/set/{set_id} | 


# **createSet**
> RouteSuccessSet createSet(createSetInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SetApi();
final createSetInput = CreateSetInput(); // CreateSetInput | 

try {
    final result = api_instance.createSet(createSetInput);
    print(result);
} catch (e) {
    print('Exception when calling SetApi->createSet: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createSetInput** | [**CreateSetInput**](CreateSetInput.md)|  | 

### Return type

[**RouteSuccessSet**](RouteSuccessSet.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteSet**
> RouteSuccessUuid deleteSet(setId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SetApi();
final setId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the set

try {
    final result = api_instance.deleteSet(setId);
    print(result);
} catch (e) {
    print('Exception when calling SetApi->deleteSet: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **setId** | **String**| The ID of the set | 

### Return type

[**RouteSuccessUuid**](RouteSuccessUuid.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editSet**
> RouteSuccessSet editSet(setId, editSetInput)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SetApi();
final setId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the exercise instance being edited
final editSetInput = EditSetInput(); // EditSetInput | 

try {
    final result = api_instance.editSet(setId, editSetInput);
    print(result);
} catch (e) {
    print('Exception when calling SetApi->editSet: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **setId** | **String**| The ID of the exercise instance being edited | 
 **editSetInput** | [**EditSetInput**](EditSetInput.md)|  | 

### Return type

[**RouteSuccessSet**](RouteSuccessSet.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getSetById**
> RouteSuccessSet getSetById(setId)



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = SetApi();
final setId = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | The ID of the set

try {
    final result = api_instance.getSetById(setId);
    print(result);
} catch (e) {
    print('Exception when calling SetApi->getSetById: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **setId** | **String**| The ID of the set | 

### Return type

[**RouteSuccessSet**](RouteSuccessSet.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

