The OpenAPI specification has to be copied manually here from a running build of the server. The route for the raw JSON file is `http://address/docs/spec/openapi.json`.

The Dart code is generated with [OpenAPI generator](https://github.com/OpenAPITools/openapi-generator). It can be installed however is more convenient, but as an example here is the installation of a wrapper with npm `npm install @openapitools/openapi-generator-cli`. **Install it while being in the `openapi` directory to contain all NodeJS files here!**

To generate Dart code run this **in this directory**:

```sh
npx @openapitools/openapi-generator-cli generate -i spec.json -g dart -o server_sdk/
```
