# openapi.api.PingApi

## Load the API package
```dart
import 'package:openapi/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle**](PingApi.md#handle) | **GET** /api/ping | 


# **handle**
> RouteSuccessString handle()



### Example
```dart
import 'package:openapi/api.dart';

final api_instance = PingApi();

try {
    final result = api_instance.handle();
    print(result);
} catch (e) {
    print('Exception when calling PingApi->handle: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**RouteSuccessString**](RouteSuccessString.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

