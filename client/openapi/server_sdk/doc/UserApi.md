# openapi.api.UserApi

## Load the API package
```dart
import 'package:openapi/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**changePassword**](UserApi.md#changepassword) | **PATCH** /api/user/password | 
[**changeUsername**](UserApi.md#changeusername) | **PATCH** /api/user/username | 
[**createUser**](UserApi.md#createuser) | **POST** /api/user | 
[**deleteUser**](UserApi.md#deleteuser) | **DELETE** /api/user | 
[**getSelf**](UserApi.md#getself) | **GET** /api/user | 


# **changePassword**
> RouteSuccessUser changePassword()



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = UserApi();

try {
    final result = api_instance.changePassword();
    print(result);
} catch (e) {
    print('Exception when calling UserApi->changePassword: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**RouteSuccessUser**](RouteSuccessUser.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **changeUsername**
> RouteSuccessUser changeUsername()



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = UserApi();

try {
    final result = api_instance.changeUsername();
    print(result);
} catch (e) {
    print('Exception when calling UserApi->changeUsername: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**RouteSuccessUser**](RouteSuccessUser.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createUser**
> RouteSuccessUser createUser(createUserInput)



### Example
```dart
import 'package:openapi/api.dart';

final api_instance = UserApi();
final createUserInput = CreateUserInput(); // CreateUserInput | 

try {
    final result = api_instance.createUser(createUserInput);
    print(result);
} catch (e) {
    print('Exception when calling UserApi->createUser: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createUserInput** | [**CreateUserInput**](CreateUserInput.md)|  | 

### Return type

[**RouteSuccessUser**](RouteSuccessUser.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteUser**
> RouteSuccessUuid deleteUser()



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = UserApi();

try {
    final result = api_instance.deleteUser();
    print(result);
} catch (e) {
    print('Exception when calling UserApi->deleteUser: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**RouteSuccessUuid**](RouteSuccessUuid.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getSelf**
> RouteSuccessUser getSelf()



### Example
```dart
import 'package:openapi/api.dart';
// TODO Configure HTTP Bearer authorization: access_token
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('access_token').setAccessToken(yourTokenGeneratorFunction);

final api_instance = UserApi();

try {
    final result = api_instance.getSelf();
    print(result);
} catch (e) {
    print('Exception when calling UserApi->getSelf: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**RouteSuccessUser**](RouteSuccessUser.md)

### Authorization

[access_token](../README.md#access_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

