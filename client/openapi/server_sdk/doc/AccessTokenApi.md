# openapi.api.AccessTokenApi

## Load the API package
```dart
import 'package:openapi/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createAccessToken**](AccessTokenApi.md#createaccesstoken) | **POST** /api/access_token | 
[**deleteToken**](AccessTokenApi.md#deletetoken) | **DELETE** /api/access_token/{token} | 


# **createAccessToken**
> RouteSuccessAccessToken createAccessToken(createAccessTokenInput)



### Example
```dart
import 'package:openapi/api.dart';

final api_instance = AccessTokenApi();
final createAccessTokenInput = CreateAccessTokenInput(); // CreateAccessTokenInput | 

try {
    final result = api_instance.createAccessToken(createAccessTokenInput);
    print(result);
} catch (e) {
    print('Exception when calling AccessTokenApi->createAccessToken: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createAccessTokenInput** | [**CreateAccessTokenInput**](CreateAccessTokenInput.md)|  | 

### Return type

[**RouteSuccessAccessToken**](RouteSuccessAccessToken.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteToken**
> RouteSuccessString deleteToken(token)



### Example
```dart
import 'package:openapi/api.dart';

final api_instance = AccessTokenApi();
final token = token_example; // String | The token being deleted

try {
    final result = api_instance.deleteToken(token);
    print(result);
} catch (e) {
    print('Exception when calling AccessTokenApi->deleteToken: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| The token being deleted | 

### Return type

[**RouteSuccessString**](RouteSuccessString.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

