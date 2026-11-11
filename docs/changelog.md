## [Unreleased]

## [0.3.793] — 2026-11-12

View:

- **Function List** lists OpenTelemetry OTLP JSON export keys: `"resourceSpans": [`, `"scopeSpans": [`, `"resourceLogs": [`, `"scopeLogs": [`, `"resourceMetrics": [`, `"scopeMetrics": [`, `"instrumentationScope": {`, `"parentSpanId": ""` / `"…"`, `"startTimeUnixNano": "…"` / `0`, `"endTimeUnixNano": "…"` / `0`, `"droppedAttributesCount": 0` / `1`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"traceId"` / `"spanId"` / `"attributes": [` / `"events": [` / `"links": [` / `"status": {` / `"kind"` / `"service"` / HAR `"startedDateTime"` / `"timings"` / OpenAPI `"openapi"` / AsyncAPI `"asyncapi"` / `"channels": {` / Pact `"consumer"` / MockServer `"httpRequest"` / Hoverfly `"hoverflyVersion"` / Mountebank `"imposters"` / WireMock `"scenarioName"` / Mockoon `"lastMigration"` / Hoppscotch `"preRequestScript"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger-ui `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / Knip `"entry"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"parentSpanId"` only matches empty or 16-char lowercase hex. Unix-nano strings are digit-only (1–20 chars).

## [0.3.792] — 2026-11-11

View:

- **Function List** lists HTTP Archive (HAR 1.2) entry / Chrome-export JSON keys: `"startedDateTime": "2009-04-16T15:53:27.373Z"`, `"timings": {`, `"serverIPAddress": ""` / `"…"`, `"pageref": ""` / `"…"`, `"_resourceType": ""` / `"…"`, `"_webSocketMessages": [`, `"_priority": ""` / `"…"`, `"_fromCache": ""` / `"…"`, `"_initiator": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"log": {` / `"version"` / `"creator": {` / `"browser": {` / `"pages": [` / `"entries": [` / `"request": {` / `"response": {` / `"cache": {` / `"comment"` / `"time"` / `"headers": [` / `"cookies": [` / `"queryString": [` / `"content": {` / `"status"` / `"method"` / `"url"` / `"$schema"` / OpenAPI `"openapi"` / AsyncAPI `"asyncapi"` / `"channels": {` / Pact `"consumer"` / MockServer `"httpRequest"` / Hoverfly `"hoverflyVersion"` / Mountebank `"imposters"` / WireMock `"scenarioName"` / Mockoon `"lastMigration"` / Hoppscotch `"preRequestScript"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger-ui `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / Knip `"entry"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"startedDateTime"` only matches ISO-8601 date prefixes (`YYYY-MM-DD…`).

## [0.3.791] — 2026-11-11

View:

- **Function List** lists AsyncAPI 2 / 3 document JSON keys: `"asyncapi": "2.0.0"` / `"2.6.0"` / `"3.0.0"`, `"channels": {`, `"operations": {`, `"servers": {` (object map; OpenAPI uses `"servers": [`), `"defaultContentType": ""` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"info": {` / `"id"` / `"tags": [` / `"components": {` / `"externalDocs": {` / OpenAPI `"openapi"` / `"swagger"` / `"paths": {` / `"servers": [` / `"webhooks": {` / Pact `"consumer"` / `"provider"` / `"interactions"` / MockServer `"httpRequest"` / Hoverfly `"hoverflyVersion"` / Mountebank `"imposters"` / WireMock `"scenarioName"` / Mockoon `"lastMigration"` / Hoppscotch `"preRequestScript"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger-ui `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / Knip `"entry"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"asyncapi"` only matches `2.N.N` / `3.N.N` version strings.

## [0.3.790] — 2026-11-11

View:

- **Function List** lists OpenAPI 3 / Swagger 2 document JSON keys: `"openapi": "3.0.0"` / `"3.0.3"` / `"3.1.0"`, `"swagger": "2.0"`, `"paths": {`, `"components": {`, `"servers": [`, `"webhooks": {`, `"jsonSchemaDialect": ""` / `"…"`, `"securityDefinitions": {`, `"externalDocs": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"info": {` / `"tags": [` / `"security": [` / `"host"` / `"basePath"` / `"schemes": [` / `"consumes": [` / `"produces": [` / `"definitions": {` / `"parameters": {` / `"responses": {` / `"request": {` / `"response": {` / redocly object `"openapi": {` / Pact `"consumer"` / `"provider"` / `"interactions"` / `"matchingRules"` / `"pactSpecification"` / MockServer `"httpRequest"` / `"httpResponse"` / Hoverfly `"hoverflyVersion"` / Mountebank `"imposters"` / WireMock `"scenarioName"` / Mockoon `"lastMigration"` / Hoppscotch `"preRequestScript"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger-ui `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / Knip `"entry"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"openapi"` only matches `3.N.N` version strings. `"swagger"` only matches `"2.0"`. Shared `"paths": {` also matches Knip when shapes overlap; Cucumber `"paths": [` / string stays on Cucumber.

## [0.3.789] — 2026-11-11

View:

- **Function List** lists Pact contract JSON keys: `"consumer": {`, `"provider": {`, `"interactions": [`, `"matchingRules": {`, `"providerStates": [`, `"providerState": ""` / `"…"`, `"pactSpecification": {`, `"interactionMarkup": ""` / `"…"`, `"interactionMarkupType": "HTML"` / `"MARKDOWN"`, `"pact:matcher:type": ""` / `"…"`, `"pact:generator:type": ""` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"request": {` / `"response": {` / `"description"` / `"method"` / `"path"` / `"headers": {` / `"body"` / `"status"` / `"metadata": {` / `"generators": {` (Nx) / `"pending"` / `"messages": [` / MockServer `"httpRequest"` / `"httpResponse"` / `"dropConnection"` / Hoverfly `"hoverflyVersion"` / `"schemaVersion"` / `"fixedDelay"` / `"bodyFile"` / Mountebank `"imposters"` / `"predicates"` / `"deepEquals"` / WireMock `"scenarioName"` / `"requiredScenarioState"` / `"fault"` / `"postServeActions"` / `"bodyFileName"` / `"fixedDelayMilliseconds"` / Mockoon `"lastMigration"` / `"endpointPrefix"` / `"bodyType"` / Hoppscotch `"preRequestScript"` / `"authType"` / Yaak `"yaakVersion"` / `"httpRequests"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"interactionMarkupType"` only matches Pact HTML / MARKDOWN enums.

## [0.3.788] — 2026-11-11

View:

- **Function List** lists MockServer expectation / verification JSON keys: `"httpRequest": {`, `"httpResponse": {`, `"httpResponseTemplate": {`, `"httpResponseClassCallback": {`, `"httpResponseObjectCallback": {`, `"httpForward": {`, `"httpForwardTemplate": {`, `"httpForwardClassCallback": {`, `"httpForwardObjectCallback": {`, `"httpOverrideForwardedRequest": {`, `"httpError": {`, `"times": {`, `"timeToLive": {`, `"socketAddress": {`, `"dropConnection": true` / `false`, `"responseBytes": ""` / `"…"`, `"remainingTimes": 0` / `1`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"request": {` / `"response": {` / `"method"` / `"path"` / `"headers": {` / `"body"` / `"statusCode"` / `"priority"` / `"unlimited"` / `"pathParameters": {` / `"queryStringParameters": {` / Yaak `"httpRequests": [` / Hoverfly `"hoverflyVersion"` / `"schemaVersion"` / `"fixedDelay"` / `"bodyFile"` / Mountebank `"imposters"` / `"predicates"` / `"deepEquals"` / WireMock `"scenarioName"` / `"requiredScenarioState"` / `"fault"` / `"postServeActions"` / `"bodyFileName"` / `"fixedDelayMilliseconds"` / Mockoon `"lastMigration"` / `"endpointPrefix"` / `"bodyType"` / Hoppscotch `"preRequestScript"` / `"authType"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out.

## [0.3.787] — 2026-11-11

View:

- **Function List** lists Hoverfly simulation JSON keys: `"hoverflyVersion": "v…"`, `"schemaVersion": "v5"` / `"v5.2"`, `"timeExported": "…"`, `"globalActions": {`, `"delaysLogNormal": [`, `"encodedBody": true` / `false`, `"fixedDelay": 0` / `2000`, `"logNormalDelay": {`, `"postServeAction": ""` / `"…"`, `"removesState": [`, `"transitionsState": {`, `"requiresState": {`, `"templated": true` / `false`, `"bodyFile": ""` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"data": {` / `"meta": {` / `"pairs": [` / `"request": {` / `"response": {` / `"status"` / `"body"` / `"headers": {` / `"matcher"` / `"value"` / `"delays": [` / `"literals": [` / `"variables": [` / Mountebank `"imposters"` / `"predicates"` / `"deepEquals"` / WireMock `"scenarioName"` / `"requiredScenarioState"` / `"fault"` / `"postServeActions"` / `"bodyFileName"` / `"fixedDelayMilliseconds"` / Mockoon `"lastMigration"` / `"endpointPrefix"` / `"bodyType"` / Hoppscotch `"preRequestScript"` / `"authType"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"schemaVersion"` only matches Hoverfly `vN` / `vN.M` / `vN.M.P` forms.

## [0.3.786] — 2026-11-11

View:

- **Function List** lists Mountebank imposters / stub / predicate JSON keys: `"imposters": [`, `"recordRequests": true` / `false`, `"allowInjection": true` / `false`, `"defaultResponse": {`, `"numberOfRequests": 0` / `3`, `"predicates": [`, `"deepEquals": {`, `"inject": ""` / `"…"`, `"decorate": ""` / `"…"`, `"shellTransform": ""` / `"…"`, `"behaviors": [`, `"concatenateArrays": true` / `false`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"port"` / `"protocol"` / `"stubs": [` / `"responses": [` / `"is": {` / `"proxy": {` / `"equals": {` / `"matches": {` / `"contains": {` / `"_links": {` / WireMock `"scenarioName"` / `"requiredScenarioState"` / `"fault"` / Mockoon `"lastMigration"` / `"endpointPrefix"` / `"bodyType"` / Hoppscotch `"preRequestScript"` / `"authType"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out.

## [0.3.785] — 2026-11-11

View:

- **Function List** lists WireMock stub-mapping JSON keys: `"scenarioName": ""` / `"…"`, `"requiredScenarioState": ""` / `"…"`, `"newScenarioState": ""` / `"…"`, `"postServeActions": {`, `"fixedDelayMilliseconds": 0` / `250`, `"bodyFileName": ""` / `"…"`, `"base64Body": ""` / `"…"`, `"proxyBaseUrl": ""` / `"…"`, `"urlPathPattern": ""` / `"…"`, `"bodyPatterns": [`, `"fault": "CONNECTION_RESET_BY_PEER"` / `"EMPTY_RESPONSE"` / `"MALFORMED_RESPONSE_CHUNK"` / `"RANDOM_DATA_THEN_CLOSE"`, `"jsonBody": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"id"` / `"uuid"` / `"priority"` / `"request": {` / `"response": {` / `"persistent"` / `"metadata": {` / `"method"` / `"url"` / `"urlPath"` / `"urlPattern"` / `"headers": {` / `"status"` / `"body"` / `"transformers": [` / Mockoon `"lastMigration"` / `"endpointPrefix"` / `"bodyType"` / Hoppscotch `"preRequestScript"` / `"authType"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"fault"` only matches WireMock fault enums.

## [0.3.784] — 2026-11-11

View:

- **Function List** lists Mockoon environment / route-response JSON keys: `"lastMigration": 33`, `"endpointPrefix": ""` / `"api"`, `"proxyMode": true` / `false`, `"proxyHost": ""` / `"…"`, `"proxyRemovePrefix": true` / `false`, `"rootChildren": [`, `"proxyReqHeaders": [`, `"proxyResHeaders": [`, `"headersCaseInsensitive": true` / `false`, `"enableRandomLatency": true` / `false`, `"tlsOptions": {`, `"bodyType": "INLINE"` / `"FILE"` / `"DATABUCKET"`, `"rulesOperator": "OR"` / `"AND"`, `"databucketID": ""` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"uuid"` / `"hostname"` / `"port"` / `"latency"` / `"cors"` / `"folders": [` / `"routes": [` / `"headers": [` / `"data": [` / `"callbacks": [` / Hoppscotch `"preRequestScript"` / `"authType"` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / `"protocolProfileBehavior"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"bodyType"` / `"rulesOperator"` only match Mockoon enums.

## [0.3.783] — 2026-11-11

View:

- **Function List** lists Hoppscotch collection / request / history JSON keys: `"preRequestScript": ""` / `"…"`, `"testScript": ""` / `"…"`, `"requestVariables": [`, `"_ref_id": "req_…"`, `"authType": "inherit"` / `"oauth-2"` / `"api-key"` / `"aws-signature"` / `"hawk"` / `"akamai-eg"` / `"jwt"` / `"none"` / `"basic"` / `"bearer"` / `"digest"`, `"authActive": true` / `false`, `"responseMeta": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"endpoint"` / `"method"` / `"folders": [` / `"requests": [` / `"environments": [` / Yaak `"yaakVersion"` / Bruno `"brunoConfig"` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / `"protocolProfileBehavior"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"_ref_id"` only matches Hoppscotch `prefix_timestamp_uuid` refs. `"authType"` only matches Hoppscotch auth enums.


## [0.3.782] — 2026-11-11

View:

- **Function List** lists Yaak workspace-export JSON keys: `"yaakVersion": "…"`, `"yaakSchema": 4`, `"httpRequests": [`, `"grpcRequests": [`, `"websocketRequests": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"timestamp"` / `"resources": {` / `"workspaces": [` / `"folders": [` / `"environments": [` / Bruno `"brunoConfig"` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / `"protocolProfileBehavior"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out.

## [0.3.781] — 2026-11-11

View:

- **Function List** lists Bruno `bruno.json` / collection-export JSON keys: `"brunoConfig": {`, `"clientCertificates": {`, `"moduleWhitelist": [`, `"additionalContextRoots": [`, `"collectionVersion": "…"`, `"bypassProxy": ""` / `"…"`, `"opencollection": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"type"` / `"items": [` / `"environments": [` / `"ignore": [` / `"scripts": {` / `"presets": {` / `"proxy": {` / Thunder Client `"client"` / `"collectionName"` / Postman `"_postman_id"` / `"protocolProfileBehavior"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"opencollection"` only matches `true`.

## [0.3.780] — 2026-11-11

View:

- **Function List** lists Thunder Client collection / environment / request export JSON keys: `"client": "Thunder Client"`, `"collectionName": "…"`, `"environmentName": "…"`, `"dateExported": "…"`, `"colId": "…"`, `"containerId": ""` / `"…"`, `"sortNum": 10000`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"url"` / `"method"` / `"folders": [` / `"requests": [` / `"variables": [` / Postman `"_postman_id"` / `"protocolProfileBehavior"` / Insomnia `"__export_format"` / `"_type"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"client"` only matches the exact Thunder Client product string.

## [0.3.779] — 2026-11-11

View:

- **Function List** lists Insomnia export / resource JSON keys: `"__export_format": 4`, `"__export_date": "…"`, `"__export_source": "insomnia.desktop.app:v…"`, `"_type": "export"` / `"workspace"` / `"request"` / `"request_group"` / `"environment"` / `"cookie_jar"` / `"api_spec"` / `"unit_test"` / `"unit_test_suite"` / `"proto_file"` / `"proto_directory"` / `"grpc_request"` / `"web_socket_request"` / `"socket_io_request"`, `"_id": "req_…"` / `"wrk_…"` / `"fld_…"` / `"env_…"` / `"jar_…"` / `"spc_…"` / `"uts_…"` / `"ut_…"` / `"pf_…"` / `"pd_…"` / `"grpc_req_…"` / `"ws-req_…"` / `"socket_…"`, `"metaSortKey": -1` / `100`, `"settingFollowRedirects": "global"` / `"on"` / `"off"`, bools `"settingStoreCookies"` / `"settingSendCookies"` / `"settingDisableRenderRequestBody"` / `"settingEncodeUrl"` / `"settingRebuildPath"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"url"` / `"method"` / `"parentId"` / `"resources": [` / Postman `"_postman_id"` / `"protocolProfileBehavior"` / ReDoc `"disableSearch"` / RapiDoc `"specUrl"` / swagger `"dom_id"` / scalar `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"__export_source"` only matches when the value mentions insomnia. `"_id"` only matches Insomnia id prefixes. `"_type"` only matches the Insomnia resource/export enums.

## [0.3.778] — 2026-11-11

View:

- **Function List** lists Postman Collection v2.x / Environment / Globals export JSON keys: `"_postman_id": "…"`, `"_exporter_id": "…"`, `"_collection_link": "…"`, `"_postman_variable_scope": "environment"` / `"globals"` / `"collection"`, `"_postman_exported_at": "…"`, `"_postman_exported_using": "Postman/…"`, `"_postman_previewlanguage": "json"` / `"xml"` / `"html"` / `"text"` / `"javascript"` / `"auto"` / `"plain"` / `"graphql"`, `"schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"` / `v2.0.0`, `"protocolProfileBehavior": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"url"` / `"item": [` / `"event": [` / `"variable": [` / `"auth": {` / `"request": {` / `"info": {` / swagger `"dom_id"` / `"layout": "BaseLayout"` / Elements `"layout": "sidebar"` / `"apiDescriptionUrl"` / scalar `"proxyUrl"` / `"layout": "modern"` / RapiDoc `"specUrl"` / `"renderStyle"` / ReDoc `"disableSearch"` / `"sideNavStyle"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / `"licenseKey"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. `"schema"` only matches when the value is a Postman collection schema URL.

## [0.3.777] — 2026-11-11

View:

- **Function List** lists ReDoc standalone options JSON keys (distinct from Redocly CLI): `"disableSearch": true`, `"minCharacterLengthToInitSearch": 3`, `"expandDefaultServerVariables": true`, `"expandResponses": "200,201"` / `"all"` / `"2XX"`, `"maxDisplayedEnumValues": 10`, `"hideDownloadButton": true`, `"downloadFileName": "…"`, `"downloadDefinitionUrl": "…"`, `"hideHostname": true`, `"hideLoading": true`, `"jsonSampleExpandLevel": 2` / `"all"`, `"hideSingleRequestSampleTab": true`, `"hideRequestPayloadSample": true`, `"expandSingleSchemaField": true`, `"schemaExpansionLevel": 2` / `"all"`, `"showObjectSchemaExamples": true`, `"showExtensions": true` / `[`, `"hideSchemaPattern": true`, `"generatedPayloadSamplesMaxItems": 10`, `"hideSchemaTitles": true`, `"simpleOneOfTypeLabel": true`, `"sortEnumValuesAlphabetically": true`, `"sortOperationsAlphabetically": true`, `"sortPropsAlphabetically": true`, `"sortTagsAlphabetically": true`, `"menuToggle": true`, `"nativeScrollbars": true`, `"onlyRequiredInSamples": true`, `"pathInMiddlePanel": true`, `"requiredPropsFirst": true`, `"scrollYOffset": 60` / `"#navbar"`, `"showWebhookVerb": true`, `"payloadSampleIdx": 0`, `"nonce": "…"`, `"sideNavStyle": "summary-only"` / `"path-only"` / `"id-only"`, `"untrustedSpec": true`, `"theme": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"url"` / swagger `"dom_id"` / `"layout": "BaseLayout"` / Elements `"layout": "sidebar"` / `"apiDescriptionUrl"` / scalar `"proxyUrl"` / `"layout": "modern"` / RapiDoc `"specUrl"` / `"renderStyle"` / `"theme": "light"` / mintlify `"topbarLinks"` / `"theme": "mint"` / Compodoc `"theme": "material"` / redocly `"apis"` / `"licenseKey"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. Shared `"hideDownloadButton": true` also matches Scalar when shapes overlap. ReDoc `"theme": {` stays here; string theme enums stay on RapiDoc / Mintlify / Compodoc / Scalar.

## [0.3.776] — 2026-11-11

View:

- **Function List** lists RapiDoc config JSON keys: `"specUrl": "…"`, `"theme": "light"` / `"dark"`, `"renderStyle": "read"` / `"view"` / `"focused"`, `"schemaStyle": "tree"` / `"table"`, `"defaultSchemaTab": "model"` / `"example"`, `"responseAreaHeight": "400px"` / `"50vh"` / `"30%"` / `"2.5em"`, `"apiKeyName": "…"`, `"apiKeyLocation": "header"` / `"query"` / `"cookie"`, `"apiKeyValue": "…"`, `"serverUrl": "…"`, `"navItemSpacing": "default"` / `"compact"` / `"relaxed"`, `"fontSize": "default"` / `"large"` / `"largest"`, `"sortEndpointsBy": "path"` / `"method"` / `"summary"` / `"none"`, `"goToPath": "…"`, `"matchPaths": "…"`, `"matchType": "includes"` / `"equals"` / `"starts-with"` / `"regex"`, `"headingText": "…"`, `"showMethodInNavBar": "as-plain-text"` / `"as-colored-text"` / `"as-colored-block"` / `false`, `"routePrefix": "…"`, `"monoFont": "…"`, `"regularFont": "…"`, colors `"bgColor"` / `"textColor"` / `"headerColor"` / `"primaryColor"` / `"navBgColor"` / `"navTextColor"` / `"navHoverBgColor"` / `"navHoverTextColor"` / `"navAccentColor"`, bools `"showHeader"` / `"showInfo"` / `"allowAuthentication"` / `"allowTry"` / `"allowServerSelection"` / `"allowApiListStyleSelection"` / `"fillRequestFieldsWithExample"` / `"usePathInNavBar"` / `"sortTags"` / `"regexPathFiltering"` / `"infoDescriptionHeadingsInNavBar"` / `"showComponents"` / `"updateRoute"` / `"resolveRelativePaths"` / `"loadFonts"` / `"showCurlBeforeTry"` / `"persistAuth"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"url"` / swagger `"dom_id"` / `"layout": "BaseLayout"` / Elements `"layout": "sidebar"` / `"apiDescriptionUrl"` / scalar `"proxyUrl"` / `"layout": "modern"` / mintlify `"topbarLinks"` / `"theme": "mint"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. Shared `"persistAuth": true` also matches Scalar when shapes overlap. RapiDoc `"theme": "light"` / `"dark"` stays here; Mintlify / Compodoc / Scalar theme enums stay on theirs.

## [0.3.775] — 2026-11-11

View:

- **Function List** lists Swagger UI `swagger-config.json` / `swagger-config.yaml` JSON-shaped keys: `"configUrl": "…"`, `"dom_id": "#swagger-ui"`, `"urls": [`, `"spec": {`, `"syntaxHighlight": {` / `true`, `"supportedSubmitMethods": [`, `"layout": "BaseLayout"` / `"StandaloneLayout"`, `"defaultModelRendering": "example"` / `"model"`, `"docExpansion": "list"` / `"full"` / `"none"`, `"operationsSorter": "alpha"` / `"method"`, `"tagsSorter": "alpha"`, `"validatorUrl": "…"` / `"none"` / `"null"` / `"localhost"` / `"127.0.0.1"`, `"oauth2RedirectUrl": "…"`, `"filter": "pet"` / `true`, bools `"deepLinking"` / `"displayOperationId"` / `"displayRequestDuration"` / `"showExtensions"` / `"showCommonExtensions"` / `"tryItOutEnabled"` / `"withCredentials"` / `"persistAuthorization"` / `"queryConfigEnabled"` / `"requestSnippetsEnabled"`, numbers `"defaultModelsExpandDepth": -1` / `"defaultModelExpandDepth": 1` / `"maxDisplayedTags": 10`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"url"` / `"plugins": [` / `"presets": [` / karma `"basePath"` / scalar `"proxyUrl"` / `"layout": "modern"` / Elements `"layout": "sidebar"` / mintlify `"topbarLinks"` / Compodoc `"theme": "material"` / redocly `"apis"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` stay out. Swagger `"layout": "BaseLayout"` / `"StandaloneLayout"` stays here; Scalar `"layout": "modern"` / `"classic"` and Elements `"layout": "sidebar"` / `"stacked"` / `"responsive"` stay on their scanners.

## [0.3.774] — 2026-11-11

View:

- **Function List** lists Stoplight Elements `elements-api` / API React component JSON keys: `"apiDescriptionUrl": "…"`, `"apiDescriptionDocument": {` / `"…"`, `"layout": "sidebar"` / `"stacked"` / `"responsive"`, `"router": "hash"` / `"memory"` / `"history"` / `"static"`, `"logo": "…"`, `"staticRouterPath": "…"`, `"tryItCorsProxy": "…"`, `"tryItCredentialsPolicy": "omit"` / `"include"` / `"same-origin"`, bools `"hideTryIt"` / `"hideTryItPanel"` / `"hideSamples"` / `"hideSchemas"` / `"hideInternal"` / `"hideExport"` / `"hideSecurityInfo"` / `"hideServerInfo"` / `"outerRouter"`, number `"maxRefDepth": 2`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / karma `"basePath"` / scalar `"proxyUrl"` / `"layout": "modern"` / mintlify `"topbarLinks"` / `"theme": "mint"` / Compodoc `"theme": "material"` / redocly `"apis"` / `"licenseKey"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` stay out. Shared `"logo"` string paths also match Mintlify / Scalar when shapes overlap. Elements `"layout": "sidebar"` stays here; Scalar `"layout": "modern"` / `"classic"` stays on Scalar.

## [0.3.773] — 2026-11-11

View:

- **Function List** lists Scalar `scalar.config.json` / API Reference JSON keys: `"proxyUrl": "…"`, `"baseServerURL": "…"`, `"searchHotKey": "k"`, `"forceDarkModeState": "dark"` / `"light"`, `"layout": "modern"` / `"classic"`, `"theme": "default"` / `"alternate"` / `"moon"` / `"purple"` / `"solarized"` / `"bluePlanet"` / `"saturn"` / `"kepler"` / `"mars"` / `"deepSpace"` / `"laserwave"` / `"none"`, `"customCss": "…"`, `"subdomain": "…"`, `"customDomain": "…"`, `"favicon": "…"`, `"logo": "…"`, `"hiddenClients": [`, `"guides": [`, `"defaultHttpClient": {`, `"pathRouting": {`, `"metaData": {`, `"siteConfig": {`, `"authentication": {`, bools `"isEditable"` / `"showSidebar"` / `"hideModels"` / `"hideDownloadButton"` / `"hideTestRequestButton"` / `"darkMode"` / `"hideDarkModeToggle"` / `"withDefaultFonts"` / `"defaultOpenAllTags"` / `"expandAllModelSections"` / `"expandAllResponses"` / `"orderRequiredPropertiesFirst"` / `"hideClientButton"` / `"hideSearch"` / `"persistAuth"` / `"showOperationId"` / `"telemetry"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / mintlify `"topbarLinks"` / `"theme": "mint"` / Compodoc `"theme": "material"` / redocly `"apis"` / `"licenseKey"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. Shared `"hideDarkModeToggle": true` also matches Compodoc when shapes overlap; shared `"favicon"` / `"logo"` string paths also match Mintlify when shapes overlap.

## [0.3.772] — 2026-11-11

View:

- **Function List** lists Mintlify `mint.json` / `docs.json` JSON keys: `"topbarLinks": [`, `"anchors": [`, `"navigation": [` / `{`, `"tabs": [`, `"versions": [`, `"redirects": [`, `"topbarCtaButton": {`, `"footerSocials": {`, `"modeToggle": {`, `"colors": {`, `"logo": {` / `"…"`, `"api": {`, `"feedback": {`, `"analytics": {`, `"integrations": {`, `"navbar": {`, `"footer": {`, `"favicon": "…"`, `"theme": "mint"` / `"palm"` / `"willow"` / `"linden"` / `"almond"` / `"aspen"` / `"peat"` / `"flower"` / `"spruce"` / `"rainfrog"` / `"luxe"` / `"maple"` / `"hazel"` / `"venus"` / `"mercury"` / `"neo"` / `"mintlify"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / redocly `"apis"` / `"licenseKey"` / spectral `"functionsDir"` / openapitools `"generator-cli"` / `"spaces"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / Compodoc `"theme": "material"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. Shared `"redirects": [` also matches vercel when shapes overlap.

## [0.3.771] — 2026-11-11

View:

- **Function List** lists Redocly `redocly.yaml` / `redocly.json` / `.redocly.yaml` JSON-shaped keys: `"apis": {`, `"decorators": {`, `"preprocessors": {`, `"resolve": {`, `"theme": {`, `"openapi": {`, `"licenseKey": "…"`, `"extends": "recommended"` / `"minimal"` / `"recommended-strict"` / path / URL, `"extends": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"rules": {` / `"plugins": [` / spectral `"functionsDir"` / `"extends": "spectral:oas"` / openapitools `"generator-cli"` / `"spaces"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / eslint `"extends": "eslint:recommended"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` / OpenAPI string `"openapi": "3.0.0"` / Compodoc string `"theme": "material"` stay out. Shared `"preprocessors": {` also matches karma when shapes overlap; shared `"extends": [` also matches spectral / eslint / stylelint / babel when shapes overlap.

## [0.3.770] — 2026-11-11

View:

- **Function List** lists Stoplight Spectral `.spectral.json` JSON keys: `"functionsDir": "…"`, `"functions": [`, `"formats": [`, `"aliases": {`, `"except": {`, `"documentationUrl": "https://…"` / `http://…` / relative path, `"extends": "spectral:oas"` / `"spectral:asyncapi"` / `"@stoplight/…"`, `"extends": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"rules": {` / `"overrides": [` / openapitools `"generator-cli"` / `"spaces"` / docfx `"metadata": [` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / eslint `"extends": "eslint:recommended"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. Shared `"extends": [` also matches eslint / stylelint / babel when shapes overlap; string `"extends"` needs a `spectral:` or `@stoplight/` value so eslint / tsconfig path extends stay on their scanners.

## [0.3.769] — 2026-11-11

View:

- **Function List** lists OpenAPI Generator CLI `openapitools.json` JSON keys: `"generator-cli": {`, `"spaces": 2`, `"storageDir": "…"`, `"useDocker": true` / `false`, `"dockerImageName": "…"`, `"generatorName": "typescript-fetch"` / `"java"` / …, `"inputSpec": "…"` / `https://…`, `"glob": "**/openapi.{json,yaml}"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"output": "…"` / `"generators": {` / `"disabled": true` / `"additionalProperties": {` / `"config": "…"` / docfx `"metadata": [` / compodoc `"output"` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / nx `"pluginsConfig"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out.

## [0.3.768] — 2026-11-11

View:

- **Function List** lists DocFX `docfx.json` JSON keys: `"metadata": [`, `"overwrite": [`, `"xref": [`, `"xrefService": [`, `"globalMetadataFiles": [`, `"fileMetadataFiles": [`, `"postProcessors": [`, `"globalMetadata": {`, `"fileMetadata": {`, `"markdownEngineProperties": {`, `"sitemap": {`, `"markdownEngineName": "markdig"` / `"dfm"`, `"rawModelOutputFolder": "…"`, `"viewModelOutputFolder": "…"`, bools `"noLangKeyword"` / `"keepFileLink"` / `"cleanupCacheHistory"` / `"disableGitFeatures"` / `"exportRawModel"` / `"exportViewModel"` / `"forcePostProcess"`, numbers `"lruSize"` / `"maxParallelism"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"files"` / `"include": [` / `"exclude": [` / `"build": {` / `"content": [` / `"resource": [` / `"template": [` / `"dest": "…"` / `"output": "…"` / playwright `"metadata": {` / compodoc `"output"` / typedoc `"entryPoints"` / api-extractor `"mainEntryPointFilePath"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. DocFX `"metadata": [` is array-only so playwright object `"metadata": {` stays on playwright.

## [0.3.767] — 2026-11-11

View:

- **Function List** lists Compodoc `.compodocrc` / `.compodocrc.json` / `compodoc.json` JSON keys: `"tsconfig": "tsconfig.doc.json"`, `"output": "documentation"`, `"theme": "material"` / `"gitbook"` / …, `"extTheme": "…"`, `"language": "en-US"`, `"assetsFolder": "…"`, `"includes": "./additional-doc"` (string path; ESDoc uses `[`), `"includesName": "…"`, `"unitTestCoverage": "…"`, `"templates": "…"`, `"host": "…"`, `"customFavicon": "…"`, `"customLogo": "…"`, `"gaID": "…"`, `"gaSite": "auto"`, `"exportFormat": "html"` / `"json"`, `"publicApiOnly": "…"`, `"coverageTestThresholdFail": "true"` / `"false"`, `"coverageExclude": [` / `"…"`, `"navTabConfig": [`, `"toggleMenuItems": [`, `"port": 8080`, `"coverageTest": 70`, `"coverageMinimumPerFile": 50`, bools `"open"` / `"silent"` / `"serve"` / `"watch"` / `"hideGenerator"` / `"hideDarkModeToggle"` / `"coverageTestShowOnlyFailed"` / `"minimal"` / `"templatePlayground"` / `"disableSourceCode"` / `"disableDomTree"` / `"disableTemplateTab"` / `"disableStyleTab"` / `"disableGraph"` / `"disableCoverage"` / `"disablePrivate"` / `"disableProtected"` / `"disableInternal"` / `"disableLifeCycleHooks"` / `"disableConstructors"` / `"disableRoutesGraph"` / `"disableSearch"` / `"disableDependencies"` / `"disableProperties"` / `"disableFilePath"` / `"disableOverview"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"files"` / `"include": [` / `"exclude": [` / esdoc `"source": "…"` / `"destination"` / `"includes": [` / `"undocumentIdentifier"` / jsdoc `"source": {` / `"recurseDepth"` / typedoc `"entryPoints"` / `"excludePrivate"` / api-extractor `"mainEntryPointFilePath"` / `"apiReport"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. Compodoc `"hideGenerator": true` also matches typedoc when shapes overlap; `"tsconfig": "…"` / `"theme": "…"` / `"watch": true` are intentionally off the typedoc scanner so Compodoc owns those shapes here.

## [0.3.766] — 2026-11-11

View:

- **Function List** lists ESDoc `.esdoc.json` / `esdoc.json` JSON keys (v0.5 + current top-level): `"source": "./src"`, `"destination": "./docs"`, `"includes": [`, `"excludes": [`, `"index": "./README.md"`, `"package": "./package.json"`, `"outputAST": true`, `"plugins": [`, `"access": [`, `"autoPrivate": true`, `"undocumentIdentifier": true`, `"unexportIdentifier": false`, `"builtinExternal": true`, `"coverage": true`, `"includeSource": true`, `"test": {`, `"title": "…"`, `"styles": [`, `"scripts": [`, `"manual": {`, `"lint": true`, `"experimentalProposal": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"extends"` / `"name"` / `"version"` / `"include": [` / `"exclude": [` / `"readme"` / `"private": true` / jsdoc `"source": {` / `"sourceType"` / `"recurseDepth"` / `"opts": {` / `"allowUnknownTags"` / typedoc `"entryPoints"` / `"excludePrivate"` / api-extractor `"mainEntryPointFilePath"` / `"apiReport"` / vitest `"coverage": {` / `"includeSource": [` / package.json `"scripts": {` / prettier `"printWidth"` / karma `"basePath"` stay out. ESDoc `"includes": [` / `"excludes": [` also match dprint when shapes overlap; `"plugins": [` also matches jsdoc when shapes overlap; `"package": "…"` / `"destination": "…"` also match jsdoc when shapes overlap.

## [0.3.765] — 2026-11-10

View:

- **Function List** lists JSDoc `conf.json` / `jsdoc.json` / `.jsdoc.json` / `jsdoc.conf.json` JSON keys: `"plugins": [`, `"recurseDepth": 10`, `"source": {`, `"sourceType": "module"` / `"script"`, `"opts": {`, `"tags": {`, `"templates": {`, `"allowUnknownTags": true`, `"dictionaries": [`, `"cleverLinks": false`, `"monospaceLinks": false`, `"includePattern": "…"`, `"excludePattern": "…"`, `"destination": "…"`, `"encoding": "utf8"`, `"template": "…"`, `"tutorials": "…"`, `"package": "…"`, `"readme": "…"`, `"access": "all"` / `"private"` / `"protected"` / `"public"` / `"undefined"`, `"pedantic": true`, `"recurse": true`, `"debug": true`, `"test": true`, `"outputSourceFiles": true`, `"includeDate": true`, `"useLongnameInNav": true`, `"layoutFile": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"extends"` / `"name"` / `"version"` / `"include": [` / `"exclude": [` / `"default": {` / `"private": true` / typedoc `"entryPoints"` / `"excludePrivate"` / api-extractor `"mainEntryPointFilePath"` / `"apiReport"` / syncpack `"semverGroups"` / versionrc `"bumpFiles"` / babel `"sourceType": "unambiguous"` / mocha `"opts": "…"` (string) / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out. JSDoc `"dictionaries": [` also matches cspell when shapes overlap; `"sourceType": "module"` / `"script"` also match babel; `"access": "public"` also matches changesets when shapes overlap.

## [0.3.764] — 2026-11-10

View:

- **Function List** lists TypeDoc `typedoc.json` / `typedoc.jsonc` JSON keys: `"entryPoints": [`, `"entryPointStrategy": "resolve"` / `"expand"` / `"packages"`, `"excludePrivate": true`, `"excludeProtected": true`, `"excludeInternal": true`, `"excludeExternals": true`, `"excludeNotDocumented": true`, `"disableSources": true`, `"includeVersion": true`, `"categorizeByGroup": true`, `"searchInComments": true`, `"searchInDocuments": true`, `"cleanOutputDir": true`, `"hideGenerator": true`, `"treatWarningsAsErrors": true`, `"treatValidationWarningsAsErrors": true`, `"useHostedBaseUrlForAbsoluteLinks": true`, `"useTsLinkResolution": true`, `"skipErrorChecking": true`, `"preserveLinkText": true`, `"githubPages": true`, `"sourceLinkTemplate": "…"`, `"gitRevision": "…"`, `"gitRemote": "…"`, `"defaultCategory": "…"`, `"titleLink": "…"`, `"hostedBaseUrl": "https://…"`, `"categoryOrder": [`, `"kindSortOrder": [`, `"navigationLeaves": [`, `"highlightLanguages": [`, `"intentionallyNotExported": [`, `"requiredToBeDocumented": [`, `"projectDocuments": [`, `"visibilityFilters": {`, `"validation": {`, `"navigationLinks": {`, `"sidebarLinks": {`, `"jsDocCompatibility": {`, `"outputs": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"extends"` / `"name"` / `"out"` / `"json"` / `"readme"` / `"theme"` / `"plugin"` / `"tsconfig"` / `"compilerOptions"` / `"pretty"` / `"watch"` / api-extractor `"mainEntryPointFilePath"` / `"apiReport"` / syncpack `"semverGroups"` / versionrc `"bumpFiles"` / release-please `"packages": {` / rush `"rushVersion"` / prettier `"printWidth"` / package.json `"bin"` / karma `"basePath"` stay out.

## [0.3.763] — 2026-11-10

View:

- **Function List** lists API Extractor `api-extractor.json` JSON keys: `"mainEntryPointFilePath": "<projectFolder>/lib/index.d.ts"`, `"bundledPackages": [`, `"newlineKind": "lf"` / `"crlf"` / `"os"`, `"enumMemberOrder": "by-name"` / `"preserve"`, `"testMode": true`, `"apiReport": {`, `"docModel": {`, `"dtsRollup": {`, `"tsdocMetadata": {`, `"reportFileName": "…"`, `"reportFolder": "…"`, `"reportTempFolder": "…"`, `"reportVariants": [`, `"includeForgottenExports": true`, `"apiJsonFilePath": "…"`, `"projectFolderUrl": "…"`, `"untrimmedFilePath": "…"`, `"alphaTrimmedFilePath": "…"`, `"betaTrimmedFilePath": "…"`, `"publicTrimmedFilePath": "…"`, `"omitTrimmingComments": true`, `"tsdocMetadataFilePath": "…"`, `"overrideTsconfig": {`, `"compilerMessageReporting": {`, `"extractorMessageReporting": {`, `"tsdocMessageReporting": {`, `"addToApiReportFile": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"extends"` / `"projectFolder"` / `"compiler"` / `"messages"` / `"enabled"` / `"name"` / `"version"` / syncpack `"semverGroups"` / versionrc `"bumpFiles"` / release-please `"packages": {` / rush `"rushVersion"` / tsconfig `"compilerOptions"` / `"skipLibCheck"` / prettier `"printWidth"` / package.json `"bin"` stay out.

## [0.3.762] — 2026-11-10

View:

- **Function List** lists syncpack `.syncpackrc` / `.syncpackrc.json` JSON keys: `"semverGroups": [`, `"versionGroups": [`, `"dependencyGroups": [`, `"customTypes": {`, `"sortAz": [`, `"sortFirst": [`, `"sortExports": [`, `"source": [`, `"indent": "  "`, `"sourceMode": "replace"` / `"extend"`, `"sortPackages": true`, `"strict": true`, `"formatBugs": true`, `"formatRepository": true`, `"maxConcurrentRequests": 12`, `"minimumReleaseAge": 1440`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / versionrc `"bumpFiles"` / `"packageFiles"` / `"infile"` / release-please `"packages": {` / `"bootstrap-sha"` / `"release-type"` / simple-git-hooks `"pre-commit"` / `"preserveUnused"` / lint-staged `"chunkSize"` / `"hidePartiallyStaged"` / semantic-release `"repositoryUrl"` / `"tagFormat"` / release-it `"requireCleanWorkingDir"` / `"hooks": {` / `"plugins": {` / commitlint `"parserPreset"` / `"defaultIgnores"` / prettier `"printWidth"` / package.json `"bin"` / tsconfig `"compilerOptions"` stay out. syncpack `"strict": true` also matches tsconfig when shapes overlap.

## [0.3.761] — 2026-11-10

View:

- **Function List** lists standard-version / commit-and-tag-version `.versionrc` / `.versionrc.json` JSON keys: `"bumpFiles": [`, `"packageFiles": [`, `"infile": "CHANGELOG.md"`, `"header": "…"`, `"types": [`, `"preMajor": true`, `"commitUrlFormat": "…"`, `"compareUrlFormat": "…"`, `"issueUrlFormat": "…"`, `"userUrlFormat": "…"`, `"releaseCommitMessageFormat": "…"`, `"issuePrefixes": [`, `"skip": {`, `"scripts": {`, `"gitTagFallback": true`, `"firstRelease": true`, `"noVerify": true`, `"commitAll": true`, `"silent": true`, `"tagPrefix": "v"`, `"releaseAs": "1.2.3"`, `"changelogHeaderPattern": "…"`, `"preset": "conventionalcommits"`, `"releaseCount": 0`, `"lernaPackage": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"sign"` / `"path"` / `"dryRun"` / release-please `"packages": {` / `"bootstrap-sha"` / `"release-type"` / `"include-v-in-tag"` / simple-git-hooks `"pre-commit"` / `"preserveUnused"` / lint-staged `"chunkSize"` / `"hidePartiallyStaged"` / semantic-release `"repositoryUrl"` / `"tagFormat"` / release-it `"requireCleanWorkingDir"` / `"hooks": {` / `"plugins": {` / commitlint `"parserPreset"` / `"defaultIgnores"` / prettier `"printWidth"` / package.json `"bin"` / tsconfig `"compilerOptions"` stay out. versionrc `"scripts": {` also matches package.json when shapes overlap; `"preset": "…"` / `"releaseAs": "…"` also match semantic-release / release-please when shapes overlap.

## [0.3.760] — 2026-11-10

View:

- **Function List** lists release-please `release-please-config.json` JSON keys: `"packages": {`, `"bootstrap-sha": "…"`, `"last-release-sha": "…"`, `"always-link-local": true`, `"plugins": [`, `"signoff": "…"`, `"group-pull-request-title-pattern": "…"`, `"release-search-depth": 10`, `"commit-search-depth": 500`, `"commit-batch-size": 10`, `"sequential-calls": true`, `"release-type": "node"`, `"bump-minor-pre-major": true`, `"bump-patch-for-minor-pre-major": true`, `"prerelease-type": "…"`, `"versioning": "default"`, `"changelog-sections": [`, `"release-as": "1.2.3"`, `"skip-github-release": true`, `"skip-changelog": true`, `"draft": true`, `"force-tag-creation": true`, `"prerelease": true`, `"draft-pull-request": true`, `"label": "…"`, `"release-label": "…"`, `"extra-label": "…"`, `"include-component-in-tag": true`, `"include-v-in-tag": true`, `"include-v-in-release-name": true`, `"changelog-type": "github"`, `"changelog-host": "…"`, `"changelog-path": "CHANGELOG.md"`, `"include-commit-authors": true`, `"pull-request-title-pattern": "…"`, `"pull-request-header": "…"`, `"pull-request-footer": "…"`, `"separate-pull-requests": true`, `"always-update": true`, `"tag-separator": "/"`, `"date-format": "%Y-%m-%d"`, `"extra-files": [`, `"exclude-paths": [`, `"version-file": "…"`, `"snapshot-label": "…"`, `"skip-snapshot": true`, `"initial-version": "0.1.0"`, `"component-no-space": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / lerna `"packages": [` / `"npmClient"` / simple-git-hooks `"pre-commit"` / `"preserveUnused"` / lint-staged `"chunkSize"` / `"hidePartiallyStaged"` / semantic-release `"repositoryUrl"` / `"tagFormat"` / release-it `"requireCleanWorkingDir"` / `"hooks": {` / `"plugins": {` / commitlint `"parserPreset"` / `"defaultIgnores"` / prettier `"printWidth"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. release-please `"packages": {` (object); lerna `"packages": [` stay on those scanners. Shared `"plugins": [` also match neighbouring scanners when shapes overlap.

## [0.3.759] — 2026-11-10

View:

- **Function List** lists simple-git-hooks `.simple-git-hooks.json` / `package.json` `"simple-git-hooks"` JSON keys: git hook command lines (`"pre-commit": "npx lint-staged"`, `"pre-push": "npm test"`, `"commit-msg": "npx commitlint --edit $1"`, `"prepare-commit-msg": "…"`, `"post-checkout": "…"`, `"post-merge": "…"`, `"pre-rebase": "…"`, `"post-commit": "…"`, `"post-rewrite": "…"`, `"pre-auto-gc": "…"`, `"applypatch-msg": "…"`, `"pre-applypatch": "…"`, `"post-applypatch": "…"`, `"pre-merge-commit": "…"`, `"pre-receive": "…"`, `"update": "…"`, `"post-receive": "…"`, `"post-update": "…"`, `"push-to-checkout": "…"`, `"sendemail-validate": "…"`), plus `"preserveUnused": true` / `[`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / lint-staged `"chunkSize"` / `"hidePartiallyStaged"` / `"diffFilter"` / `"globOptions"` / `"*.js"` globs / semantic-release `"repositoryUrl"` / `"tagFormat"` / release-it `"hooks": {` / `"requireCleanWorkingDir"` / commitlint `"parserPreset"` / `"defaultIgnores"` / prettier `"printWidth"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out.

## [0.3.758] — 2026-11-10

View:

- **Function List** lists lint-staged `.lintstagedrc` / `.lintstagedrc.json` / `package.json` `"lint-staged"` JSON keys: `"chunkSize": 10`, `"maxArgLength": 4096`, `"hidePartiallyStaged": true`, `"hideUnstaged": true`, `"diffFilter": "ACMR"`, `"globOptions": {`, `"concurrent": true` / `2`, `"relative": true`, `"shell": true` / `"/bin/bash"`, `"stash": true`, `"verbose": true`, `"quiet": true`, `"diff": "HEAD"`, plus micromatch glob task lines (`"*.js": "eslint --fix"`, `"*.{js,ts}": [`, `"src/**/*.ts": "…"`). Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / semantic-release `"repositoryUrl"` / `"tagFormat"` / `"presetConfig"` / release-it `"requireCleanWorkingDir"` / `"preReleaseId"` / `"hooks": {` / `"plugins": {` / commitlint `"parserPreset"` / `"defaultIgnores"` / `"helpUrl"` / `"prompt"` / biome `"formatter": {` / eslint `"parser"` / `"env"` / `"languageOptions"` / prettier `"printWidth"` / stylelint `"ignoreFiles"` / node-tap `"coverage-map"` / `"allow-empty-coverage"` / c8 `"src"` / nyc `"temp-dir"` / `"branches": 80` / Cucumber `"requireModule"` / Stryker `"mutate"` / Detox `"apps"` / Codecept `"helpers": {` / WDIO `"waitforTimeout"` / TestCafe `"selectorTimeout"` / Karma `"frameworks"` / Nightwatch `"src_folders"` / Jasmine `"spec_dir"` / AVA `"failFast"` / Mocha `"allowUncaught"` / Jest `"testEnvironment"` / Playwright `"testDir"` / Vitest `"pool"` / Cypress `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / package.json `"scripts"` / tsconfig `"compilerOptions"` / shared `"cwd"` / `"ignore"` / `"debug"` stay out. Glob keys need `*` or `{…}`; bare `"eslint": "…"` stays out.

## [0.3.757] — 2026-11-10

View:

- **Function List** lists semantic-release `.releaserc` / `.releaserc.json` / `package.json` `"release"` JSON keys: `"repositoryUrl": "…"`, `"tagFormat": "v${version}"` (must contain `${version}`), `"branches": "…"` / `[`, `"plugins": [`, `"extends": "…"` / `[`, `"preset": "…"`, `"presetConfig": {`, `"ci": true`, `"dryRun": true`, `"debug": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / release-it `"requireCleanWorkingDir"` / `"preReleaseId"` / `"increment"` / `"hooks": {` / `"plugins": {` / `"git": {` / `"npm": {` / `"github": {` / `"gitlab": {` / commitlint `"parserPreset"` / `"defaultIgnores"` / `"helpUrl"` / `"prompt"` / biome `"formatter": {` / eslint `"parser"` / `"env"` / `"languageOptions"` / prettier `"printWidth"` / stylelint `"ignoreFiles"` / node-tap `"coverage-map"` / `"allow-empty-coverage"` / c8 `"src"` / nyc `"temp-dir"` / `"branches": 80` / Cucumber `"requireModule"` / Stryker `"mutate"` / Detox `"apps"` / Codecept `"helpers": {` / WDIO `"waitforTimeout"` / TestCafe `"selectorTimeout"` / Karma `"frameworks"` / Nightwatch `"src_folders"` / Jasmine `"spec_dir"` / AVA `"failFast"` / Mocha `"allowUncaught"` / Jest `"testEnvironment"` / Playwright `"testDir"` / Vitest `"pool"` / Cypress `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. semantic-release `"plugins": [` / `"branches": [` / `"branches": "…"`; release-it `"plugins": {`; nyc / c8 / tap `"branches": 80` stay on those scanners. Shared `"extends"` / `"plugins": [` / `"ci"` / `"dryRun"` / `"debug"` also match neighbouring scanners when shapes overlap.

## [0.3.756] — 2026-11-10

View:

- **Function List** lists release-it `.release-it.json` / `package.json` `"release-it"` JSON keys: `"requireCleanWorkingDir": true`, `"requireUpstream": true`, `"requireCommits": true`, `"requireCommitsFail": true`, `"addUntrackedFiles": true`, `"preReleaseId": "…"`, `"onlyVersion": true`, `"skipChecks": true`, `"increment": "patch"` / `"minor"` / `"major"` / `"conventional:…"` / `false`, `"hooks": {`, `"plugins": {`, `"git": {`, `"npm": {`, `"github": {`, `"gitlab": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / commitlint `"parserPreset"` / `"defaultIgnores"` / `"helpUrl"` / `"prompt"` / biome `"formatter": {` / eslint `"parser"` / `"env"` / `"languageOptions"` / prettier `"printWidth"` / stylelint `"ignoreFiles"` / node-tap `"coverage-map"` / `"allow-empty-coverage"` / c8 `"src"` / nyc `"temp-dir"` / Cucumber `"requireModule"` / Stryker `"mutate"` / Detox `"apps"` / Codecept `"helpers": {` / WDIO `"waitforTimeout"` / TestCafe `"selectorTimeout"` / Karma `"frameworks"` / Nightwatch `"src_folders"` / Jasmine `"spec_dir"` / AVA `"failFast"` / Mocha `"allowUncaught"` / Jest `"testEnvironment"` / Playwright `"testDir"` / Vitest `"pool"` / Cypress `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. release-it `"plugins": {` (object); commitlint / eslint `"plugins": [` stay on those scanners. Shared `"ci"` / `"dryRun"` stay out of this scanner.

## [0.3.755] — 2026-11-10

View:

- **Function List** lists commitlint `.commitlintrc` / `.commitlintrc.json` / `package.json` `"commitlint"` JSON keys: `"parserPreset": "…"` / `{`, `"formatter": "…"`, `"defaultIgnores": true`, `"helpUrl": "…"`, `"prompt": {`, `"extends": "…"` / `[`, `"rules": {`, `"ignores": [`, `"plugins": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / biome `"formatter": {` / eslint `"parser"` / `"env"` / `"languageOptions"` / prettier `"printWidth"` / stylelint `"ignoreFiles"` / node-tap `"coverage-map"` / `"allow-empty-coverage"` / c8 `"src"` / nyc `"temp-dir"` / Cucumber `"requireModule"` / Stryker `"mutate"` / Detox `"apps"` / Codecept `"helpers": {` / WDIO `"waitforTimeout"` / TestCafe `"selectorTimeout"` / Karma `"frameworks"` / Nightwatch `"src_folders"` / Jasmine `"spec_dir"` / AVA `"failFast"` / Mocha `"allowUncaught"` / Jest `"testEnvironment"` / Playwright `"testDir"` / Vitest `"pool"` / Cypress `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. biome `"formatter": {` stays on biome; commitlint uses `"formatter": "…"`. Shared `"extends"` / `"rules"` / `"ignores"` / `"plugins"` also match neighbouring scanners when shapes overlap.

## [0.3.754] — 2026-11-10

View:

- **Function List** lists node-tap `package.json` `"tap"` / `.taprc` JSON keys: `"coverage-map": "…"`, `"coverage-report": [` / `"…"`, `"coverage-exclude": [` / `"…"`, `"allow-empty-coverage": true`, `"allow-incomplete-coverage": true`, `"disable-coverage": true`, `"show-full-coverage": true`, `"omit-version": true`, `"omit-whitespace": true`, `"fail-todo": true`, `"fail-skip": true`, `"fail-only": true`, `"reporter-file": "…"`, `"reporter-arg": [` / `"…"`, `"output-file": "…"`, `"output-dir": "…"`, `"test-arg": [` / `"…"`, `"test-env": [` / `"…"`, `"node-arg": [` / `"…"`, `"filter-quietly": true`, `"save-fixture": true`, `"snapshot": true`, `"snapshot-clean-cwd": true`, `"typecheck": true`, `"type-strip-only": true`, `"plugin": [` / `"…"`, `"files": [` / `"…"`, `"include": [` / `"…"`, `"exclude": [` / `"…"`, `"serial": [` / `"…"`, `"before": "…"`, `"after": "…"`, `"reporter": "…"`, `"extends": "…"`, `"tsconfig": "…"`, `"timeout": 60`, `"jobs": 4`, `"branches": 80`, `"functions": 80`, `"lines": 80`, `"statements": 80`, `"bail": true`, `"browser": true`, `"color": true`, `"comments": true`, `"passes": true`, `"diag": true`, `"shuffle": true`, `"only": true`, `"invert": true`, `"debug": true`, `"grep": [` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / AVA `"failFast"` / `"serial": true` / `"tap": true` / Mocha `"allowUncaught"` / `"bail": true` / c8 `"src"` / `"allowExternal"` / `"temp-directory"` / nyc `"temp-dir"` / `"watermarks"` / Cucumber `"requireModule"` / Stryker `"mutate"` / `"testRunner": "jest"` / Detox `"apps"` / Codecept `"helpers": {` / WDIO `"waitforTimeout"` / TestCafe `"selectorTimeout"` / Karma `"frameworks"` / Nightwatch `"src_folders"` / Jasmine `"spec_dir"` / Jest `"testEnvironment"` / Playwright `"testDir"` / Vitest `"pool"` / Cypress `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. AVA `"serial": true` stays on AVA; tap uses `"serial": [` / `"…"`. Shared `"include"` / `"exclude"` / `"files"` / `"reporter"` / `"timeout"` / `"jobs"` / `"bail"` / thresholds also match neighbouring scanners when shapes overlap.

## [0.3.753] — 2026-11-10

View:

- **Function List** lists c8 `.c8rc` / `.c8rc.json` / `c8.config` JSON keys: `"all": true`, `"check-coverage": true`, `"clean": true`, `"src": [` / `"…"`, `"include": [` / `"…"`, `"exclude": [` / `"…"`, `"reporter": [` / `"…"`, `"report-dir": "…"`, `"reports-dir": "…"`, `"temp-directory": "…"`, `"extension": [` / `"…"`, `"exclude-after-remap": true`, `"excludeNodeModules": true`, `"skip-full": true`, `"skip-empty": true`, `"allowExternal": true`, `"omit-relative": true`, `"per-file": true`, `"branches": 80`, `"functions": 80`, `"lines": 80`, `"statements": 80`, `"cwd": "…"`, `"wrapper": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / nyc `"temp-dir"` / `"watermarks"` / `"show-process-tree"` / `"instrumenter"` / `"complete-copy"` / `"produce-source-map"` / `"hook-run-in-context"` / `"hook-run-in-this-context"` / `"hook-require"` / `"babel-cache"` / `"use-spawn-wrap"` / `"eager"` / `"cache-dir"` / `"source-map"` / `"instrument"` / `"cache"` / `"exit-on-error"` / Cucumber `"requireModule"` / Stryker `"mutate"` / `"testRunner": "jest"` / Detox `"apps"` / Codecept `"helpers": {` / WDIO `"waitforTimeout"` / TestCafe `"selectorTimeout"` / Karma `"frameworks"` / Nightwatch `"src_folders"` / Jasmine `"spec_dir"` / AVA `"failFast"` / Mocha `"allowUncaught"` / Jest `"testEnvironment"` / Playwright `"testDir"` / Vitest `"pool"` / Cypress `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"include"` / `"exclude"` / `"reporter"` / `"all"` / `"check-coverage"` / `"report-dir"` / thresholds / `"cwd"` also match neighbouring scanners when shapes overlap.

## [0.3.752] — 2026-11-10

View:

- **Function List** lists Istanbul nyc `.nycrc` / `.nycrc.json` / `nyc.config` JSON keys: `"check-coverage": true`, `"report-dir": "…"`, `"temp-dir": "…"`, `"watermarks": {`, `"skip-full": true`, `"skip-empty": true`, `"show-process-tree": true`, `"exclude-after-remap": true`, `"instrumenter": "…"`, `"complete-copy": true`, `"produce-source-map": true`, `"hook-run-in-context": true`, `"hook-run-in-this-context": true`, `"hook-require": true`, `"branches": 80`, `"functions": 80`, `"lines": 80`, `"statements": 80`, `"babel-cache": true`, `"use-spawn-wrap": true`, `"excludeNodeModules": true`, `"exit-on-error": true`, `"omit-relative": true`, `"per-file": true`, `"eager": true`, `"cache-dir": "…"`, `"source-map": true`, `"instrument": true`, `"all": true`, `"include": [` / `"…"`, `"exclude": [` / `"…"`, `"reporter": [` / `"…"`, `"require": [` / `"…"`, `"extension": [` / `"…"`, `"cache": true`, `"clean": true`, `"cwd": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Cucumber `"requireModule"` / `"formatOptions"` / `"retryTagFilter"` / `"publishQuiet"` / `"worldParameters"` / `"dryRun"` / `"order": "defined"` / Stryker `"mutate"` / `"testRunner": "jest"` / `"coverageAnalysis"` / `"timeoutMS"` / Detox `"apps"` / `"devices"` / `"selectedConfiguration"` / Codecept `"helpers": {` / `"stepTimeout"` / WDIO `"waitforTimeout"` / TestCafe `"selectorTimeout"` / Karma `"frameworks"` / Nightwatch `"src_folders"` / Jasmine `"spec_dir"` / AVA `"failFast"` / Mocha `"allowUncaught"` / Jest `"testEnvironment"` / Playwright `"testDir"` / Vitest `"pool"` / Cypress `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"include"` / `"exclude"` / `"reporter"` / `"require"` / `"all"` / `"cache"` / `"cwd"` also match neighbouring scanners when shapes overlap.

## [0.3.751] — 2026-11-10

View:

- **Function List** lists Cucumber.js `.cucumberrc.json` / `cucumber.json` / `.cucumberrc` JSON keys: `"requireModule": [` / `"…"`, `"formatOptions": {`, `"retryTagFilter": "…"`, `"publishQuiet": true`, `"worldParameters": {`, `"dryRun": true`, `"backtrace": true`, `"paths": [` / `"…"`, `"format": [` / `"…"`, `"order": "defined"` / `"random"`, `"tags": "…"`, `"loader": [` / `"…"`, `"publish": true` / `{`, `"parallel": 2`, `"import": [` / `"…"`, `"require": [` / `"…"`, `"language": "…"`, `"retry": 1`, `"strict": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Stryker `"mutate"` / `"testRunner": "jest"` / `"coverageAnalysis"` / `"timeoutMS"` / `"timeoutFactor"` / `"dryRunTimeoutMinutes"` / `"maxConcurrentTestRunners"` / `"disableTypeChecks"` / `"ignoreStatic"` / `"appendPlugins"` / `"htmlReporter"` / `"commandRunner"` / Detox `"apps"` / `"devices"` / `"selectedConfiguration"` / `"behavior"` / `"artifacts"` / `"reversePorts"` / `"skipLegacyWorkersSynchronization"` / `"testRunner": {` / `"configurations": {` / Codecept `"helpers": {` / `"include": {` / `"stepTimeout"` / `"fullPromiseBased"` / `"gherkin"` / `"noGlobals"` / WDIO `"waitforTimeout"` / `"waitforInterval"` / `"connectionRetryTimeout"` / `"connectionRetryCount"` / `"maxInstances"` / `"mochaOpts"` / `"specFileRetries"` / `"automationProtocol"` / `"runner": "local"` / TestCafe `"selectorTimeout"` / `"assertionTimeout"` / `"quarantineMode"` / `"skipJsErrors"` / `"port1"` / `"port2"` / `"videoPath"` / `"disableNativeAutomation"` / Karma `"frameworks"` / `"preprocessors"` / `"autoWatch"` / `"singleRun"` / `"customLaunchers"` / `"browserNoActivityTimeout"` / `"captureTimeout"` / `"logLevel": "INFO"` / Nightwatch `"src_folders"` / `"page_objects_path"` / `"skip_testcases_on_fail"` / `"end_session_on_fail"` / `"test_settings"` / `"desiredCapabilities"` / Jasmine `"spec_dir"` / `"spec_files"` / `"jsLoader"` / `"stopSpecOnExpectationFailure"` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / `"parallel": true` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / `"forceExit"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / `"paths": {` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"require": [` / `"import": [` / `"language": "…"` / `"retry": 1` / `"parallel": 2` also match neighbouring scanners when shapes overlap. Knip `"paths": {` stays on Knip; Cucumber uses `"paths": [` / `"…"`. Mocha `"parallel": true` stays on Mocha; Cucumber uses a number.

## [0.3.750] — 2026-11-10

View:

- **Function List** lists StrykerJS `stryker.conf.json` / `.stryker.conf.json` / `stryker.config.json` JSON keys: `"mutate": [`, `"testRunner": "jest"` / `"mocha"` / `"karma"` / `"cucumber"` / `"command"` / `"vitest"` / `"jasmine"` / `"tap"`, `"coverageAnalysis": "perTest"` / `"all"` / `"off"`, `"timeoutMS": 60000`, `"timeoutFactor": 1.5`, `"dryRunTimeoutMinutes": 5`, `"maxConcurrentTestRunners": 4`, `"disableTypeChecks": true` / `"…"`, `"ignoreStatic": true`, `"inPlace": true`, `"incremental": true`, `"allowEmpty": true`, `"force": true`, `"cleanTempDir": true` / `"always"`, `"tempDirName": "…"`, `"incrementalFile": "…"`, `"fileLogLevel": "off"` / `"fatal"` / `"error"` / `"warn"` / `"info"` / `"debug"` / `"trace"`, `"ignorePatterns": [`, `"appendPlugins": [`, `"checkers": [`, `"testRunnerNodeArgs": [`, `"checkerNodeArgs": [`, `"htmlReporter": {`, `"clearTextReporter": {`, `"dashboard": {`, `"mochaOptions": {`, `"commandRunner": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Detox `"apps"` / `"devices"` / `"selectedConfiguration"` / `"behavior"` / `"artifacts"` / `"reversePorts"` / `"skipLegacyWorkersSynchronization"` / `"testRunner": {` / `"configurations": {` / Codecept `"helpers": {` / `"include": {` / `"stepTimeout"` / `"fullPromiseBased"` / `"gherkin"` / `"noGlobals"` / WDIO `"waitforTimeout"` / `"waitforInterval"` / `"connectionRetryTimeout"` / `"connectionRetryCount"` / `"maxInstances"` / `"mochaOpts"` / `"specFileRetries"` / `"automationProtocol"` / `"runner": "local"` / TestCafe `"selectorTimeout"` / `"assertionTimeout"` / `"quarantineMode"` / `"skipJsErrors"` / `"port1"` / `"port2"` / `"videoPath"` / `"disableNativeAutomation"` / Karma `"frameworks"` / `"preprocessors"` / `"autoWatch"` / `"singleRun"` / `"customLaunchers"` / `"browserNoActivityTimeout"` / `"captureTimeout"` / `"logLevel": "INFO"` / Nightwatch `"src_folders"` / `"page_objects_path"` / `"skip_testcases_on_fail"` / `"end_session_on_fail"` / `"test_settings"` / `"desiredCapabilities"` / Jasmine `"spec_dir"` / `"spec_files"` / `"jsLoader"` / `"stopSpecOnExpectationFailure"` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations": [` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` / shared `"reporters": [` / `"concurrency": 1` / `"plugins": [` / `"logLevel": "info"` stay out. Detox `"testRunner": {` stays on Detox; Stryker uses `"testRunner": "jest"`. package.json `"jest": {` / other shared runner objects stay out.

## [0.3.749] — 2026-11-10

View:

- **Function List** lists Detox `.detoxrc.json` / `.detoxrc` / `detox.config` JSON keys: `"apps": {`, `"devices": {`, `"configurations": {`, `"testRunner": {`, `"behavior": {`, `"logger": {`, `"artifacts": {`, `"session": {`, `"cli": {`, `"selectedConfiguration": "…"`, `"reversePorts": [`, `"skipLegacyWorkersSynchronization": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Codecept `"helpers": {` / `"include": {` / `"stepTimeout"` / `"fullPromiseBased"` / `"gherkin"` / `"noGlobals"` / WDIO `"waitforTimeout"` / `"waitforInterval"` / `"connectionRetryTimeout"` / `"connectionRetryCount"` / `"maxInstances"` / `"mochaOpts"` / `"specFileRetries"` / `"automationProtocol"` / `"runner": "local"` / TestCafe `"selectorTimeout"` / `"assertionTimeout"` / `"quarantineMode"` / `"skipJsErrors"` / `"port1"` / `"port2"` / `"videoPath"` / `"disableNativeAutomation"` / Karma `"frameworks"` / `"preprocessors"` / `"autoWatch"` / `"singleRun"` / `"customLaunchers"` / `"browserNoActivityTimeout"` / `"captureTimeout"` / `"logLevel": "INFO"` / Nightwatch `"src_folders"` / `"page_objects_path"` / `"skip_testcases_on_fail"` / `"end_session_on_fail"` / `"test_settings"` / `"desiredCapabilities"` / Jasmine `"spec_dir"` / `"spec_files"` / `"jsLoader"` / `"stopSpecOnExpectationFailure"` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations": [` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. VS Code launch `"configurations": [` stays on launch; Detox uses `"configurations": {`. Nightwatch `"test_runner"` stays on Nightwatch; Detox uses `"testRunner": {`.

## [0.3.748] — 2026-11-10

View:

- **Function List** lists CodeceptJS `codecept.conf.json` / `codecept.json` / `.codecept` JSON keys: `"tests": "…"` / `[`, `"output": "…"`, `"helpers": {`, `"include": {`, `"bootstrap": "…"`, `"teardown": "…"`, `"hooks": {`, `"gherkin": {`, `"plugins": {`, `"mocha": {`, `"multiple": {`, `"ai": {`, `"stepTimeout": 5000`, `"stepTimeoutOverride": [`, `"timeout": 10000`, `"retry": 2` / `{`, `"require": [` / `"…"`, `"grep": "…"`, `"translation": "…"`, `"verbose": true`, `"noGlobals": true`, `"fullPromiseBased": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / WDIO `"waitforTimeout"` / `"waitforInterval"` / `"connectionRetryTimeout"` / `"connectionRetryCount"` / `"maxInstances"` / `"mochaOpts"` / `"specFileRetries"` / `"automationProtocol"` / `"runner": "local"` / TestCafe `"selectorTimeout"` / `"assertionTimeout"` / `"quarantineMode"` / `"skipJsErrors"` / `"port1"` / `"port2"` / `"videoPath"` / `"disableNativeAutomation"` / Karma `"frameworks"` / `"preprocessors"` / `"autoWatch"` / `"singleRun"` / `"customLaunchers"` / `"browserNoActivityTimeout"` / `"captureTimeout"` / `"logLevel": "INFO"` / Nightwatch `"src_folders"` / `"page_objects_path"` / `"skip_testcases_on_fail"` / `"end_session_on_fail"` / `"test_settings"` / `"desiredCapabilities"` / Jasmine `"spec_dir"` / `"spec_files"` / `"jsLoader"` / `"stopSpecOnExpectationFailure"` / `"helpers": [` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"timeout": 10000` / `"require": [` / `"verbose": true` / `"plugins": {` also match neighbouring scanners when shapes overlap. Jasmine `"helpers": [` stays on Jasmine; Codecept uses `"helpers": {`.

## [0.3.747] — 2026-11-10

View:

- **Function List** lists WebdriverIO `wdio.conf.json` / `wdio.conf` / `.wdio` JSON keys: `"runner": "local"` / `"browser"`, `"specs": [` / `"…"`, `"exclude": [` / `"…"`, `"maxInstances": 5`, `"capabilities": [` / `{`, `"logLevel": "info"` / `"trace"` / `"debug"` / `"warn"` / `"error"` / `"silent"`, `"bail": 0`, `"baseUrl": "…"`, `"waitforTimeout": 10000`, `"waitforInterval": 500`, `"connectionRetryTimeout": 120000`, `"connectionRetryCount": 3`, `"services": [` / `"…"`, `"framework": "mocha"` / `"jasmine"` / `"cucumber"`, `"reporters": [` / `"…"`, `"mochaOpts": {`, `"jasmineOpts": {`, `"cucumberOpts": {`, `"specFileRetries": 1`, `"specFileRetriesDelay": 0`, `"specFileRetriesDeferred": true`, `"autoCompileOpts": {`, `"outputDir": "…"`, `"hostname": "…"`, `"port": 4444`, `"path": "/wd/hub"`, `"protocol": "http"` / `"https"`, `"injectGlobals": true`, `"tsConfigPath": "…"`, `"automationProtocol": "webdriver"` / `"devtools"` / `"puppeteer"`, `"headless": true`, `"suites": {`, `"execArgv": [`, `"region": "…"`, `"updateJob": true`, `"user": "…"`, `"key": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / TestCafe `"selectorTimeout"` / `"assertionTimeout"` / `"quarantineMode"` / `"skipJsErrors"` / `"port1"` / `"port2"` / `"videoPath"` / `"disableNativeAutomation"` / Karma `"frameworks"` / `"preprocessors"` / `"autoWatch"` / `"singleRun"` / `"customLaunchers"` / `"browserNoActivityTimeout"` / `"captureTimeout"` / `"logLevel": "INFO"` / Nightwatch `"src_folders"` / `"page_objects_path"` / `"skip_testcases_on_fail"` / `"end_session_on_fail"` / `"test_settings"` / `"desiredCapabilities"` / Jasmine `"spec_dir"` / `"spec_files"` / `"jsLoader"` / `"stopSpecOnExpectationFailure"` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"exclude": [` / `"hostname": "…"` / `"port": 4444` / `"reporters": [` / `"baseUrl": "…"` / `"tsConfigPath": "…"` also match neighbouring scanners when shapes overlap.


## [0.3.746] — 2026-11-10

View:

- **Function List** lists TestCafe `.testcaferc.json` / `.testcaferc` JSON keys: `"browsers": [` / `"…"`, `"src": [` / `"…"`, `"reporter": [` / `"…"` / `{`, `"concurrency": 1`, `"quarantineMode": true` / `{`, `"selectorTimeout": 10000`, `"assertionTimeout": 3000`, `"pageLoadTimeout": 30000`, `"ajaxRequestTimeout": 120000`, `"browserInitTimeout": 120000`, `"testExecutionTimeout": 0`, `"runExecutionTimeout": 0`, `"skipJsErrors": true`, `"skipUncaughtErrors": true`, `"stopOnFirstFail": true`, `"disablePageCaching": true`, `"disablePageReloads": true`, `"disableNativeAutomation": true`, `"developmentMode": true`, `"debugMode": true`, `"debugOnFail": true`, `"color": true`, `"disableMultipleWindows": true`, `"retryTestPages": true`, `"cache": true`, `"screenshots": {`, `"videoPath": "…"`, `"videoOptions": {`, `"videoEncodingOptions": {`, `"hostname": "…"`, `"port1": 1337`, `"port2": 1338`, `"proxy": "…"`, `"proxyBypass": [` / `"…"`, `"ssl": {`, `"compilerOptions": {`, `"filter": {`, `"hooks": {`, `"clientScripts": [`, `"tsConfigPath": "…"`, `"userVariables": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Karma `"frameworks"` / `"preprocessors"` / `"autoWatch"` / `"singleRun"` / `"customLaunchers"` / `"browserNoActivityTimeout"` / `"captureTimeout"` / `"logLevel": "INFO"` / Nightwatch `"src_folders"` / `"page_objects_path"` / `"skip_testcases_on_fail"` / `"end_session_on_fail"` / `"test_settings"` / `"desiredCapabilities"` / Jasmine `"spec_dir"` / `"spec_files"` / `"jsLoader"` / `"stopSpecOnExpectationFailure"` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"baseUrl": "http://…"` / `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"browsers": [` / `"concurrency": 1` / `"hostname": "…"` / `"pageLoadTimeout": 30000` / `"screenshots": {` / `"reporter": "…"` / `"compilerOptions": {` also match neighbouring scanners when shapes overlap.

## [0.3.745] — 2026-11-10

View:

- **Function List** lists Karma `karma.conf.json` / `karma.conf` JSON keys: `"basePath": "…"`, `"frameworks": [` / `"…"`, `"files": [` / `"…"`, `"exclude": [` / `"…"`, `"preprocessors": {`, `"reporters": [` / `"…"`, `"plugins": [` / `"…"`, `"browsers": [` / `"…"`, `"customLaunchers": {`, `"client": {`, `"proxies": {`, `"middleware": [` / `"…"`, `"beforeMiddleware": [` / `"…"`, `"browserConsoleLogOptions": {`, `"httpsServerOptions": {`, `"upstreamProxy": {`, `"port": 9876`, `"concurrency": 1`, `"browserNoActivityTimeout": 30000`, `"browserDisconnectTimeout": 2000`, `"browserDisconnectTolerance": 0`, `"browserSocketTimeout": 20000`, `"captureTimeout": 60000`, `"reportSlowerThan": 500`, `"pingTimeout": 5000`, `"processKillTimeout": 2000`, `"colors": true`, `"autoWatch": true`, `"singleRun": true`, `"restartOnFileChange": true`, `"failOnEmptyTestSuite": true`, `"failOnFailingTestSuite": true`, `"crossOriginAttribute": true`, `"forceJSONP": true`, `"proxyValidateSSL": true`, `"browserClearCache": true`, `"logLevel": "INFO"` / `"DEBUG"` / `"WARN"` / `"ERROR"` / `"DISABLE"` / `"OFF"`, `"hostname": "…"`, `"listenAddress": "…"`, `"protocol": "http"` / `"https"`, `"urlRoot": "…"`, `"formatError": "…"`, `"transports": [` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Nightwatch `"src_folders"` / `"page_objects_path"` / `"skip_testcases_on_fail"` / `"end_session_on_fail"` / `"test_settings"` / `"desiredCapabilities"` / Jasmine `"spec_dir"` / `"spec_files"` / `"jsLoader"` / `"stopSpecOnExpectationFailure"` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"baseUrl": "http://…"` / `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"files": [` / `"exclude": [` / `"port": 9876` / `"concurrency": 1` / `"reporters": [` / `"plugins": [` also match neighbouring scanners when shapes overlap.

## [0.3.744] — 2026-11-10

View:

- **Function List** lists Nightwatch `nightwatch.json` / `nightwatch.conf.json` JSON keys: `"src_folders": [` / `"…"`, `"output_folder": "…"`, `"custom_commands_path": [` / `"…"`, `"custom_assertions_path": [` / `"…"`, `"page_objects_path": [` / `"…"`, `"globals_path": "…"`, `"globals": {`, `"launch_url": "…"`, `"webdriver": {`, `"selenium": {`, `"test_settings": {`, `"desiredCapabilities": {`, `"screenshots": {`, `"test_workers": true` / `{`, `"skip_testcases_on_fail": true`, `"end_session_on_fail": true`, `"unit_tests_mode": true`, `"live_output": true`, `"disable_colors": true`, `"persist_globals": true`, `"start_session": true`, `"silent": true`, `"output": true`, `"detailed_output": true`, `"parallel_mode": true`, `"enable_fail_fast": true`, `"always_async_commands": true`, `"use_xpath": true`, `"parallel_process_delay": 10`, `"filter": "…"`, `"exclude": [` / `"…"`, `"skipgroup": "…"`, `"skiptags": "…"`, `"tag_filter": "…"`, `"test_runner": "default"` / `"mocha"` / `"cucumber"` / `{`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Jasmine `"spec_dir"` / `"spec_files"` / `"jsLoader"` / `"stopSpecOnExpectationFailure"` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"baseUrl": "http://…"` / `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"exclude": [` / `"globals": {` / `"output": true` / `"silent": true` also match neighbouring scanners when shapes overlap.

## [0.3.743] — 2026-11-10

View:

- **Function List** lists Jasmine `jasmine.json` / `spec/support/jasmine.json` JSON keys: `"spec_dir": "spec"`, `"spec_files": [` / `"…"`, `"helpers": [` / `"…"`, `"requires": [` / `"…"`, `"jsLoader": "require"` / `"import"`, `"cssFiles": [` / `"…"`, `"jsFiles": [` / `"…"`, `"env": {`, `"random": true`, `"seed": 1234` / `"…"`, `"stopOnSpecFailure": true`, `"stopSpecOnExpectationFailure": true`, `"failSpecWithNoExpectations": true`, `"forbidDuplicateNames": true`, `"hideDisabled": true`, `"verboseDeprecations": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / AVA `"failFast"` / `"failWithoutAssertions"` / `"utilizeParallelBuilds"` / `"ignoredByWatcher"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"baseUrl": "http://…"` / `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"helpers": [` / `"requires": [` / `"random": true` / `"env": {` also match neighbouring scanners when shapes overlap.

## [0.3.742] — 2026-11-10

View:

- **Function List** lists AVA `ava.config.json` / `package.json` `"ava"` JSON keys: `"files": [` / `"…"`, `"match": [` / `"…"`, `"concurrency": 5`, `"failFast": true`, `"failWithoutAssertions": true`, `"cache": true`, `"workerThreads": true`, `"serial": true`, `"tap": true`, `"verbose": true`, `"utilizeParallelBuilds": true`, `"timeout": 100` / `"10s"` / `"2m"` / `"500ms"`, `"require": [` / `"…"`, `"nodeArguments": [` / `"…"`, `"extensions": [` / `{` / `"…"`, `"ignoredByWatcher": [` / `"…"`, `"snapshotDir": "…"`, `"environmentVariables": {`, `"watchMode": {`, `"typescript": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Mocha `"allowUncaught"` / `"ui": "bdd"` / `"watch-files"` / `"checkLeaks"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"baseUrl": "http://…"` / `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"require": …` / `"timeout": …` / `"files": [` / `"workerThreads": true` also match Mocha / Jest when shapes overlap.

## [0.3.741] — 2026-11-10

View:

- **Function List** lists Mocha `.mocharc.json` / `.mocharc` JSON keys: `"allowUncaught": true`, `"asyncOnly": true`, `"bail": true`, `"checkLeaks": true`, `"color": true`, `"colours": true`, `"delay": true`, `"diff": true`, `"exit": true`, `"failZero": true`, `"forbidOnly": true`, `"forbidPending": true`, `"fullTrace": true`, `"growl": true`, `"inlineDiffs": true`, `"invert": true`, `"parallel": true`, `"recursive": true`, `"sort": true`, `"watch": true`, `"jobs": 2`, `"retries": 2`, `"slow": 75` / `"75"`, `"timeout": 2000` / `"2s"` / `"500ms"`, `"fgrep": "…"`, `"grep": "…"` / `[`, `"package": "./package.json"`, `"reporter": "spec"`, `"ui": "bdd"` / `"tdd"` / `"qunit"` / `"exports"`, `"config": "…"`, `"opts": "…"`, `"extension": "js"` / `[`, `"file": "…"` / `[`, `"ignore": "…"` / `[`, `"exclude": "…"` / `[`, `"require": "…"` / `[`, `"spec": "…"` / `[`, `"watch-files": "…"` / `[`, `"watch-ignore": "…"` / `[`, `"node-option": "…"` / `[`, `"globals": "…"` / `[`, `"reporter-option": "…"` / `[`, `"reporterOption": "…"` / `[`, `"reporterOptions": {` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / Cypress `"baseUrl": "http://…"` / `"specPattern"` / `"chromeWebSecurity"` / `"e2e"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"reporter": "…"` / `"retries": 2` / `"forbidOnly": true` / `"timeout": 2000` also match Playwright / Cypress / Jest when shapes overlap.

## [0.3.740] — 2026-11-10

View:

- **Function List** lists Cypress `cypress.json` / `cypress.config` JSON keys: `"baseUrl": "http://…"` / `"https://…"`, `"specPattern": "…"` / `[`, `"excludeSpecPattern": "…"` / `[`, `"supportFile": "…"` / false, `"fixturesFolder": "…"` / false, `"screenshotsFolder": "…"`, `"videosFolder": "…"`, `"downloadsFolder": "…"`, `"fileServerFolder": "…"`, `"indexHtmlFile": "…"`, `"devServerPublicPathRoute": "…"`, `"projectId": "…"`, `"reporter": "…"`, `"userAgent": "…"`, `"defaultBrowser": "…"`, `"video": true`, `"screenshotOnRunFailure": true`, `"trashAssetsBeforeRuns": true`, `"chromeWebSecurity": true`, `"includeShadowDom": true`, `"watchForFileChanges": true`, `"modifyObstructiveCode": true`, `"waitForAnimations": true`, `"testIsolation": true`, `"justInTimeCompile": true`, `"experimentalSingleTabRunMode": true`, `"forceHttp1": true`, `"manageBrowserMemory": true`, `"removeSRIAttributes": true`, `"injectDocumentDomain": true`, `"videoCompression": true` / 32, `"retries": 2` / `{`, `"viewportWidth": 1000`, `"viewportHeight": 660`, `"defaultCommandTimeout": 4000`, `"taskTimeout": 60000`, `"pageLoadTimeout": 60000`, `"requestTimeout": 5000`, `"responseTimeout": 30000`, `"numTestsKeptInMemory": 50`, `"redirectionLimit": 20`, `"port": 3000`, `"animationDistanceThreshold": 5`, `"keystrokeDelay": 10`, `"slowTestThreshold": 10000`, `"scrollBehavior": "top"` / `"bottom"` / `"center"` / `"nearest"` / false / `{`, `"visibilityStrategy": "modern"` / `"legacy"`, `"e2e": {`, `"component": {`, `"env": {`, `"expose": {`, `"hosts": {`, `"reporterOptions": {`, `"devServer": {`, `"clientCertificates": [`, `"blockHosts": "…"` / `[`, `"trustedCertificates": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"forbidOnly"` / `"webServer"` / `"preserveOutput"` / Vitest `"pool"` / `"fileParallelism"` / `"hookTimeout"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` / `"baseUrl": "."` stay out. Shared `"reporter": "…"` / `"retries": 2` also match Playwright when shapes overlap.

## [0.3.739] — 2026-11-09

View:

- **Function List** lists Vitest `vitest.config.json` / `vitest.config` JSON keys: `"environment": "node"` / `"jsdom"` / `"happy-dom"` / `"edge-runtime"` / `"…"`, `"pool": "threads"` / `"forks"` / `"vmThreads"` / `"vmForks"`, `"fileParallelism": true`, `"isolate": true`, `"allowOnly": true`, `"dangerouslyIgnoreUnhandledErrors": true`, `"passWithNoTests": true`, `"clearMocks": true`, `"mockReset": true`, `"restoreMocks": true`, `"unstubEnvs": true`, `"unstubGlobals": true`, `"globals": true`, `"ui": true`, `"open": true`, `"hideSkippedTests": true`, `"logHeapUsage": true`, `"printConsoleTrace": true`, `"update": true`, `"watch": true`, `"include": [`, `"exclude": [`, `"includeSource": [`, `"setupFiles": [`, `"globalSetup": "…"` / `[`, `"watchExclude": [`, `"forceRerunTriggers": [`, `"reporters": [`, `"projects": [`, `"workspace": [`, `"environmentMatchGlobs": [`, `"environmentOptions": {`, `"poolOptions": {`, `"sequence": {`, `"typecheck": {`, `"benchmark": {`, `"browser": {`, `"coverage": {`, `"css": {` / true, `"deps": {`, `"server": {`, `"fakeTimers": {`, `"expect": {`, `"chaiConfig": {`, `"snapshotFormat": {`, `"outputFile": {` / `"…"`, `"api": {` / true / 51204, `"uiBase": "/"`, `"root": "."`, `"dir": "…"`, `"cacheDir": "…"`, `"mode": "test"` / `"benchmark"`, `"testNamePattern": "…"`, `"testTimeout": 5000`, `"hookTimeout": 10000`, `"teardownTimeout": 10000`, `"maxWorkers": 4` / `"50%"`, `"minWorkers": 1`, `"maxConcurrency": 5`, `"retry": 1`, `"bail": 1`, `"slowTestThreshold": 300`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / Playwright `"testDir"` / `"fullyParallel"` / `"forbidOnly"` / `"webServer"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"include": [` / `"exclude": [` / `"setupFiles": [` / `"projects": [` / `"clearMocks": true` / `"passWithNoTests": true` also match Jest / Knip when shapes overlap.

## [0.3.738] — 2026-11-09

View:

- **Function List** lists Playwright `playwright.config.json` / `playwright.config` JSON keys: `"testDir": "…"`, `"testMatch": "…"` / `[`, `"testIgnore": "…"` / `[`, `"outputDir": "…"`, `"fullyParallel": true`, `"forbidOnly": true`, `"globalSetup": "…"`, `"globalTeardown": "…"`, `"globalTimeout": 60000`, `"grep": "…"` / `[`, `"grepInvert": "…"` / `[`, `"maxFailures": 1`, `"preserveOutput": "always"` / `"never"` / `"failures-only"`, `"reporter": "…"` / `[`, `"reportSlowTests": {`, `"quiet": true`, `"retries": 2`, `"workers": 4` / `"50%"`, `"shard": {`, `"timeout": 30000`, `"updateSnapshots": "all"` / `"changed"` / `"missing"` / `"none"`, `"ignoreSnapshots": true`, `"use": {`, `"projects": [`, `"webServer": {` / `[`, `"expect": {`, `"snapshotPathTemplate": "…"`, `"snapshotDir": "…"`, `"metadata": {`, `"captureGitInfo": {`, `"tsconfig": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Jest `"testEnvironment"` / `"setupFilesAfterEnv"` / `"coverageProvider"` / `"clearMocks"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"projects": [` / `"globalSetup": "…"` / `"testMatch": [` also match Jest when shapes overlap.

## [0.3.737] — 2026-11-09

View:

- **Function List** lists Jest `jest.config.json` / `jest.config.jsonc` JSON keys: `"testEnvironment": "node"` / `"jsdom"` / `"…"`, `"testMatch": [`, `"testRegex": "…"` / `[`, `"testPathIgnorePatterns": [`, `"collectCoverageFrom": [`, `"coverageDirectory": "…"`, `"coveragePathIgnorePatterns": [`, `"coverageReporters": [`, `"coverageThreshold": {`, `"coverageProvider": "babel"` / `"v8"`, `"setupFiles": [`, `"setupFilesAfterEnv": [`, `"globalSetup": "…"`, `"globalTeardown": "…"`, `"moduleNameMapper": {`, `"moduleFileExtensions": [`, `"moduleDirectories": [`, `"modulePaths": [`, `"transform": {`, `"transformIgnorePatterns": [`, `"unmockedModulePathPatterns": [`, `"roots": [`, `"rootDir": "…"`, `"preset": "…"`, `"projects": [`, `"reporters": [`, `"snapshotSerializers": [`, `"snapshotFormat": {`, `"watchPathIgnorePatterns": [`, `"watchPlugins": [`, `"clearMocks": true`, `"resetMocks": true`, `"restoreMocks": true`, `"resetModules": true`, `"automock": true`, `"fakeTimers": {`, `"testTimeout": 5000`, `"maxWorkers": 4` / `"50%"`, `"maxConcurrency": 5`, `"detectOpenHandles": true`, `"forceExit": true`, `"errorOnDeprecated": true`, `"injectGlobals": true`, `"passWithNoTests": true`, `"testLocationInResults": true`, `"workerThreads": true`, `"slowTestThreshold": 5`, `"notifyMode": "failure"`, `"prettierPath": "…"`, `"testRunner": "…"`, `"testSequencer": "…"`, `"workerIdleMemoryLimit": "…"`, `"extensionsToTreatAsEsm": [`, `"haste": {`, `"sandboxInjectedGlobals": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"displayName"` / `"verbose"` / `"silent"` / `"bail"` / `"cache"` / `"globals"` / CSpell `"words"` / Oxlint `"categories"` / Knip `"entry"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"roots": [` / `"transform": {` / `"preset": "…"` also match neighbouring scanners when shapes overlap.

## [0.3.736] — 2026-11-09

View:

- **Function List** lists CSpell `cspell.json` / `.cspell.json` / `cspell.config.json` JSON keys: `"words": [`, `"ignoreWords": [`, `"flagWords": [`, `"suggestWords": [`, `"noSuggestWords": [`, `"userWords": [`, `"ignorePaths": [`, `"ignoreRegExpList": [`, `"includeRegExpList": [`, `"patterns": [`, `"dictionaries": [`, `"dictionaryDefinitions": [`, `"languageSettings": [`, `"overrides": [`, `"enableFiletypes": [`, `"enabledLanguageIds": [`, `"import": [` / `"…"`, `"language": "en"` / `"en-GB"` / `"en,fr"`, `"allowCompoundWords": true`, `"caseSensitive": true`, `"useGitignore": true`, `"enableGlobDot": true`, `"loadDefaultConfiguration": true`, `"noConfigSearch": true`, `"validateDirectives": true`, `"minWordLength": 4`, `"maxNumberOfProblems": 100`, `"numSuggestions": 8`, `"suggestionsTimeout": 500`, `"spellCheckDelayMs": 50`, `"gitignoreRoot": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"enabled"` / `"description"` / `"files"` / `"id"` / Oxlint `"categories"` / `"jsPlugins"` / Knip `"entry"` / `"ignoreBinaries"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / eslint `"parser"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"overrides": [` / `"ignorePaths": [` also match eslint / prettier / renovate when shapes match.


## [0.3.735] — 2026-11-09

View:

- **Function List** lists Oxlint `.oxlintrc.json` / `oxlintrc.json` JSON keys: `"categories": {`, `"jsPlugins": [`, `"options": {`, `"plugins": [`, `"rules": {`, `"overrides": [`, `"env": {`, `"globals": {`, `"settings": {`, `"extends": [`, `"ignorePatterns": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"root"` / ESLint `"parser"` / `"parserOptions"` / `"languageOptions"` / `"linterOptions"` / `"noInlineConfig"` / `"reportUnusedDisableDirectives"` / `"ignores"` / `"processor"` / `"extends": "…"` (string; oxlint JSON is array-only) / Knip `"entry"` / `"ignoreBinaries"` / Deno `"nodeModulesDir"` / Renovate `"packageRules"` / launch `"configurations"` / tasks `"tasks": [` / Dev Container `"image"` / turbo `"pipeline"` / nx `"namedInputs"` / prettier `"printWidth"` / biome `"linter"` / package.json `"scripts"` / tsconfig `"compilerOptions"` stay out. Shared `"plugins": [` / `"rules": {` / `"overrides": [` / `"env": {` / `"globals": {` / `"settings": {` / `"extends": [` / `"ignorePatterns": [` also match eslint / stylelint / prettier / biome when shapes match.

## [0.3.734] — 2026-11-09

View:

- **Function List** lists Knip `knip.json` / `knip.jsonc` JSON keys: `"entry": [`, `"project": [`, `"paths": {`, `"workspaces": {`, `"rules": {`, `"include": [`, `"exclude": [`, `"tags": [`, `"preprocessor": "…"` / `[`, `"preprocessorOptions": {`, `"ignore": [`, `"ignoreFiles": [`, `"ignoreBinaries": [`, `"ignoreDependencies": [`, `"ignoreMembers": [`, `"ignoreUnresolved": [`, `"ignoreWorkspaces": [`, `"ignoreIssues": {`, `"cycles": {`, `"ignoreExportsUsedInFile": true` / `{`, `"ignoreGlobalBinaries": true`, `"includeEntryExports": true`, `"includeSubpathExports": true`, `"treatConfigHintsAsErrors": true`, `"treatTagHintsAsErrors": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / Deno `"nodeModulesDir"` / `"vendor"` / `"unstable"` / `"importMap"` / Renovate `"packageRules"` / `"rangeStrategy"` / `"dependencyDashboard"` / launch `"configurations"` / tasks `"tasks": [` / `"isBackground"` / Dev Container `"image"` / `"forwardPorts"` / turbo `"pipeline"` / `"globalDependencies"` / nx `"namedInputs"` / eslint `"root"` / prettier `"printWidth"` / biome `"linter"` / package.json `"scripts"` / tsconfig `"compilerOptions"` / webextension `"manifest_version"` stay out. Shared `"paths": {` / `"rules": {` / `"include": [` / `"exclude": [` / `"workspaces": {` also match tsconfig / eslint / package.json when shapes match.

## [0.3.733] — 2026-11-09

View:

- **Function List** lists Deno `deno.json` / `deno.jsonc` JSON keys: `"tasks": {`, `"imports": {`, `"scopes": {`, `"lint": {`, `"fmt": {`, `"test": {`, `"bench": {`, `"compilerOptions": {`, `"permissions": {`, `"publish": {`, `"exports": {` / `"…"`, `"nodeModulesDir": "auto"` / `"manual"` / `"none"` / true / false, `"vendor": true`, `"lock": true` / `"…"`, `"unstable": [`, `"exclude": [`, `"workspace": [`, `"importMap": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"description"` / Renovate `"packageRules"` / `"rangeStrategy"` / `"dependencyDashboard"` / `"enabledManagers"` / `"rebaseWhen"` / `"lockFileMaintenance"` / `"vulnerabilityAlerts"` / launch `"configurations"` / `"compounds"` / `"preLaunchTask"` / tasks `"tasks": [` / `"inputs": [` / `"dependsOn"` / `"problemMatcher"` / `"isBackground"` / `"version": "2.0.0"` / `"version": "0.2.0"` / Dev Container `"image"` / `"forwardPorts"` / `"customizations"` / `"postCreateCommand"` / `"remoteUser"` / `"shutdownAction"` / webextension `"manifest_version"` / `"background"` / `"action"` / `"content_scripts"` / forge `"packagerConfig"` / `"makers"` / neutralino `"applicationId"` / `"defaultMode"` / electron-builder `"directories"` / `"asar"` / `"nsis": {` / wails `"assetdir"` / `"frontend:dir"` / nest-cli `"collection"` / `"entryFile"` / angular `"architect"` / `"schematics"` / `"projectType"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"documentRoot"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / package.json `"scripts": {` stay out. Shared `"tasks": {` / `"imports": {` / `"exports": {` / `"…"` also match package.json / turbo when shapes match; shared `"compilerOptions": {` / `"exclude": [` also match tsconfig; shared `"permissions": {` stays distinct from webextension `"permissions": [`.

## [0.3.732] — 2026-11-09

View:

- **Function List** lists Renovate `renovate.json` / `renovate.json5` / `.renovaterc.json` JSON keys: `"extends": [` / `"…"`, `"packageRules": [`, `"schedule": [`, `"timezone": "…"`, `"automerge": true`, `"rangeStrategy": "auto"` / `"bump"` / `"widen"` / `"replace"` / `"update-lockfile"` / `"in-range-only"` / `"pin"` / `"pinDigest"`, `"vulnerabilityAlerts": {`, `"lockFileMaintenance": {`, `"dependencyDashboard": true`, `"dependencyDashboardTitle": "…"`, `"enabledManagers": [`, `"ignoreDeps": [`, `"ignorePaths": [`, `"includePaths": [`, `"postUpdateOptions": [`, `"rebaseWhen": "auto"` / `"never"` / `"conflicted"` / `"behind-base-branch"`, `"baseBranches": [`, `"assignees": [`, `"reviewers": [`, `"labels": [`, `"onboarding": true`, `"onboardingConfig": {`, `"platform": "github"` / `"gitlab"` / `"bitbucket"` / `"azure"` / `"gitea"` / `"forgejo"` / `"bitbucket-server"`, `"autodiscover": true`, `"separateMajorMinor": true`, `"separateMinorPatch": true`, `"branchPrefix": "…"`, `"semanticCommits": "enabled"` / `"disabled"` / `"auto"`, `"hostRules": [`, `"matchUpdateTypes": [`, `"matchPackageNames": [`, `"matchManagers": [`, `"matchFileNames": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"description"` / `"version"` / `"enabled"` / launch `"configurations"` / `"compounds"` / `"preLaunchTask"` / tasks `"tasks": [` / `"inputs": [` / `"dependsOn"` / `"problemMatcher"` / `"isBackground"` / `"version": "2.0.0"` / `"version": "0.2.0"` / Dev Container `"image"` / `"forwardPorts"` / `"customizations"` / `"postCreateCommand"` / `"remoteUser"` / `"shutdownAction"` / webextension `"manifest_version"` / `"background"` / `"action"` / `"content_scripts"` / forge `"packagerConfig"` / `"makers"` / neutralino `"applicationId"` / `"defaultMode"` / electron-builder `"directories"` / `"asar"` / `"nsis": {` / wails `"assetdir"` / `"frontend:dir"` / nest-cli `"collection"` / `"entryFile"` / angular `"architect"` / `"schematics"` / `"projectType"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"documentRoot"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks": {` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts": {` stay out. Shared `"extends": [` / `"extends": "…"` also match eslint / stylelint / babel / parcel / turbo / nx / tsconfig when shapes match.

## [0.3.731] — 2026-11-09

View:

- **Function List** lists VS Code / Dev Containers `.devcontainer/devcontainer.json` JSON keys: `"image": "…"`, `"dockerFile": "…"` / `"dockerfile": "…"`, `"context": "…"`, `"build": {`, `"features": {`, `"customizations": {`, `"forwardPorts": [`, `"portsAttributes": {`, `"otherPortsAttributes": {`, `"postCreateCommand": "…" / `[`, `"postStartCommand": "…" / `[`, `"postAttachCommand": "…" / `[`, `"updateContentCommand": "…" / `[`, `"initializeCommand": "…" / `[`, `"onCreateCommand": "…" / `[`, `"remoteUser": "…"`, `"containerUser": "…"`, `"workspaceFolder": "…"`, `"workspaceMount": "…"`, `"mounts": [`, `"runArgs": [`, `"containerEnv": {`, `"remoteEnv": {`, `"overrideCommand": true`, `"shutdownAction": "none"` / `"stopContainer"` / `"stopCompose"`, `"hostRequirements": {`, `"waitFor": "…"`, `"privileged": true`, `"capAdd": [`, `"securityOpt": [`, `"service": "…"`, `"dockerComposeFile": "…" / `[`, `"runServices": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"id"` / `"version"` / `"appPort"` / launch `"configurations"` / `"compounds"` / `"preLaunchTask"` / tasks `"tasks": [` / `"inputs": [` / `"dependsOn"` / `"problemMatcher"` / `"isBackground"` / webextension `"manifest_version"` / `"background"` / `"action"` / `"content_scripts"` / forge `"packagerConfig"` / `"makers"` / neutralino `"applicationId"` / `"defaultMode"` / electron-builder `"directories"` / `"asar"` / `"nsis": {` / wails `"assetdir"` / `"frontend:dir"` / nest-cli `"collection"` / `"entryFile"` / angular `"architect"` / `"schematics"` / `"projectType"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"documentRoot"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks": {` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts": {` stay out.

## [0.3.730] — 2026-11-09

View:

- **Function List** lists VS Code / Cursor `.vscode/tasks.json` JSON keys: `"version": "2.0.0"`, `"tasks": [`, `"inputs": [`, `"dependsOn": [` / `"…"`, `"dependsOrder": "sequence"` / `"parallel"`, `"problemMatcher": [` / `"$tsc"` / `"…"`, `"group": {` / `"build"` / `"test"` / `"none"`, `"presentation": {`, `"runOptions": {`, `"detail": "…"`, `"isBackground": true`, `"promptOnClose": true`, `"hide": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"type"` / `"label"` / `"command"` / `"args"` / `"cwd"` / `"env"` / `"options": {` / free-form `"version": "1.0.0"` / launch `"version": "0.2.0"` / `"configurations"` / `"compounds"` / `"preLaunchTask"` / `"postDebugTask"` / `"request"` / `"console"` / `"MIMode"` / turbo `"tasks": {` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / webextension `"manifest_version"` / `"background"` / `"action"` / `"content_scripts"` / forge `"packagerConfig"` / `"makers"` / neutralino `"applicationId"` / `"defaultMode"` / electron-builder `"directories"` / `"asar"` / `"nsis": {` / wails `"assetdir"` / `"frontend:dir"` / nest-cli `"collection"` / `"entryFile"` / angular `"architect"` / `"schematics"` / `"projectType"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"documentRoot"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts": {` stay out. Shared `"presentation": {` also matches launch.json.

## [0.3.729] — 2026-11-09

View:

- **Function List** lists VS Code / Cursor `.vscode/launch.json` JSON keys: `"version": "0.2.0"`, `"configurations": [`, `"compounds": [`, `"preLaunchTask": "…"`, `"postDebugTask": "…"`, `"request": "launch"` / `"attach"`, `"console": "internalConsole"` / `"integratedTerminal"` / `"externalTerminal"`, `"internalConsoleOptions": "neverOpen"` / `"openOnSessionStart"` / `"openOnFirstSessionStart"`, `"consoleTitle": "…"`, `"serverReadyAction": {`, `"pipeTransport": {`, `"presentation": {`, `"runtimeArgs": [`, `"runtimeExecutable": "…"`, `"miDebuggerPath": "…"`, `"MIMode": "gdb"` / `"lldb"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"type"` / `"program"` / `"cwd"` / `"env"` / `"args"` / `"port"` / `"outDir"` / free-form `"version": "1.0.0"` / angular `"configurations": {` / `"architect"` / `"schematics"` / `"projectType"` / webextension `"manifest_version"` / `"background"` / `"action"` / `"browser_action"` / `"content_scripts"` / `"permissions"` / forge `"packagerConfig"` / `"rebuildConfig"` / `"makers"` / `"publishers"` / `"buildIdentifier"` / neutralino `"applicationId"` / `"defaultMode"` / `"enableServer"` / `"enableNativeAPI"` / `"tokenSecurity"` / `"nativeAllowList"` / `"modes"` / `"globalVariables"` / electron-builder `"directories"` / `"asar"` / `"electronVersion"` / `"artifactName"` / `"nsis": {` / wails `"assetdir"` / `"frontend:dir"` / `"wailsjsdir"` / `"nsisType"` / `"debounceMS"` / `"obfuscated"` / `"bindings"` / nest-cli `"collection"` / `"entryFile"` / `"generateOptions"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"documentRoot"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / `"outDir"` / package.json `"scripts": {` stay out.

## [0.3.728] — 2026-11-09

View:

- **Function List** lists browser / WebExtension `manifest.json` JSON keys (Chrome MV2/MV3, Firefox, Safari): `"manifest_version": 2` / `3`, `"background": {`, `"action": {`, `"browser_action": {`, `"page_action": {`, `"sidebar_action": {`, `"chrome_url_overrides": {`, `"chrome_settings_overrides": {`, `"commands": {`, `"content_security_policy": {` / `"…"`, `"declarative_net_request": {`, `"externally_connectable": {`, `"icons": {`, `"omnibox": {`, `"options_ui": {`, `"storage": {`, `"browser_specific_settings": {`, `"theme": {`, `"user_scripts": {`, `"content_scripts": [`, `"permissions": [`, `"host_permissions": [`, `"optional_permissions": [`, `"optional_host_permissions": [`, `"web_accessible_resources": [`, `"protocol_handlers": [`, `"devtools_page": "…"`, `"options_page": "…"`, `"homepage_url": "…"`, `"update_url": "…"`, `"default_locale": "…"`, `"minimum_chrome_version": "…"`, `"short_name": "…"`, `"incognito": "spanning"` / `"split"` / `"not_allowed"`, `"offline_enabled": true`, `"converted_from_user_script": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"description"` / `"author"` / forge `"packagerConfig"` / `"rebuildConfig"` / `"makers"` / `"publishers"` / `"buildIdentifier"` / neutralino `"applicationId"` / `"defaultMode"` / `"enableServer"` / `"enableNativeAPI"` / `"tokenSecurity"` / `"nativeAllowList"` / `"modes"` / `"globalVariables"` / electron-builder `"directories"` / `"asar"` / `"electronVersion"` / `"artifactName"` / `"nsis": {` / wails `"assetdir"` / `"frontend:dir"` / `"wailsjsdir"` / `"nsisType"` / `"debounceMS"` / `"obfuscated"` / `"bindings"` / nest-cli `"collection"` / `"entryFile"` / `"generateOptions"` / angular `"architect"` / `"schematics"` / `"projectType"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"documentRoot"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / `"outDir"` / package.json `"scripts": {` stay out.

## [0.3.727] — 2026-11-09

View:

- **Function List** lists Electron Forge `forge.config.json` / `config.forge` JSON keys: `"packagerConfig": {`, `"rebuildConfig": {`, `"hooks": {`, `"makers": [`, `"publishers": [`, `"plugins": [`, `"buildIdentifier": "…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"outDir"` / `"appId"` / `"productName"` / `"extends"` / `"publish"` / neutralino `"applicationId"` / `"defaultMode"` / `"enableServer"` / `"enableNativeAPI"` / `"tokenSecurity"` / `"nativeAllowList"` / `"modes"` / `"globalVariables"` / electron-builder `"directories"` / `"asar"` / `"electronVersion"` / `"artifactName"` / `"nsis": {` / wails `"assetdir"` / `"frontend:dir"` / `"wailsjsdir"` / `"nsisType"` / `"debounceMS"` / `"obfuscated"` / `"bindings"` / nest-cli `"collection"` / `"entryFile"` / `"generateOptions"` / angular `"architect"` / `"schematics"` / `"projectType"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"documentRoot"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / `"outDir"` / package.json `"scripts": {` stay out. Shared `"hooks": {` also matches ionic; shared `"plugins": [` also matches babel / eslint / prettier / biome / stylelint / dprint when shapes match.

## [0.3.726] — 2026-11-09

View:

- **Function List** lists Neutralino.js `neutralino.config.json` JSON keys: `"applicationId": "…"`, `"defaultMode": "window"` / `"browser"` / `"cloud"` / `"chrome"`, `"tokenSecurity": "one-time"` / `"none"`, `"storageLocation": "app"` / `"system"`, `"modes": {`, `"globalVariables": {`, `"logging": {`, `"nativeAllowList": [`, `"nativeBlockList": [`, `"extensions": [`, `"enableServer": true`, `"enableNativeAPI": true`, `"enableExtensions": true`, `"exportAuthInfo": true`, `"singlePageServe": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"url"` / `"port"` / `"documentRoot"` / `"cli"` / electron-builder `"directories"` / `"asar"` / `"electronVersion"` / `"artifactName"` / `"nsis": {` / wails `"assetdir"` / `"frontend:dir"` / `"wailsjsdir"` / `"nsisType"` / `"debounceMS"` / `"obfuscated"` / `"bindings"` / nest-cli `"collection"` / `"entryFile"` / `"generateOptions"` / angular `"architect"` / `"schematics"` / `"projectType"` / `"cli": {` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"hooks"` / `"documentRoot"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts": {` stay out.

## [0.3.725] — 2026-11-09

View:

- **Function List** lists electron-builder `electron-builder.json` JSON keys: `"directories": {`, `"nsis": {`, `"dmg": {`, `"mac": {`, `"win": {`, `"linux": {`, `"mas": {`, `"pkg": {`, `"portable": {`, `"appx": {`, `"snap": {`, `"appImage": {`, `"flatpak": {`, `"squirrelWindows": {`, `"extraMetadata": {`, `"files": [`, `"extraResources": [`, `"extraFiles": [`, `"fileAssociations": [`, `"protocols": [`, `"asarUnpack": [` / `"…"`, `"electronVersion": "…"`, `"electronDist": "…"`, `"artifactName": "…"`, `"copyright": "…"`, `"icon": "…"`, `"afterSign": "…"`, `"afterPack": "…"`, `"beforePack": "…"`, `"beforeBuild": "…"`, `"afterAllArtifactBuild": "…"`, `"buildVersion": "…"`, `"compression": "store"` / `"normal"` / `"maximum"`, `"asar": true`, `"nodeGypRebuild": true`, `"npmRebuild": true`, `"buildDependenciesFromSource": true`, `"forceCodeSigning": true`, `"removePackageScripts": true`, `"removePackageKeywords": true`, `"includeSubNodeModules": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"appId"` / `"productName"` / `"extends"` / `"publish"` / wails `"assetdir"` / `"frontend:dir"` / `"wailsjsdir"` / `"nsisType"` / `"debounceMS"` / `"obfuscated"` / `"bindings"` / nest-cli `"collection"` / `"entryFile"` / `"generateOptions"` / angular `"architect"` / `"schematics"` / `"projectType"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"hooks"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts": {` stay out.

## [0.3.724] — 2026-11-09

View:

- **Function List** lists Wails `wails.json` JSON keys: `"assetdir": "…"`, `"reloaddirs": "…"`, `"build:dir": "…"`, `"build:tags": "…"`, `"frontend:dir": "…"`, `"frontend:install": "…"`, `"frontend:build": "…"`, `"frontend:dev": "…"`, `"frontend:dev:build": "…"`, `"frontend:dev:install": "…"`, `"frontend:dev:watcher": "…"`, `"frontend:dev:serverUrl": "…"` / `"auto"`, `"wailsjsdir": "…"`, `"outputfilename": "…"`, `"devServer": "…"`, `"appargs": "…"`, `"garbleargs": "…"`, `"nsisType": "multiple"` / `"single"`, `"debounceMS": 100`, `"viteServerTimeout": 10`, `"runNonNativeBuildHooks": true`, `"obfuscated": true`, `"preBuildHooks": {`, `"postBuildHooks": {`, `"author": {`, `"info": {`, `"bindings": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / nest-cli `"collection"` / `"entryFile"` / `"generateOptions"` / angular `"architect"` / `"schematics"` / `"projectType"` / eas `"submit"` / `"developmentClient"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"hooks"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts": {` stay out.

## [0.3.723] — 2026-11-09

View:

- **Function List** lists NestJS `nest-cli.json` JSON keys: `"collection": "@nestjs/…"`, `"entryFile": "main"`, `"generateOptions": {`, `"monorepo": true`, `"deleteOutDir": true`, `"webpack": true`, `"webpackConfigPath": "…"`, `"tsConfigPath": "…"`, `"watchAssets": true`, `"manualRestart": true`, `"typeCheck": true`, `"builder": "tsc"` / `"swc"` / `"webpack"`, `"language": "ts"` / `"js"`, `"exec": "node"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"version"` / `"name"` / `"projects"` / `"root"` / `"sourceRoot"` / `"compilerOptions"` / `"plugins"` / `"assets"` / angular `"architect"` / `"schematics"` / `"cli"` / `"projectType"` / `"newProjectRoot"` / `"prefix"` / `"aot"` / `"assets": [` / eas `"submit"` / `"developmentClient"` / `"distribution"` / expo `"expo"` / `"slug"` / ionic `"integrations"` / `"hooks"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts": {` stay out. Nest `"builder": "tsc"` / `"swc"` / `"webpack"` (no `@scope:target`) is listed; Angular `"builder": "@angular…:…"` stays on the angular path.

## [0.3.722] — 2026-11-09

View:

- **Function List** lists Angular CLI `angular.json` JSON keys: `"architect": {`, `"schematics": {`, `"cli": {`, `"stylePreprocessorOptions": {`, `"i18n": {`, `"newProjectRoot": "…"`, `"defaultProject": "…"`, `"projectType": "application"` / `"library"`, `"sourceRoot": "…"`, `"prefix": "…"`, `"builder": "@angular…:…"`, `"outputPath": "…"`, `"index": "…"`, `"main": "…"`, `"polyfills": "…"` / `[`, `"tsConfig": "…"`, `"defaultConfiguration": "…"`, `"outputHashing": "all"` / `"media"` / `"bundles"` / `"none"`, `"inlineStyleLanguage": "css"` / `"scss"` / `"sass"` / `"less"`, `"browser": "…"`, `"browserTarget": "…"`, `"buildTarget": "…"`, `"servePath": "…"`, `"publicHost": "…"`, `"baseHref": "…"`, `"deployUrl": "…"`, `"resourcesOutputPath": "…"`, `"crossOrigin": "anonymous"` / `"use-credentials"` / `"none"`, `"ngswConfigPath": "…"`, `"webWorkerTsConfig": "…"`, `"server": "…"` (path string), `"assets": [`, `"styles": [`, `"scripts": [`, `"fileReplacements": [`, `"budgets": [`, `"allowedCommonJsDependencies": [`, `"aot": true`, `"buildOptimizer": true`, `"vendorChunk": true`, `"namedChunks": true`, `"extractLicenses": true`, `"extractCss": true`, `"subresourceIntegrity": true`, `"deleteOutputPath": true`, `"preserveSymlinks": true`, `"progress": true`, `"verbose": true`, `"statsJson": true`, `"watch": true`, `"commonChunk": true`, `"forceIvy": true`, `"enableIvy": true`, `"ssl": true`, `"hmr": true`, `"liveReload": true`, `"open": true`, `"optimization": true` / `{`, `"sourceMap": true` / `{`, `"serviceWorker": true` / `"…"`, `"prerender": true` / `{`, `"ssr": true` / `{`, `"localize": true` / `[`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"version"` / `"name"` / `"projects"` / `"root"` / `"options"` / `"configurations"` / `"build"` / `"serve"` / `"test"` / `"lint"` / eas `"submit"` / `"developmentClient"` / `"distribution"` / `"credentialsSource"` / `"appVersionSource"` / `"channel"` / `"resourceClass"` / `"buildArtifactPaths"` / expo `"expo"` / `"slug"` / `"orientation"` / `"splash"` / ionic `"integrations"` / `"hooks"` / `"proxies"` / `"watchPatterns"` / `"documentRoot"` / `"type": "angular"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / `"server": {` / `"cordova"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts": {` stay out. Angular `"scripts": [` (array) is listed.

## [0.3.721] — 2026-11-09

View:

- **Function List** lists Expo EAS `eas.json` JSON keys (top-level + build/submit profiles): `"submit": {`, `"developmentClient": true`, `"withoutCredentials": true`, `"simulator": true`, `"distribution": "store"` / `"internal"` / `"simulator"`, `"credentialsSource": "local"` / `"remote"`, `"appVersionSource": "local"` / `"remote"`, `"autoIncrement": true` / `"version"` / `"buildNumber"`, `"channel": "…"`, `"releaseChannel": "…"`, `"resourceClass": "…"`, `"prebuildCommand": "…"`, `"extends": "…"`, `"buildArtifactPaths": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"cli": {` / `"build": {` / `"env": {` / `"cache": {` / `"name"` / `"version"` / `"node"` / `"yarn"` / `"pnpm"` / `"bun"` / expo `"expo"` / `"slug"` / `"orientation"` / `"userInterfaceStyle"` / `"splash"` / `"assetBundlePatterns"` / `"jsEngine"` / `"newArchEnabled"` / ionic `"integrations"` / `"hooks"` / `"proxies"` / `"watchPatterns"` / `"documentRoot"` / `"type": "angular"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / `"server"` / `"cordova"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out.

## [0.3.720] — 2026-11-09

View:

- **Function List** lists Expo `app.json` / `app.config.json` JSON keys (often under `"expo"`): `"expo": {`, `"splash": {`, `"updates": {`, `"notification": {`, `"androidStatusBar": {`, `"androidNavigationBar": {`, `"experiments": {`, `"runtimeVersion": {` / `"…"`, `"assetBundlePatterns": [`, `"platforms": [`, `"slug": "…"`, `"orientation": "portrait"` / `"landscape"` / `"default"`, `"userInterfaceStyle": "light"` / `"dark"` / `"automatic"`, `"scheme": "…"`, `"owner": "…"`, `"sdkVersion": "…"`, `"primaryColor": "…"`, `"jsEngine": "hermes"` / `"jsc"`, `"icon": "…"`, `"newArchEnabled": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"description"` / `"extra": {` / `"plugins": [` / `"ios": {` / `"android": {` / `"web": {` / ionic `"integrations"` / `"hooks"` / `"proxies"` / `"watchPatterns"` / `"documentRoot"` / `"type": "angular"` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / `"server"` / `"cordova"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out.

## [0.3.719] — 2026-11-09

View:

- **Function List** lists Ionic `ionic.config.json` JSON keys: `"integrations": {`, `"hooks": {`, `"proxies": [`, `"watchPatterns": [`, `"documentRoot": "…"`, `"type": "angular"` / `"angular-standalone"` / `"react"` / `"react-vite"` / `"vue"` / `"vue-vite"` / `"custom"` / `"ionic-angular"` / `"ionic1"`, `"npmClient": "npm"` / `"yarn"` / `"pnpm"` / `"bun"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"id"` / `"version"` / `"app": {` / capacitor `"appId"` / `"appName"` / `"webDir"` / `"bundledWebRuntime"` / `"server"` / `"cordova"` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"npmClient"` also matches lerna when shapes match; shared `"plugins": {` stays on capacitor / PostCSS.

## [0.3.718] — 2026-11-09

View:

- **Function List** lists Capacitor `capacitor.config.json` JSON keys: `"appId": "com.app…"`, `"appName": "…"`, `"webDir": "…"`, `"backgroundColor": "…"`, `"loggingBehavior": "none"` / `"debug"` / `"production"`, `"overrideUserAgent": "…"`, `"appendUserAgent": "…"`, `"androidScheme": "…"`, `"iosScheme": "…"`, `"server": {`, `"plugins": {`, `"android": {`, `"ios": {`, `"cordova": {`, `"includePlugins": [`, `"packageClassList": [`, `"bundledWebRuntime": true`, `"zoomEnabled": true`, `"handleApplicationNotifications": true`, `"scrollEnabled": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"app": {` / tauri `"productName"` / `"identifier"` / `"withGlobalTauri"` / `"allowlist"` / `"frontendDist"` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"plugins": {` also matches PostCSS when shapes match.

## [0.3.717] — 2026-11-09

View:

- **Function List** lists Tauri `tauri.conf.json` JSON keys: `"productName": "…"`, `"identifier": "com.app…"`, `"frontendDist": "…"`, `"distDir": "…"`, `"devUrl": "…"`, `"devPath": "…"`, `"beforeDevCommand": "…"`, `"beforeBuildCommand": "…"`, `"beforeBundleCommand": "…"`, `"copyright": "…"`, `"category": "…"`, `"shortDescription": "…"`, `"longDescription": "…"`, `"build": {`, `"bundle": {`, `"tauri": {`, `"security": {`, `"trayIcon": {`, `"systemTray": {`, `"allowlist": {`, `"updater": {`, `"linux": {`, `"macOS": {`, `"additionalWatchFolders": [`, `"windows": [`, `"withGlobalTauri": true`, `"removeUnusedCommands": true`, `"macOSPrivateApi": true`, `"enableGTKAppId": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / `"version"` / `"app": {` / `"plugins": {` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / firebase `"hosting"` / `"firestore"` / `"emulators"` / `"auth"` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / PostCSS `"plugins": {` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"build": {` also matches vercel when shapes match.

## [0.3.716] — 2026-11-09

View:

- **Function List** lists Firebase `firebase.json` JSON keys: `"hosting": {` / `[`, `"functions": {` / `[`, `"firestore": {`, `"database": {` / `[`, `"storage": {` / `[`, `"emulators": {`, `"remoteconfig": {`, `"extensions": {`, `"flutter": {`, `"dataconnect": {`, `"apphosting": {` / `[`, `"auth": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / vercel `"rewrites"` / `"redirects"` / `"headers"` / `"crons"` / `"regions"` / `"framework"` / `"buildCommand"` / `"cleanUrls"` / `"version": 2` / changesets `"baseBranch"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / PostCSS `"plugins": {` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"functions": {` also matches vercel when shapes match; firebase multi-codebase `"functions": [` stays here (vercel rejects the array form).

## [0.3.715] — 2026-11-09

View:

- **Function List** lists Vercel `vercel.json` JSON keys: `"rewrites": [`, `"redirects": [`, `"headers": [`, `"crons": [`, `"regions": [`, `"functionFailoverRegions": [`, `"builds": [`, `"routes": [`, `"functions": {`, `"images": {`, `"build": {`, `"env": {`, `"github": {`, `"git": {`, `"framework": "nextjs"` / `null`, `"buildCommand": "…"`, `"installCommand": "…"`, `"devCommand": "…"`, `"outputDirectory": "…"`, `"ignoreCommand": "…"`, `"bulkRedirectsPath": "…"`, `"bunVersion": "1.x"`, `"cleanUrls": true`, `"trailingSlash": false`, `"fluid": true`, `"public": true`, `"skipMiddlewareRequestBody": true`, `"version": 2`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / `"name"` / changesets `"baseBranch"` / `"access"` / rush `"rushVersion"` / `"projects"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / PostCSS `"plugins": {` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"env": {` also matches other JSON when shapes match; `"version": 2` is number-shaped (lerna string `"version"` stays on lerna).

## [0.3.714] — 2026-11-09

View:

- **Function List** lists Changesets `.changeset/config.json` JSON keys: `"changelog": "@changesets/cli/changelog"` / `[` / `false`, `"commit": false`, `"fixed": [`, `"linked": [`, `"access": "restricted"` / `"public"`, `"baseBranch": "main"`, `"updateInternalDependencies": "patch"` / `"minor"`, `"ignore": [`, `"prettier": true`, `"privatePackages": {` / `false`, `"snapshot": {`, `"changedFilePatterns": [`, `"bumpVersionsWithWorkspaceProtocolOnly": true`, `"___experimentalUnsafeOptions_WILL_CHANGE_IN_PATCH": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / rush `"rushVersion"` / `"projects"` / `"ensureConsistentVersions"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / PostCSS `"plugins": {` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"ignore": [` also matches babel / pyrightconfig when shapes match; shared `"changelog": true` / `false` also matches lerna when shapes match.

## [0.3.713] — 2026-11-09

View:

- **Function List** lists Rush `rush.json` JSON keys: `"rushVersion": "5.147.1"`, `"pnpmVersion": "8.15.0"` / `"npmVersion"` / `"yarnVersion"`, `"projects": [`, `"repository": {`, `"gitPolicy": {`, `"approvedPackagesPolicy": {`, `"eventHooks": {`, `"npmOptions": {`, `"pnpmOptions": {`, `"yarnOptions": {`, `"variants": [`, `"allowedProjectTags": [`, `"nodeSupportedVersionRange": ">=18"`, `"nodeSupportedVersionInstructions": "…"`, `"projectFolderMinDepth": 1`, `"projectFolderMaxDepth": 2`, `"ensureConsistentVersions": true`, `"telemetryEnabled": true`, `"hotfixChangeEnabled": true`, `"suppressNodeLtsWarning": true`, `"suppressRushIsPublicVersionCheck": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / lerna `"npmClient"` / `"useWorkspaces"` / `"packages": [` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / PostCSS `"plugins": {` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out.

## [0.3.712] — 2026-11-09

View:

- **Function List** lists Lerna `lerna.json` JSON keys: `"version": "independent"` / `"1.2.3"`, `"npmClient": "npm"` / `"yarn"` / `"pnpm"` / `"bun"`, `"packages": [`, `"command": {`, `"ignoreChanges": [`, `"nohoist": [`, `"npmClientArgs": [`, `"allowBranch": "…"` / `[`, `"registry": "…"`, `"changelogPreset": "…"`, `"message": "…"`, `"loglevel": "info"` / `"verbose"` / …, `"useWorkspaces": true`, `"useNx": true`, `"stream": true`, `"sort": true`, `"rejectCycles": true`, `"conventionalCommits": true`, `"exact": true`, `"forceLocal": true`, `"hoist": true`, `"syncWorkspaceLock": true`, `"granularPathspec": true`, `"changelog": true`, `"progress": true`, `"ci": true`, `"concurrency": 4`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / nx `"namedInputs"` / `"targetDefaults"` / `"cacheDirectory"` / Parcel `"transformers"` / PostCSS `"plugins": {` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"concurrency": N` also matches turbo when shapes match; shared `"packages": [` also matches Pipfile-shaped tables when shapes match.

## [0.3.711] — 2026-11-08

View:

- **Function List** lists Nx `nx.json` / `nx.jsonc` JSON keys: `"extends": "…"` / `"extends": [`, `"namedInputs": {`, `"targetDefaults": {`, `"tasksRunnerOptions": {`, `"workspaceLayout": {`, `"generators": {`, `"plugins": [`, `"pluginsConfig": {`, `"affected": {`, `"release": {`, `"sync": {`, `"tui": {`, `"cli": {`, `"installation": {`, `"defaultProject": "…"`, `"defaultBase": "…"`, `"npmScope": "…"`, `"cacheDirectory": "…"`, `"nxCloudId": "…"`, `"nxCloudUrl": "…"`, `"parallel": 3`, `"useDaemonProcess": true`, `"useInferencePlugins": true`, `"neverConnectToCloud": true`, `"analytics": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / turbo `"tasks"` / `"pipeline"` / `"globalDependencies"` / `"cacheDir"` / `"envMode"` / Parcel `"transformers"` / `"namers"` / PostCSS `"plugins": {` / babel `"presets"` / SWC `"jsc"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / biome `"linter"` / dprint `"includes"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"extends": "…"` / `"extends": [` also match parcel / babel / eslint / stylelint / turbo / tsconfig when shapes match; shared `"plugins": [` also match eslint / prettier / stylelint / babel / dprint when shapes match.

## [0.3.710] — 2026-11-08

View:

- **Function List** lists Turborepo `turbo.json` / `turbo.jsonc` JSON keys: `"extends": [`, `"tasks": {`, `"pipeline": {` (legacy), `"globalDependencies": [`, `"globalEnv": [`, `"globalPassThroughEnv": [`, `"tags": [`, `"global": {`, `"futureFlags": {`, `"remoteCache": {`, `"experimentalObservability": {`, `"ui": "tui"` / `"stream"`, `"envMode": "strict"` / `"loose"`, `"cacheDir": "…"`, `"cacheMaxAge": "7d"`, `"cacheMaxSize": "10GB"`, `"concurrency": "1"` / `"50%"` / `10`, `"daemon": true`, `"noUpdateNotifier": true`, `"agentGuidance": true`, `"dangerouslyDisablePackageManagerCheck": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / Parcel `"transformers"` / `"namers"` / `"bundlers"` / PostCSS `"plugins"` / `"parser"` / babel `"presets"` / `"parserOpts"` / SWC `"jsc"` / `"minify"` / eslint `"root"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / `"semi"` / biome `"linter"` / `"formatter"` / dprint `"includes"` / `"newLineKind"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"extends": [` also match parcel / babel / eslint / stylelint; shared `"tasks": {` also match package.json / deno.json when shapes match.

## [0.3.709] — 2026-11-08

View:

- **Function List** lists Parcel `.parcelrc` / `.parcelrc.json` / `parcelrc.json` JSON keys: `"extends": "…"` / `"extends": [`, `"transformers": {`, `"namers": [`, `"bundlers": {`, `"runtimes": [`, `"optimizers": {`, `"packagers": {`, `"compressors": {`, `"resolvers": [`, `"validators": {`, `"reporters": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / PostCSS `"plugins"` / `"parser"` / `"syntax"` / `"stringifier"` / `"map"` / babel `"presets"` / `"parserOpts"` / `"rootMode"` / SWC `"jsc"` / `"inlineSourcesContent"` / `"isModule"` / `"minify"` / eslint `"root"` / `"rules"` / `"parserOptions"` / `"ignorePatterns"` / stylelint `"ignoreFiles"` / `"customSyntax"` / `"processors"` / prettier `"printWidth"` / `"semi"` / biome `"linter"` / `"formatter"` / dprint `"includes"` / `"newLineKind"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"extends": "…"` / `"extends": [` also match babel / eslint / stylelint when shapes match.

## [0.3.708] — 2026-11-08

View:

- **Function List** lists PostCSS `.postcssrc` / `.postcssrc.json` / `postcss.config.json` JSON keys: `"plugins": {` / `"plugins": [`, `"parser": "…"`, `"syntax": "…"`, `"stringifier": "…"`, `"map": true` / `false` / `{`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / babel `"presets"` / `"parserOpts"` / `"rootMode"` / SWC `"jsc"` / `"inlineSourcesContent"` / `"isModule"` / `"minify"` / eslint `"root"` / `"rules"` / `"parserOptions"` / `"ignorePatterns"` / stylelint `"ignoreFiles"` / `"customSyntax"` / `"processors"` / prettier `"printWidth"` / `"semi"` / biome `"linter"` / `"formatter"` / dprint `"includes"` / `"newLineKind"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"parser": "…"` / `"plugins": [` also match eslint / prettier / stylelint / babel when shapes match. `"plugins": {` is PostCSS-shaped (eslint plugins is `[`).

## [0.3.707] — 2026-11-08

View:

- **Function List** lists SWC `.swcrc` / `.swcrc.json` / `swc.config.json` JSON keys: `"jsc": {`, `"module": {`, `"env": {`, `"minify": true`, `"sourceMaps": true` / `"inline"`, `"inlineSourcesContent": true`, `"isModule": true` / `"unknown"`, `"sourceRoot": "…"`, `"test": [` / `"…"`, `"exclude": [` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / babel `"presets"` / `"parserOpts"` / `"rootMode"` / `"passPerPreset"` / eslint `"root"` / `"parser"` / `"rules"` / stylelint `"ignoreFiles"` / prettier `"printWidth"` / `"semi"` / biome `"linter"` / `"formatter"` / dprint `"includes"` / `"newLineKind"` / tsconfig `"compilerOptions"` / `"module": "ESNext"` (string) / package.json `"scripts"` stay out. Shared `"env"` / `"sourceMaps"` / `"exclude"` also match babel / eslint / tsconfig when shapes match.

## [0.3.706] — 2026-11-08

View:

- **Function List** lists Babel `.babelrc` / `.babelrc.json` / `babel.config.json` JSON keys: `"presets": [`, `"plugins": [`, `"env": {`, `"overrides": [`, `"ignore": [` / `"…"`, `"only": [` / `"…"`, `"extends": "…"`, `"sourceType": "module"` / `"script"` / `"unambiguous"`, `"sourceMaps": true` / `"inline"` / `"both"`, `"retainLines": true`, `"comments": true`, `"compact": true` / `"auto"`, `"minified": true`, `"babelrc": true`, `"rootMode": "root"` / `"upward"` / `"upward-optional"`, `"parserOpts": {`, `"generatorOpts": {`, `"passPerPreset": true`, `"assumptions": {`, `"browserslistEnv": "…"`, `"browserslistConfigFile": true` / `"…"`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / eslint `"root"` / `"parser"` / `"parserOptions"` / `"rules"` / `"ignorePatterns"` / `"languageOptions"` / stylelint `"ignoreFiles"` / `"customSyntax"` / `"processors"` / prettier `"printWidth"` / `"semi"` / biome `"linter"` / `"formatter"` / dprint `"includes"` / `"newLineKind"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"plugins"` / `"overrides"` / `"extends"` / `"env"` also match eslint / prettier / stylelint when shapes match.

## [0.3.705] — 2026-11-08

View:

- **Function List** lists Stylelint `.stylelintrc` / `.stylelintrc.json` / `stylelint.config.json` JSON keys: `"extends": "…"` / `"extends": [`, `"plugins": [`, `"processors": [`, `"rules": {`, `"overrides": [`, `"ignoreFiles": [` / `"…"`, `"customSyntax": "…"`, `"defaultSeverity": "warning"` / `"error"`, `"configurationComment": "…"`, `"ignoreDisables": true`, `"reportNeedlessDisables": true`, `"reportInvalidScopeDisables": true`, `"reportDescriptionlessDisables": true`, `"reportUnscopedDisables": true`, `"allowEmptyInput": true`, `"quietDeprecationWarnings": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / eslint `"root"` / `"env"` / `"parser"` / `"parserOptions"` / `"ignorePatterns"` / `"processor"` / `"languageOptions"` / prettier `"printWidth"` / `"semi"` / biome `"linter"` / `"formatter"` / dprint `"includes"` / `"newLineKind"` / tsconfig `"compilerOptions"` / package.json `"scripts"` stay out. Shared `"extends"` / `"plugins"` / `"rules"` / `"overrides"` also match eslint / prettier when shapes match.

## [0.3.704] — 2026-11-08

View:

- **Function List** lists ESLint `.eslintrc` / `.eslintrc.json` / `eslint.config.json` JSON keys: `"root": true`, `"env": {`, `"extends": "…"` / `"extends": [`, `"parser": "…"`, `"parserOptions": {`, `"plugins": [`, `"rules": {`, `"overrides": [`, `"globals": {`, `"ignorePatterns": [` / `"…"`, `"settings": {`, `"processor": "…"`, `"noInlineConfig": true`, `"reportUnusedDisableDirectives": true` / `"error"` / `"warn"` / `"off"`, `"ignores": [`, `"languageOptions": {`, `"linterOptions": {`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"$schema"` / prettier `"printWidth"` / `"semi"` / biome `"linter"` / `"formatter"` / `"indentStyle"` / dprint `"includes"` / `"newLineKind"` / tsconfig `"compilerOptions"` / `"files"` / package.json `"scripts"` stay out. Shared `"plugins"` / `"overrides"` arrays also match prettier when shapes match. `"extends"` string also matches tsconfig.

## [0.3.703] — 2026-11-08

View:

- **Function List** lists Prettier `.prettierrc` / `.prettierrc.json` / `prettier.config.json` JSON keys: `"printWidth": 80`, `"tabWidth": 2`, `"useTabs": false`, `"semi": true`, `"singleQuote": false`, `"jsxSingleQuote": false`, `"bracketSpacing": true`, `"bracketSameLine": false`, `"jsxBracketSameLine": false`, `"requirePragma": false`, `"insertPragma": false`, `"vueIndentScriptAndStyle": false`, `"singleAttributePerLine": false`, `"experimentalTernaries": false`, `"quoteProps": "as-needed"` / `"consistent"` / `"preserve"`, `"trailingComma": "all"` / `"es5"` / `"none"`, `"arrowParens": "always"` / `"avoid"`, `"proseWrap": "always"` / `"never"` / `"preserve"`, `"htmlWhitespaceSensitivity": "css"` / `"strict"` / `"ignore"`, `"endOfLine": "lf"` / `"crlf"` / `"cr"` / `"auto"`, `"embeddedLanguageFormatting": "auto"` / `"off"`, `"objectWrap": "preserve"` / `"collapse"`, `"experimentalOperatorPosition": "start"` / `"end"`, `"plugins": [`, `"overrides": [`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"parser"` / `"filepath"` / `"$schema"` stay out. biome `"indentStyle"` / `"lineWidth"` / `"trailingCommas"` (plural) / `"arrowParentheses"` / `"quoteProperties"` (camel) stay on biome. dprint `"indentWidth"` / `"newLineKind"` / `"includes"` stay on dprint. package.json / composer / tsconfig / Pyre / pyrightconfig / biome / dprint structural keys stay out where shapes differ.

## [0.3.702] — 2026-11-08

View:

- **Function List** lists dprint `dprint.json` / `.dprint.json` JSON keys: `"typescript": {`, `"json": {`, `"jsonc": {`, `"markdown": {`, `"toml": {`, `"dockerfile": {`, `"rust": {`, `"python": {`, `"ruff": {`, `"malva": {`, `"markup": {`, `"yaml": {`, `"graphql": {`, `"jupyter": {`, `"exec": {`, `"css": {`, `"javascript": {`, `"biome": {`, `"plugins": [`, `"includes": [`, `"excludes": [`, `"incremental": true`, `"useTabs": false`, `"newLineKind": "lf"` / `"crlf"` / `"auto"` / `"system"`, `"indentWidth": 2`, `"lineWidth": 80`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"extends"` / `"$schema"` / biome-only keys (`"linter"`, `"formatter"`, `"organizeImports"`, `"indentStyle"`, `"lineEnding"`) stay out. tsconfig `"include": [` / `"exclude": [` stay on their paths (dprint uses plural `"includes"` / `"excludes"`). package.json / composer / tsconfig / Pyre / pyrightconfig / biome structural keys stay out where shapes differ.

## [0.3.701] — 2026-11-08

View:

- **Function List** lists Biome `biome.json` / `biome.jsonc` JSON keys: `"linter": {`, `"formatter": {`, `"organizeImports": {`, `"assist": {`, `"vcs": {`, `"files": {`, `"javascript": {`, `"json": {`, `"css": {`, `"graphql": {`, `"html": {`, `"overrides": [`, `"plugins": [`, `"indentStyle": "tab"` / `"space"`, `"lineEnding": "lf"` / `"crlf"` / `"cr"`, `"attributePosition": "auto"` / `"multiline"`, `"clientKind": "git"`, `"quoteStyle": "double"` / `"single"`, `"jsxQuoteStyle": "double"` / `"single"`, `"quoteProperties": "asNeeded"` / `"preserve"`, `"trailingCommas": "all"` / `"es5"` / `"none"`, `"semicolons": "always"` / `"asNeeded"`, `"arrowParentheses": "always"` / `"asNeeded"`, `"defaultBranch": "main"`, `"indentWidth": 2`, `"lineWidth": 100`, `"maxSize": 1048576`, `"bracketSpacing": true`, `"bracketSameLine": false`, `"useEditorconfig": true`, `"useIgnoreFile": true`, `"ignoreUnknown": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Generic `"enabled": true` / `"rules": {` / `"extends"` stay out. tsconfig `"files": [` / package.json `"overrides": {` stay on their paths. package.json / composer / tsconfig / Pyre / pyrightconfig structural keys stay out.

## [0.3.700] — 2026-11-08

View:

- **Function List** lists TypeScript / JS `tsconfig.json` / `jsconfig.json` JSON keys: `"compilerOptions": {`, `"extends": "…"`, `"files": [`, `"include": [`, `"exclude": [`, `"references": [`, `"target": "ES2022"` / `"ESNext"` / `"ES5"` / `"ES6"` / `"ES2015"`…`"ES2024"`, `"module": "ESNext"` / `"CommonJS"` / `"NodeNext"` / `"Node16"` / `"ES2022"` / `"AMD"` / `"UMD"` / `"System"` / `"None"`, `"moduleResolution": "bundler"` / `"node"` / `"node16"` / `"nodenext"` / `"classic"`, `"jsx": "react-jsx"` / `"react-jsxdev"` / `"react"` / `"react-native"` / `"preserve"`, `"lib": [`, `"types": [`, `"typeRoots": [`, `"outDir": "dist"`, `"rootDir": "src"`, `"baseUrl": "."`, `"strict": true`, `"noEmit": true`, `"declaration": true`, `"declarationMap": true`, `"sourceMap": true`, `"skipLibCheck": true`, `"esModuleInterop": true`, `"allowJs": true`, `"checkJs": true`, `"resolveJsonModule": true`, `"isolatedModules": true`, `"verbatimModuleSyntax": true`, `"noUnusedLocals": true`, `"noUnusedParameters": true`, `"noFallthroughCasesInSwitch": true`, `"exactOptionalPropertyTypes": true`, `"noImplicitOverride": true`, `"noPropertyAccessFromIndexSignature": true`, `"noUncheckedIndexedAccess": true`, `"forceConsistentCasingInFileNames": true`, `"incremental": true`, `"composite": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Maps other than `compilerOptions` (`"paths": {`) stay out. package.json / composer.json structural keys, Pyre / pyrightconfig JSON, and leaf metadata stay out.

## [0.3.699] — 2026-11-08

View:

- **Function List** lists Pyright / basedpyright `pyrightconfig.json` JSON keys: `"include": [`, `"exclude": [`, `"ignore": [`, `"strict": [`, `"extraPaths": [`, `"executionEnvironments": [`, `"typeCheckingMode": "strict"` / `"standard"` / `"basic"` / `"off"` / `"recommended"` / `"all"`, `"pythonVersion": "3.12"`, `"pythonPlatform": "Linux"` / `"Darwin"` / `"Windows"` / `"All"`, `"stubPath": "typings"`, `"venvPath": "."`, `"venv": ".venv"`, `"typeshedPath": "stubs"`, `"useLibraryCodeForTypes": true`, `"enableTypeIgnoreComments": true`, `"enableReachabilityAnalysis": true`, `"deprecateTypingAliases": false`, `"disableBytesTypePromotions": true`, `"analyzeUnannotatedFunctions": true`, `"strictListInference": true`, `"strictDictionaryInference": true`, `"strictSetInference": true`, `"strictParameterNoneValue": true`, `"enableExperimentalFeatures": false`, `"reportMissingImports": "error"` / `true`, `"reportMissingTypeStubs": false` / `"warning"`, `"reportUnusedImport": "none"`, `"failOnWarnings": true`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. Maps (`"defineConstant": {`) stay out. package.json / composer.json structural keys, Pyre `.pyre_configuration`, pyright / ty TOML, and leaf metadata stay out.

## [0.3.698] — 2026-11-08

View:

- **Function List** lists Pyre `.pyre_configuration` / `.pyre_configuration.local` JSON keys: `"source_directories": [`, `"search_path": [`, `"exclude": [`, `"extensions": [`, `"ignore_all_errors": [`, `"site_roots": [`, `"binary": "…"`, `"logger": "…"`, `"typeshed": "…"`, `"python_version": "3.10.0"`, `"system_platform": "linux"` / `"win32"` / `"darwin"` / `"cygwin"` / `"aix"` / `"freebsd"` / `"netbsd"` / `"openbsd"` / `"sunos5"` / `"emscripten"` / `"wasi"`, `"site_package_search_strategy": "all"` / `"none"` / `"pep561"`, `"strict": true`, `"workers": 4`. Optional trailing `,` and `//` / `/*` comment. Incomplete quotes / missing `:` stay out. package.json / composer.json structural keys, pyright / ty TOML, and leaf metadata stay out.

## [0.3.697] — 2026-11-08

View:

- **Function List** lists Astral ty `pyproject.toml` / `ty.toml` `[tool.ty]` sections and fields: `[tool.ty]`, `[tool.ty.rules]`, `[tool.ty.analysis]`, `[tool.ty.environment]`, `[tool.ty.src]`, `[tool.ty.terminal]`, `[[tool.ty.overrides]]`, `[tool.ty.overrides.rules]`, `[tool.ty.overrides.analysis]`, `baseline = "ty-baseline.json"`, `respect-type-ignore-comments = false`, `return-type-inference = "always"` / `"never"`, `strict-equality-semantics = true`, `strict-generic-narrowing = true`, `python-version = "3.12"`, `python-platform = "linux"` / `"win32"` / `"darwin"` / `"android"` / `"ios"` / `"all"`, `typeshed = "stubs"`, `exclude-scripts = true`, `respect-ignore-files = false`, `error-on-warning = false`, `output-format = "full"` / `"concise"` / `"github"` / `"gitlab"` / `"junit"` / `"sarif"`, `all = "error"` / `"warn"` / `"ignore"`, `possibly-unresolved-reference = "warn"`, `division-by-zero = "ignore"`, `unused-ignore-comment = "warn"`, `redundant-cast = "ignore"`, `possibly-missing-attribute = "error"`, `possibly-missing-import = "error"`, `index-out-of-bounds = "ignore"`, `import-not-found = "warn"`. Exact kebab-case. TOML spaces around `=`. Optional quotes on string values. Optional `#` / `;` comment. Arrays (`include` / `exclude` / `extra-paths` / `root` / `allowed-unresolved-imports` / `replace-imports-with-any`) stay out. Bare `python` / `quiet` / `verbose` / `color` / `path` / `config` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / autoflake / pytype / unimport / pycln / flynt / pyupgrade / refurb / pyright / basedpyright / tox / setuptools / Cargo / Pipfile / pyproject `[tool]` umbrella / ruff / Desktop Entry / pip.conf stay out.

## [0.3.696] — 2026-11-08

View:

- **Function List** lists Pyright / basedpyright `pyproject.toml` `[tool.pyright]` / `[tool.basedpyright]` sections and fields: `[tool.pyright]`, `[tool.basedpyright]`, `typeCheckingMode = "strict"` / `"standard"` / `"basic"` / `"off"` / `"recommended"` / `"all"`, `pythonVersion = "3.12"`, `pythonPlatform = "Linux"` / `"Darwin"` / `"Windows"` / `"All"`, `stubPath = "typings"`, `venvPath = "."`, `venv = ".venv"`, `useLibraryCodeForTypes = true`, `enableTypeIgnoreComments = true`, `enableReachabilityAnalysis = true`, `deprecateTypingAliases = false`, `disableBytesTypePromotions = true`, `analyzeUnannotatedFunctions = true`, `strictListInference = true`, `strictDictionaryInference = true`, `strictSetInference = true`, `strictParameterNoneValue = true`, `enableExperimentalFeatures = false`, `reportMissingImports = "error"` / `true`, `reportMissingTypeStubs = false` / `"warning"`, `reportUnusedImport = "none"`, `failOnWarnings = true` (basedpyright). Exact camelCase. TOML spaces around `=`. Optional quotes on string values. Optional `#` / `;` comment. Arrays (`include` / `exclude` / `ignore` / `extraPaths` / `strict` / `executionEnvironments`) and maps (`defineConstant`) stay out. Bare `quiet` / `verbose` / `color` / `path` / `config` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / autoflake / pytype / unimport / pycln / flynt / pyupgrade / refurb / tox / setuptools / Cargo / Pipfile / pyproject `[tool]` umbrella / ruff / Desktop Entry / pip.conf stay out.

## [0.3.695] — 2026-11-08

View:

- **Function List** lists Refurb `pyproject.toml` `[tool.refurb]` / `[[tool.refurb.amend]]` sections and fields: `[tool.refurb]`, `[[tool.refurb.amend]]`, `disable_all = true` / `enable_all = true`, `python_version = "3.10"` / `3.11`, `format = "text"` / `"github"`, `sort_by = "filename"` / `"error"`. Exact snake_case. TOML spaces around `=`. Optional quotes on string values. Optional `#` / `;` comment. Arrays (`ignore` / `enable` / `disable` / `load` / `mypy_args`) stay out. Bare `quiet` / `color` / `verbose` / `debug` / `path` / `config_file` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / autoflake / pytype / unimport / pycln / flynt / pyupgrade / tox / setuptools / Cargo / Pipfile / pyproject `[tool]` umbrella / ruff / Desktop Entry / pip.conf stay out.

## [0.3.694] — 2026-11-08

View:

- **Function List** lists pyupgrade `setup.cfg` / `tox.ini` `[pyupgrade]` sections and fields: `[pyupgrade]`, `py39-plus = true` / `py39_plus`, `py3-plus` / `py3-only` / `py36-plus`…`py315-plus`, `keep-percent-format = true` / `keep_percent_format`, `keep-mock = true` / `keep_mock`, `keep-runtime-typing = true` / `keep_runtime_typing`, `exit-zero-even-if-changed = true` / `exit_zero_even_if_changed`. Exact kebab-case or snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Bare `verbose` / `quiet` / `filenames` / `exclude` / `config` stay out. JSON/TOML arrays stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / autoflake / pytype / unimport / pycln / flynt / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pyupgrade]` tables / Desktop Entry / pip.conf stay out.

## [0.3.693] — 2026-11-08

View:

- **Function List** lists flynt `setup.cfg` / `tox.ini` / `flynt.toml` `[flynt]` sections and fields: `[flynt]`, `aggressive = true` / `false`, `dry-run = true` / `dry_run`, `stdout = true` / `false`, `string = true` / `0`, `notebook = true` / `yes`, `report = true` / `false`, `fail-on-change = true` / `fail_on_change`, `no-multiline = true` / `no_multiline`, `line-length = 88` / `line_length = 100`, `transform-percent = true` / `transform_percent` / `no-transform-percent = false` / `no_transform_percent`, `transform-format = true` / `transform_format` / `no-transform-format = false` / `no_transform_format`, `transform-concats = true` / `transform_concats`, `transform-joins = true` / `transform_joins`. Exact kebab-case or snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Bare `verbose` / `quiet` / `exclude` / `config` / `src` stay out. JSON/TOML `exclude = ["…"]` arrays stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / autoflake / pytype / unimport / pycln / tox / setuptools / Cargo / Pipfile / pyproject `[tool.flynt]` tables / Desktop Entry / pip.conf stay out.

## [0.3.692] — 2026-11-08

View:

- **Function List** lists pycln `setup.cfg` / `tox.ini` `[pycln]` sections and fields: `[pycln]`, `all = true` / `false`, `expand-stars = true` / `expand_stars`, `no-gitignore = true` / `no_gitignore`, `disable-all-dunder-policy = true` / `disable_all_dunder_policy`, `silence = true` / `0`, `check = true` / `false`, `diff = true` / `yes`, `include = .*\.py$` / `src/.*\.py$`, `exclude = .*tests/.*` / `venv/`, `extend-exclude = legacy/` / `extend_exclude`, `skip-imports = os,sys` / `skip_imports = django.db,typing` / `[os,sys]`, `path = .` / `paths = src,tests`. Exact kebab-case or snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on path / import lists ok. Bare `verbose` / `quiet` / `config` stay out. JSON/TOML `include = ["…"]` arrays stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / autoflake / pytype / unimport / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pycln]` tables / Desktop Entry / pip.conf stay out.

## [0.3.691] — 2026-11-08

View:

- **Function List** lists unimport `setup.cfg` / `tox.ini` `[unimport]` sections and fields: `[unimport]`, `include-star-import = true` / `include_star_import`, `ignore-init = true` / `ignore_init`, `gitignore = true` / `false`, `sources = src` / `src,tests` / `path1 path2`, `exclude = __init__.py|tests/` / `tests/`, `include = test|tests.py`, `remove = false` / `True`, `check = true` / `0`, `diff = true` / `yes`, `jobs = 4` / `0`. Exact kebab-case or snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on path lists ok. Bare `verbose` / `color` / `format` / `permission` / `config` / `per-file-ignores` stay out. JSON/TOML `sources = ["…"]` arrays stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / autoflake / pytype / tox / setuptools / Cargo / Pipfile / pyproject `[tool.unimport]` tables / Desktop Entry / pip.conf stay out.

## [0.3.690] — 2026-11-08

View:

- **Function List** lists pytype `setup.cfg` / `tox.ini` / `.pytype.cfg` `[pytype]` sections and fields: `[pytype]`, `inputs = .` / `src,tests` / `pineboolib`, `exclude = **/*_test.py` / `**/test_*.py,env/`, `python_version = 3.10` / `3.9`, `pythonpath = .` / `.:~/repo2` / `./src;./lib`, `disable = attribute-error` / `name-error,import-error`, `output = .pytype` / `./.pytype`, `keep_going = True` / `false`, `jobs = 4` / `auto`, `report_errors = True` / `0`, `protocols = True`, `strict_import = False`. Exact snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on path / disable lists ok. Bare `verbose` / `platform` / `module_name` / `nofail` / `check` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / autoflake / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pytype]` tables / Desktop Entry / pip.conf stay out.

## [0.3.689] — 2026-11-08

View:

- **Function List** lists autoflake `setup.cfg` / `tox.ini` `[autoflake]` sections and fields: `[autoflake]`, `remove-all-unused-imports = true` / `remove_all_unused_imports`, `expand-star-imports = True` / `expand_star_imports`, `ignore-init-module-imports = yes` / `ignore_init_module_imports`, `remove-duplicate-keys = 1` / `remove_duplicate_keys`, `remove-unused-variables = false` / `remove_unused_variables`, `remove-rhs-for-unused-variables = off` / `remove_rhs_for_unused_variables`, `ignore-pass-statements = true` / `ignore_pass_statements`, `ignore-pass-after-docstring = true` / `ignore_pass_after_docstring`, `check-diff = true` / `check_diff`, `imports = django,requests,urllib3`. Exact kebab-case or snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on import lists ok. Bare `check` / `exclude` / `jobs` / `quiet` / `verbose` / `recursive` / `in-place` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / radon / tox / setuptools / Cargo / Pipfile / pyproject `[tool.autoflake]` tables / Desktop Entry / pip.conf stay out.

## [0.3.688] — 2026-11-08

View:

- **Function List** lists Radon `radon.cfg` / `setup.cfg` / `tox.ini` `[radon]` sections and fields: `[radon]`, `cc_min = B` / `A`…`F`, `cc_max = E` / `F`, `mi_min = A` / `B` / `C`, `mi_max = C`, `show_complexity = true` / `False`, `average = yes`, `total_average = 1`, `show_closures = true`, `no_assert = false`, `order = SCORE` / `LINES` / `ALPHA`, `exclude = test_*.py` / `tests/*,*/migrations/*`, `ignore = docs,venv`, `output_file = radon.json` / `./out/radon.txt`, `include_ipynb = true`, `ipynb_cells = false`, `functions = true`. Exact snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on path lists ok. Bare `verbose` / `json` / `xml` / `md` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / pylama / tox / setuptools / Cargo / Pipfile / pyproject `[tool.radon]` tables / Desktop Entry / pip.conf stay out.

## [0.3.687] — 2026-11-08

View:

- **Function List** lists Pylama `pylama.ini` / `setup.cfg` / `tox.ini` / `pytest.ini` `[pylama]` / `[pylama:…]` sections and fields: `[pylama]`, `[pylama:pycodestyle]` / `[pylama:pylint]` / `[pylama:pyflakes]` / `[pylama:pydocstyle]` / `[pylama:mccabe]` / `[pylama:mypy]` / `[pylama:eradicate]` / `[pylama:isort]`, `[pylama:*/tests.py]` / `[pylama:*/pylama/main.py]` / `[pylama:src/*.py]`, `linters = pylint,mccabe` / `pycodestyle,pyflakes,isort`, `format = pylint` / `pycodestyle` / `pydocstyle` / `parsable` / `json`, `skip = */.tox/*,*/.env/*` / `*/messages.py` / `1`, `concurrent = True` / `false`, `abspath = yes`, `max_complexity = 10` / `0`. Exact snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on linter lists ok. Bare `ignore` / `select` / `max_line_length` / `verbose` / `options` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / hypothesis / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pylama]` tables / Desktop Entry / pip.conf stay out.

## [0.3.686] — 2026-11-08

View:

- **Function List** lists Hypothesis `pytest.ini` / `setup.cfg` / `tox.ini` `[hypothesis]` / `[tool:hypothesis]` sections and fields: `[hypothesis]`, `[tool:hypothesis]`, `max_examples = 100` / `1000`, `derandomize = True` / `false`, `deadline = 500` / `500.0` / `none`, `suppress_health_check = too_slow,filter_too_much` / `data_too_large` / `return_value` / `large_base_example` / `not_a_test_method` / `function_scoped_fixture` / `differing_executors`, `phases = explicit,reuse,generate,target,shrink` / `explain`, `verbosity = verbose` / `quiet` / `normal` / `debug`, `database = .hypothesis/examples` / `none`, `print_blob = True`, `report_multiple_bugs = false`, `stateful_step_count = 50`, `backend = hypothesis.internal.conjecture`. Exact snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on lists ok. Bare `verbose` / `seed` / `timeout` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / sqlfluff / tox / setuptools / Cargo / Pipfile / pyproject `[tool.hypothesis]` tables / Desktop Entry / pip.conf stay out.

## [0.3.685] — 2026-11-08

View:

- **Function List** lists SQLFluff `.sqlfluff` / `setup.cfg` / `tox.ini` `[sqlfluff]` / `[sqlfluff:…]` sections and fields: `[sqlfluff]`, `[sqlfluff:indentation]`, `[sqlfluff:rules]`, `[sqlfluff:rules:L010]`, `[sqlfluff:templater]` / `[sqlfluff:templater:jinja]` / `[sqlfluff:templater:dbt]`, `[sqlfluff:layout]` / `[sqlfluff:layout:type:comma]`, `dialect = postgres` / `ansi` / `bigquery` / `snowflake`, `templater = jinja` / `dbt` / `raw` / `python` / `placeholder`, `rules = L001,L010` / `ALL`, `exclude_rules = L031,L011`, `max_line_length = 120`, `large_file_skip_byte_limit = 20000`, `large_file_skip_char_limit = 10000`, `runaway_limit = 10`, `processes = 4` / `-1`, `disable_noqa = True`, `nocolor = false`, `fix_even_unparsable = True`, `warn_unused_ignores = yes`, `ignore = parsing,templating` / `lexing`, `sql_file_exts = .sql,.sql.j2`, `output_line_length = 80`, `indent_unit = space` / `tab`, `tab_space_size = 4`, `indented_joins = False`, `indented_ctes = True`, `indented_using_on = True`, `indented_on_contents = True`, `indented_then = True`, `indented_then_contents = false`, `allow_implicit_indents = False`, `template_blocks_indent = True`, `skip_indentation_in = script_style_expression`, `spacing_before = touch` / `single` / `touch:inline`, `spacing_after = touch`, `line_position = trailing` / `leading` / `alone`, `allow_scalar = True`, `single_table_references = consistent` / `qualified` / `unqualified`, `unquoted_identifiers_policy = all` / `aliases` / `column_aliases`, `quoted_identifiers_policy = all` / `aliases` / `column_aliases` / `none`, `capitalisation_policy = consistent` / `upper` / `lower` / `pascal`, `extended_capitalisation_policy = consistent`, `preferred_quoted_literal_style = consistent` / `single_quotes` / `double_quotes`, `force_enable = True`, `group_by_and_order_by_style = consistent` / `implicit` / `explicit`, `apply_dbt_builtins = True`, `load_macros_from_path = macros`, `library_path = ./libs`. Exact snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on rule lists ok. Bare `verbose` / `encoding` / `exclude` stay out. `ignore` needs sqlfluff stages (`parsing` / `templating` / `lexing`), not flake8 codes. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / rstcheck / tox / setuptools / Cargo / Pipfile / pyproject `[tool.sqlfluff]` tables / Desktop Entry / pip.conf stay out.

## [0.3.684] — 2026-11-08

View:

- **Function List** lists rstcheck `.rstcheck.cfg` / `setup.cfg` / `tox.ini` `[rstcheck]` sections and fields: `[rstcheck]`, `report_level = WARNING` / `ERROR` / `INFO` / `SEVERE` / `NONE` / `1`…`5`, `ignore_directives = automodule,click` / `mdinclude`, `ignore_roles = src,RFC`, `ignore_substitutions = image_link`, `ignore_languages = python,cpp`, `ignore_messages = (Document or section may not begin with a transition\\.$)`, `sphinx_source_dir = docs` / `./docs` / `src/docs`. Exact snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Trailing commas on ignore lists ok. Bare `verbose` / `quiet` / `ignore` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / doc8 / tox / setuptools / Cargo / Pipfile / pyproject `[tool.rstcheck]` tables / Desktop Entry / pip.conf stay out.

## [0.3.683] — 2026-11-07

View:

- **Function List** lists doc8 `setup.cfg` / `tox.ini` / `doc8.ini` / `pep8.ini` `[doc8]` / `[tool:doc8]` sections and fields: `[doc8]`, `[tool:doc8]`, `allow-long-titles = True` / `false`, `ignore-path = .tox,docs/_build` / `*.egg-info`, `ignore-path-errors = docs/odd.rst;D001;D002`, `default-extension = .rst` / `.txt`, `extension = .rst,.txt`, `file-encoding = utf-8` / `latin-1` / `cp1252`, `max-line-length = 99` / `120`, `ignore = D000,D001` / `D002`, `sphinx = True` / `yes`. Exact kebab-case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Bare `verbose` / `quiet` stay out. `ignore` needs comma-separated `D` / `D0`…`D9` / `D###` check ids (no flake8 `E`/`F`/`W` codes). flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / pydoclint / tox / setuptools / Cargo / Pipfile / pyproject `[tool.doc8]` tables / Desktop Entry / pip.conf stay out.

## [0.3.682] — 2026-11-07

View:

- **Function List** lists pydoclint `setup.cfg` / `tox.ini` `[pydoclint]` / `[tool:pydoclint]` sections and fields: `[pydoclint]`, `[tool:pydoclint]`, `arg-type-hints-in-docstring = True` / `false`, `arg-type-hints-in-signature = yes`, `check-arg-order = True`, `skip-checking-short-docstrings = false`, `skip-checking-raises = True`, `skip-checking-private-functions = yes`, `allow-init-docstring = True`, `require-return-section-when-returning-nothing = True`, `check-return-types = false`, `require-yield-section-when-yielding-nothing = True`, `check-yield-types = yes`, `ignore-underscore-only-args = True`, `ignore-private-args = false`, `ignore-special-dunder-args = True`, `check-class-attributes = yes`, `ignore-private-class-attributes = True`, `ignore-underscore-only-class-attributes = false`, `ignore-special-dunder-class-attributes = True`, `treat-property-methods-as-class-attributes = yes`, `only-attrs-with-ClassVar-are-treated-as-class-attrs = True`, `require-inline-class-var-docs = false`, `should-document-star-arguments = True`, `omit-stars-when-documenting-varargs = yes`, `check-style-mismatch = True`, `check-arg-defaults = false`, `baseline = pydoclint-baseline.txt` / `./baseline.txt`, `generate-baseline = True`, `auto-regenerate-baseline = false`, `show-filenames-in-every-violation-message = True`, `native-mode-noqa-location = docstring` / `definition`, `include-stub-files = True`. Exact kebab-case keys (ClassVar casing preserved). INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Bare `style` / `exclude` / `quiet` / `config` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / darglint / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pydoclint]` tables / Desktop Entry / pip.conf stay out.

## [0.3.681] — 2026-11-07

View:

- **Function List** lists darglint `setup.cfg` / `tox.ini` / `.darglint` `[darglint]` sections and fields: `[darglint]`, `docstring_style = google` / `sphinx` / `numpy`, `strictness = short` / `long` / `full`, `ignore = DAR101,DAR201` / `DAR1` / `DAR`, `enable = DAR103`, `ignore_regex = ^test_` / `^_(.*)`, `ignore_raise = ValueError,TypeError` / `builtins.ValueError`, `message_template = {path}:{line} {msg_id} {msg}`, `indentation = 4`. Exact snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. `ignore` / `enable` need comma-separated `DAR` / `DAR0`…`DAR9` / `DAR###` check ids (no flake8 `E`/`F`/`W` codes). Bare `message_template` without `{…}` stays out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / import-linter / tox / setuptools / Cargo / Pipfile / pyproject `[tool.darglint]` tables / Desktop Entry / pip.conf stay out.

## [0.3.680] — 2026-11-07

View:

- **Function List** lists import-linter `.importlinter` / `setup.cfg` / `tox.ini` `[importlinter]` / `[importlinter:contract:…]` sections and fields: `[importlinter]`, `[importlinter:contract:layers]` / `my-forbidden` / `acyclic_1`, `root_package = mypackage` / `mypackage.core`, `root_packages = pkg.a, pkg.b`, `include_external_packages = True` / `false`, `exclude_type_checking_imports = yes` / `0`, `unmatched_ignore_imports_alerting = error` / `warn` / `none`, `allow_indirect_imports = false`, `as_packages = true`, `exhaustive = True`, `type = layers` / `independence` / `forbidden` / `protected` / `acyclic_siblings`, `forbidden_modules = pkg.legacy` / `pkg.a, pkg.b.*`, `protected_modules = pkg.api`, `allowed_importers = pkg.cli`, `source_modules = pkg.ui`, `exhaustive_ignores = pkg.legacy`. Exact snake_case keys. INI `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. Bare `name` / `layers` / `modules` / `containers` / `ignore_imports` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / scriv / tox / setuptools / Cargo / Pipfile / pyproject `[tool.importlinter]` tables / Desktop Entry / pip.conf stay out.

## [0.3.679] — 2026-11-07

View:

- **Function List** lists scriv `setup.cfg` / `tox.ini` `[scriv]` / `[tool:scriv]` sections and fields: `[scriv]`, `[tool:scriv]`, `format = md` / `rst` / `text`, `output_file = CHANGELOG.md` / `CHANGELOG.${format}` / `docs/HISTORY.rst`, `insert_marker = scriv-insert-here` / `changelog-insert`, `end_marker = scriv-end-here`, `version = literal: 1.2.3` / `call: pkg:get_version` / `parse: setup.py:version = "([^"]+)"`, `entry_directory = changelog.d` / `news/fragments`, `main_branches = master, main`, `new_path_template = {{ version }}.md`, `entry_title_template = {{ version }} — {{ date }}` / `{% if version %}…{% endif %}`, `rst_header_chars = "#*="` / `"*=-"`, `md_header_level = 1`…`6`, `keep_fragments = false`, `skip_fragments = yes`. Exact snake_case keys. INI / TOML `key = value` or `key=value`. Optional quotes on values (needed for `#` in `rst_header_chars`). Optional `#` / `;` comment outside quotes. Free-form `categories =` blobs stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / python-semantic-release / tox / setuptools / Cargo / Pipfile / pyproject `[tool.scriv]` tables / Desktop Entry / pip.conf stay out.

## [0.3.678] — 2026-11-07

View:

- **Function List** lists python-semantic-release `setup.cfg` / `tox.ini` `[semantic_release]` / `[tool:semantic_release]` sections and fields: `[semantic_release]`, `[tool:semantic_release]`, `version_variable = pkg/__init__.py:__version__` / comma lists, `version_pattern = setup.py:version=(\d+\.\d+\.\d+)`, `version_toml = pyproject.toml:project.version` / `tool.poetry.version`, `version_source = commit` / `tag` / `tag_only`, `prerelease_tag = beta`, `tag_commit = true`, `patch_without_tag = false`, `major_on_zero = yes`, `commit_parser = angular` / `emoji` / `scipy` / `tag`, `commit_version_number = true`, `commit_subject = chore(release): {version}`, `commit_author = semantic-release <semantic-release>` / email, `changelog_file = CHANGELOG.md` / `docs/HISTORY.rst`, `changelog_placeholder = <!-- next-version -->`, `changelog_scope = true`, `changelog_capitalize = false`, `upload_to_pypi = true`, `upload_to_repository = false`, `upload_to_release = 1`, `dist_path = dist/`, `remove_dist = true`, `build_command = python -m build` / `python setup.py sdist bdist_wheel` / `false`, `hvcs = github` / `gitlab` / `gitea`, `hvcs_domain = git.example.com`, `hvcs_api_domain = git.example.com/api/v4`, `check_build_status = true`, `tag_format = v{version}` / `release-{version}`, `ignore_token_for_push = true`, `include_additional_files = README.md,CHANGELOG.md`, `pre_commit_command = ./scripts/check.sh`, `use_textual_changelog_sections = true`, `dist_glob_patterns = *.whl`, `upload_to_pypi_glob_patterns = *`. Exact snake_case keys. INI / TOML `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. `tag_format` / `commit_subject` need `{version}` (Commitizen `$version` stays out). Bare `branch` / `repository` / `commit_message` / hook arrays stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / Commitizen / tox / setuptools / Cargo / Pipfile / pyproject `[tool.semantic_release]` tables / Desktop Entry / pip.conf stay out.

## [0.3.677] — 2026-11-07

View:

- **Function List** lists Commitizen `setup.cfg` / `.cz.cfg` / `tox.ini` `[tool:commitizen]` / `[commitizen]` sections and fields: `[tool:commitizen]`, `[commitizen]`, `name = cz_conventional_commits` / `cz_jira`, `version_provider = commitizen` / `pep621` / `poetry` / `scm` / `uv` / `cargo` / `npm` / `composer`, `version_scheme = pep440` / `semver` / `semver2`, `version_files = src/__version__.py` / `setup.cfg:version`, `tag_format = $version` / `v$version` / `v${version}`, `bump_message = bump: version $current_version → $new_version`, `update_changelog_on_bump = true`, `changelog_file = CHANGELOG.md` / `docs/HISTORY.rst`, `changelog_incremental = false`, `changelog_start_rev = v1.0.0`, `changelog_merge_prerelease = true`, `annotated_tag = true`, `gpg_sign = false`, `major_version_zero = yes`, `use_shortcuts = 1`, `allow_abort = no`, `retry_after_failure = on`, `encoding = utf-8`, `prerelease_offset = 0`, `message_length_limit = 72`, `breaking_change_exclamation_in_title = true`. Exact snake_case keys. INI / TOML `key = value` or `key=value`. Optional quotes on values. Optional `#` / `;` comment. `name` needs a `cz_` plugin id. `tag_format` / `bump_message` need `$version` / `${version}` / `$current_version` / `$new_version` (unicode arrow ok). Bare `version` / `style` / hook arrays stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / Twine `.pypirc` / tox / setuptools / Cargo / Pipfile / pyproject `[tool.commitizen]` tables / Desktop Entry / pip.conf stay out.

## [0.3.676] — 2026-11-07

View:

- **Function List** lists Twine / distutils `.pypirc` sections and fields: `[distutils]`, `[pypi]`, `[testpypi]`, `[server-login]`, `index-servers = pypi testpypi` / `pypi,testpypi`, `repository = https://upload.pypi.org/legacy/` / `https://test.pypi.org/legacy/` / `http://pypi.example.com/`, `username = __token__`. Exact stock section names (no free-form private-index headers). INI `key = value` or `key=value`. Optional `#` / `;` comment. `repository` needs an `http://` / `https://` URL with a `.` host. `username` only matches Twine's `__token__` API-token convention. `password` / `cert` / `client_cert` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / check-wheel-contents / tox / setuptools / Cargo / Pipfile / pyproject / Desktop Entry / pip.conf / yum `.repo` stay out.

## [0.3.675] — 2026-11-07

View:

- **Function List** lists check-wheel-contents `setup.cfg` / `tox.ini` / `check-wheel-contents.cfg` `[tool:check-wheel-contents]` / `[check-wheel-contents]` sections and fields: `[tool:check-wheel-contents]`, `[check-wheel-contents]`, `select = W001,W002` / `W0` / `W`, `ignore = W009,W101`, `toplevel = foo.py,bar/`, `package = src/mypkg`, `src_dir = src`, `package_omit = .*,CVS,*.pyc,*.pyo,*.egg-info`. Exact check-wheel-contents keys (`src_dir` / `package_omit` keep underscores). INI `key = value` or `key=value`. Optional `#` / `;` comment. `select` / `ignore` need comma-separated `W` / `W0`…`W2` / `W###` check ids (no flake8 `E`/`F` codes). `package` needs a path-ish value so towncrier `package = myproject` stays out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / cibuildwheel / tox / setuptools / Cargo / Pipfile / pyproject `[tool.check-wheel-contents]` / Desktop Entry / pip.conf stay out.

## [0.3.674] — 2026-11-07

View:

- **Function List** lists cibuildwheel `setup.cfg` / `tox.ini` `[tool:cibuildwheel]` / `[cibuildwheel]` / platform or selector overrides (`[tool:cibuildwheel:linux]`, `[tool:cibuildwheel:cp39-manylinux_x86_64]`) and fields: `[tool:cibuildwheel]`, `[cibuildwheel]`, `[tool:cibuildwheel:linux]` / `windows` / `macos` / `pyodide` / `ios` / `android`, `[tool:cibuildwheel:cp39-manylinux_x86_64]`, `before-all = yum install -y libffi-devel`, `before-build = pip install -r req.txt`, `before-test = pip install pytest`, `repair-wheel-command = auditwheel repair -w {dest_dir} {wheel}`, `test-command = pytest {project}/tests`, `test-requires = pytest`, `test-extras = test`, `test-groups = test`, `test-skip = pp*`, `build-frontend = build` / `pip` / `build[uv]`, `build-verbosity = 1` / `-1`, `environment-pass = CFLAGS CXXFLAGS`, `container-engine = docker` / `podman`, `dependency-versions = pinned` / `latest` / `file:constraints.txt`, `config-settings = --build-option=--use-cython`, `pyodide-version = 0.26.0`, `xbuild-tools = cmake ninja`, `manylinux-x86_64-image = manylinux2014` / `manylinux_2_28` / `quay.io/pypa/…`, `musllinux-x86_64-image = musllinux_1_2`. Exact kebab-case keys. INI `key = value` or `key=value`. Optional `#` / `;` comment. Bare `build` / `skip` / `archs` / `environment` / `enable` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / zest.releaser / tox / setuptools / Cargo / Pipfile / pyproject `[tool.cibuildwheel]` / Desktop Entry / pip.conf stay out.

## [0.3.673] — 2026-11-07

View:

- **Function List** lists zest.releaser `setup.cfg` / `.pypirc` `[zest.releaser]` sections and fields: `[zest.releaser]`, `create-wheel = yes` / `false`, `upload-pypi = no`, `release = True`, `no-input = on`, `register = OFF`, `push-changes = 1`, `less-zeroes = true`, `tag-signing = yes`, `run-pre-commit = false`, `extra-message = [ci skip]`, `prefix-message = [TAG]`, `development-marker = .dev0`, `tag-format = v{version}` / `%(version)s`, `tag-message = Tagging {version}`, `date-format = %%Y-%%m-%%d`, `history-file = CHANGES.rst` / `docs/HISTORY.md`, `history_format = md` / `rst`, `version-levels = 3`. Exact zest.releaser keys (mostly kebab-case; `history_format` keeps its underscore). INI `key = value` or `key=value`. Optional `#` / `;` comment. Bool values accept yes/true/on/1 / no/false/off/0 (case-insensitive). `tag-format` / `tag-message` need `{version}` or `%(version)s`; `date-format` needs `%`; `history-file` needs a changelog-ish path; `history_format` is `md` / `rst`; `version-levels` is 0–10; `development-marker` needs a leading `.` or `dev`. Bare `name` / `version` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / towncrier / tox / setuptools / Cargo / Pipfile / pyproject `[tool.zest-releaser]` / Desktop Entry / pip.conf stay out.

## [0.3.672] — 2026-11-07

View:

- **Function List** lists towncrier `setup.cfg` / `tox.ini` `[tool:towncrier]` / `[towncrier]` sections and fields: `[tool:towncrier]`, `[towncrier]`, `package = myproject`, `package_dir = src`, `filename = NEWS.rst` / `CHANGES.md` / `docs/CHANGELOG.txt`, `directory = newsfragments` / `changelog.d` / `changes`, `title_format = {name} {version} ({project_date})`, `issue_format = \`{issue} <https://…/{issue}>\`_`, `underlines = =-~`, `template = towncrier:default.rst` / `templates/release.rst`, `start_string = .. towncrier release notes start`, `orphan_prefix = +`, `all_bullets = true`, `single_file = False`, `wrap = yes`. Exact towncrier keys (snake_case). INI `key = value` or `key=value`. Optional `#` / `;` comment. `title_format` needs `{version}` / `{name}` / `{project_date}`; `issue_format` needs `{issue}`; `filename` needs changelog-ish basename or `.rst`/`.md`/`.txt`; `directory` needs a news/changelog cue; `start_string` needs `towncrier` or a leading `.. `; bare `name` / `version` stay out. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / setuptools_scm / tox / setuptools / Cargo / Pipfile / pyproject `[tool.towncrier]` / Desktop Entry / pip.conf stay out.

## [0.3.671] — 2026-11-07

View:

- **Function List** lists setuptools_scm `setup.cfg` `[tool:setuptools_scm]` / `[setuptools_scm]` sections and fields: `[tool:setuptools_scm]`, `[setuptools_scm]`, `version_scheme = guess-next-dev` / `post-release` / `python-simplified-semver` / `release-branch-semver` / `no-guess-dev` / `calver-by-date`, `local_scheme = node-and-date` / `node-and-timestamp` / `dirty-tag` / `no-local-version`, `write_to = pkg/_version.py`, `version_file = src/pkg/_version.py`, `write_to_template = version = '{version}'`, `version_file_template = __version__ = %s`, `fallback_version = 0.0.0`, `tag_regex = …(?P<version>…)…`, `git_describe_command = git describe …`, `parentdir_prefix_version = myproject-`, `relative_to = setup.py` / `pyproject.toml`, `root = .` / `..`, `normalize = true`, `search_parent_directories = False`. Exact snake_case keys. INI `key = value` or `key=value`. Optional `#` / `;` comment. Stock `version_scheme` / `local_scheme` presets only. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / versioneer / tox / setuptools / Cargo / Pipfile / pyproject `[tool.setuptools_scm]` / Desktop Entry / pip.conf stay out.

## [0.3.670] — 2026-11-07

View:

- **Function List** lists versioneer `setup.cfg` `[versioneer]` sections and fields: `[versioneer]`, `VCS = git` / `mercurial` / `darcs` / `fossil` / `bazaar`, `style = pep440` / `pep440-branch` / `pep440-pre` / `pep440-post` / `pep440-old` / `git-describe` / `git-describe-long`, `versionfile_source = src/pkg/_version.py`, `versionfile_build = pkg/_version.py`, `tag_prefix =` / `tag_prefix = v`, `parentdir_prefix = myproject-`, `verbose = True` / `false`. Exact versioneer keys (`VCS` / `style` casing; snake_case path keys). INI `key = value` or `key=value`. Optional `#` / `;` comment. `tag_prefix` may be empty. `style = sphinx` stays on docformatter. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / bumpversion / tox / setuptools / Cargo / Pipfile / pyproject `[tool.versioneer]` / Desktop Entry / pip.conf stay out.

## [0.3.669] — 2026-11-07

View:

- **Function List** lists bumpversion / bump2version `setup.cfg` / `.bumpversion.cfg` `[bumpversion]` / `[bumpversion:file:…]` / `[bumpversion:part:…]` sections and fields: `[bumpversion]`, `[bumpversion:file:setup.py]` / `[bumpversion:file:src/pkg/__init__.py]`, `[bumpversion:part:release]`, `current_version = 1.2.3`, `parse = (?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)`, `serialize = {major}.{minor}.{patch}`, `search = version='{current_version}'`, `replace = version='{new_version}'`, `tag_name = v{new_version}`, `tag_message = Bump version: {current_version} → {new_version}`, `allow_dirty = False` / `true`, `sign_tags = True` / `0`, `optional_value = gamma`, `first_value = alpha`. Exact bumpversion keys (snake_case). INI `key = value` or `key=value`. Optional `#` / `;` comment. Bare `commit` / `tag` / `message` / `files` / `values` stay out. `parse` needs `(?P<…>` or `{…}`; `serialize` / `search` / `replace` / `tag_name` / `tag_message` need `{…}`; `current_version` needs a dotted version token. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / check-manifest / tox / setuptools / Cargo / Pipfile / pyproject `[tool.bumpversion]` / Desktop Entry / pip.conf stay out.

## [0.3.668] — 2026-11-07

View:

- **Function List** lists check-manifest `setup.cfg` / `tox.ini` `[check-manifest]` sections and fields: `[check-manifest]`, `ignore = .travis.yml` / `.github/*,*.mo` / `*.egg-info/*`, `ignore-default-rules = true` / `false`, `ignore-bad-ideas = *.c` / `generated/*.py,*.so`. Exact check-manifest keys (kebab-case). INI `key = value` or `key=value`. Optional `#` / `;` comment. `ignore` / `ignore-bad-ideas` values need a path/glob token (`.` / `/` / `*` / `?`) so flake8 error-code `ignore = E203,W503` stays on flake8. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / interrogate / tox / setuptools / Cargo / Pipfile / pyproject `[tool.check-manifest]` / Desktop Entry / pip.conf stay out.

## [0.3.667] — 2026-11-07

View:

- **Function List** lists interrogate `setup.cfg` / `tox.ini` `[tool:interrogate]` / `[interrogate]` sections and fields: `[tool:interrogate]`, `[interrogate]`, `ignore-init-method = true`, `ignore-init-module = false`, `ignore-magic = true`, `ignore-semiprivate` / `ignore-private` / `ignore-property-decorators` / `ignore-module` / `ignore-nested-functions` / `ignore-nested-classes` / `ignore-setters` / `ignore-overloaded-functions`, `fail-under = 80` / `95.0`, `ignore-regex = ^get$,^mock_.*`, `whitelist-regex = ^test_`, `omit-covered-files = true`, `generate-badge = .` / `docs/_static`, `badge-format = svg` / `png`. Exact interrogate keys (kebab-case). INI `key = value` or `key=value`. Optional `#` / `;` comment. Bare `exclude` / `verbose` / `quiet` / `color` / `ext` / `style` stay out (flake8 / shared / docformatter). flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / vulture / tox / setuptools / Cargo / Pipfile / pyproject `[tool.interrogate]` / Desktop Entry / pip.conf stay out.

## [0.3.666] — 2026-11-07

View:

- **Function List** lists vulture `setup.cfg` / `tox.ini` `[vulture]` sections and fields: `[vulture]`, `ignore_decorators = @app.route,@require_*` / `ignore-decorators`, `ignore_names = visit_*,do_*` / `ignore-names`, `make_whitelist = true` / `make-whitelist`, `min_confidence = 80` / `min-confidence`, `sort_by_size = true` / `sort-by-size`, `paths = .` / `src,tests`. Exact vulture keys (snake_case + kebab aliases). INI `key = value` or `key=value`. Optional `#` / `;` comment. Bare `exclude` stays out (flake8). flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / docformatter / tox / setuptools / Cargo / Pipfile / pyproject `[tool.vulture]` / Desktop Entry / pip.conf stay out.

## [0.3.665] — 2026-11-07

View:

- **Function List** lists docformatter `setup.cfg` / `tox.ini` `[docformatter]` sections and fields: `[docformatter]`, `wrap-summaries = 82`, `wrap-descriptions = 81`, `pre-summary-newline = true`, `pre-summary-space = true`, `make-summary-multi-line = false`, `close-quotes-on-newline = true`, `force-wrap = true`, `blank = true`, `black = true`, `non-strict = false`, `style = sphinx` / `epytext` / `numpy` / `google`, `tab-width = 4` / `tab_width`, `non-cap = http,https`, `rest-section-adorns = …`, `docstring-length = 10 200`. Exact docformatter keys (kebab-case + `tab_width`). INI `key = value` or `key=value`. Optional `#` / `;` comment. `recursive` / bare `exclude` / `check` / `diff` stay out (autopep8 / flake8 / black). flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / codespell / tox / setuptools / Cargo / Pipfile / pyproject `[tool.docformatter]` / Desktop Entry / pip.conf stay out.

## [0.3.664] — 2026-11-07

View:

- **Function List** lists codespell `setup.cfg` / `tox.ini` / `.codespellrc` `[codespell]` sections and fields: `[codespell]`, `ignore-words-list = foobar,bizbaz` / `ignore_words_list`, `ignore-words` / `ignore_words`, `quiet-level = 3` / `quiet_level`, `check-filenames` / `check_filenames`, `check-hidden` / `check_hidden`, `uri-ignore-words-list` / `uri_ignore_words_list`, `ignore-regex` / `ignore_regex`, `write-changes` / `write_changes`, `exclude-file` / `exclude_file`, `builtin = clear,rare,code`, `interactive = 3`, `summary`, `skip`, `count`. Exact codespell keys (kebab + snake aliases). INI `key = value` or `key=value`. Optional `#` / `;` comment. Bare `exclude` stays out (flake8). flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / bandit / tox / setuptools / Cargo / Pipfile / pyproject `[tool.codespell]` / Desktop Entry / pip.conf stay out.

## [0.3.663] — 2026-11-07

View:

- **Function List** lists Bandit `setup.cfg` / `tox.ini` / `.bandit` `[bandit]` sections and fields: `[bandit]`, `skips = B101,B601`, `tests = B201,B301`, `exclude_dirs = /tests,/path/to/other` / `exclude-dirs`. Exact bandit keys (snake_case + kebab `exclude-dirs`). INI `key = value` or `key=value`. Optional `#` / `;` comment. Bare `exclude` stays out (flake8). flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / pydocstyle / tox / setuptools / Cargo / Pipfile / pyproject `[tool.bandit]` / Desktop Entry / pip.conf stay out.

## [0.3.662] — 2026-11-07

View:

- **Function List** lists pydocstyle `setup.cfg` / `tox.ini` / `.pydocstyle` `[pydocstyle]` / legacy `[pep257]` sections and fields: `[pydocstyle]`, `[pep257]`, `convention = pep257` / `google` / `numpy`, `add_ignore = D100,D104` / `add-ignore`, `add_select = D212` / `add-select`, `ignore = D203`, `select = D` / `D1,D2`, `match = (?!test_).*\.py`, `match_dir` / `match-dir`, `ignore_decorators` / `ignore-decorators = property|cached_property`. Exact pydocstyle keys (snake_case + kebab aliases). INI `key = value` or `key=value`. Optional `#` / `;` comment. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / yapf / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pydocstyle]` / Desktop Entry / pip.conf stay out.

## [0.3.661] — 2026-11-07

View:

- **Function List** lists YAPF `setup.cfg` / `tox.ini` `[yapf]` sections and fields: `[yapf]`, `based_on_style = pep8` / `google` / `chromium` / `facebook` / `yapf`, `COLUMN_LIMIT = 100`, `INDENT_WIDTH = 4`, `CONTINUATION_INDENT_WIDTH = 4`, `SPACES_BEFORE_COMMENT = 2`, `CONTINUATION_ALIGN_STYLE = SPACE` / `FIXED` / `VALIGN-RIGHT`, `SPLIT_BEFORE_LOGICAL_OPERATOR = true`, `SPLIT_BEFORE_NAMED_ASSIGNS = true`, `DEDENT_CLOSING_BRACKETS = true`, `COALESCE_BRACKETS = true`, `ALLOW_MULTILINE_LAMBDAS = true`, `BLANK_LINE_BEFORE_NESTED_CLASS_OR_DEF = true`, `USE_TABS = false`, and other stock UPPER_SNAKE knobs. Exact YAPF keys (`based_on_style` + UPPER_SNAKE). INI `key = value` or `key=value`. Optional `#` / `;` comment. `.style.yapf` `[style]` section headers stay out (too generic); distinctive keys still list. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / black / tox / setuptools / Cargo / Pipfile / pyproject `[tool.yapf]` / Desktop Entry / pip.conf stay out.

## [0.3.660] — 2026-11-07

View:

- **Function List** lists Black `setup.cfg` / `tox.ini` `[black]` sections and fields: `[black]`, `line-length = 88`, `target-version = py39` / `py37,py38,py39`, `skip-string-normalization = true`, `skip-magic-trailing-comma = true`, `preview = true`, `check = true`, `diff = true`, `color = true`, `fast = true`, `quiet = true`, `verbose = true`, `include = \.pyi?$`, `exclude = /(\.git|\.venv)/`, `extend-exclude = docs`, `force-exclude = migrations`, `required-version = 24.3.0`, `workers = 4`. Exact Black keys (kebab-case). INI `key = value` or `key=value`. Optional `#` / `;` comment. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / autopep8 / tox / setuptools / Cargo / Pipfile / pyproject `[tool.black]` / Desktop Entry / pip.conf stay out.

## [0.3.659] — 2026-11-07

View:

- **Function List** lists autopep8 `setup.cfg` / `tox.ini` / `.pep8` `[autopep8]` sections and fields: `[autopep8]`, `aggressive = 1`, `experimental = True`, `max_line_length = 120`, `ignore = E226,E24`, `select = E,W`, `hang_closing = True`, `recursive = True`, `jobs = 4`, `verbose = 1`, `diff = True`, `exit_code = True`, `pep8_passes = -1`, `line_range = 1 10`, `global_config = /etc/autopep8.cfg`, `ignore_local_config = True`. Exact autopep8 keys (snake_case). `indent_size` stays out so EditorConfig key lines are not claimed. INI `key = value` or `key=value`. Optional `#` / `;` comment. flake8 / mypy / pytest / isort / coverage / pylint / pycodestyle / tox / setuptools / Cargo / Pipfile / pyproject `[tool.autopep8]` / Desktop Entry / pip.conf stay out.

## [0.3.658] — 2026-11-07

View:

- **Function List** lists pycodestyle `.pycodestyle` / `setup.cfg` / `tox.ini` `[pycodestyle]` / legacy `[pep8]` sections and fields: `[pycodestyle]`, `[pep8]`, `max-line-length = 88`, `max-doc-length = 72`, `indent-size = 4`, `ignore = E203,W503`, `select = E,F,W`, `exclude = .git,.venv,build,dist`, `filename = *.py`, `format = default` / `pylint` / custom `%(path)s:…`, `hang-closing = true`, `statistics = true`, `count = true`, `show-source = true`, `show-pep8 = true`, `first = true`, `diff = true`, `benchmark = true`, `quiet = 1`, `verbose = 1`. Exact pycodestyle keys (kebab-case). INI `key = value` or `key=value`. Optional `#` / `;` comment. flake8 / mypy / pytest / isort / coverage / pylint / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pycodestyle]` / Desktop Entry / pip.conf stay out.

## [0.3.657] — 2026-11-07

View:

- **Function List** lists pylint `.pylintrc` / `pylintrc` / `setup.cfg` `[pylint.…]` sections and fields: `[MAIN]` / `[MASTER]` / `[MESSAGES CONTROL]` / `[REPORTS]` / `[BASIC]` / `[REFACTORING]` / `[FORMAT]` / `[LOGGING]` / `[MISCELLANEOUS]` / `[SIMILARITIES]` / `[SPELLING]` / `[TYPECHECK]` / `[CLASSES]` / `[DESIGN]` / `[IMPORTS]` / `[EXCEPTIONS]` / `[VARIABLES]` / `[METHOD_ARGS]` / `[STRING]`, `[pylint.main]` / `[pylint.master]` / `[pylint.messages_control]` / `[pylint.reports]` / `[pylint.basic]` / `[pylint.format]` / `[pylint.design]` / `[pylint.typecheck]` / …, `fail-under = 10`, `jobs = 0`, `disable = C0114,C0115`, `enable = useless-suppression`, `max-line-length = 100`, `good-names = i,j,k,ex,Run,_`, `max-args = 5`, `max-locals = 15`, `max-returns = 6`, `max-branches = 12`, `max-statements = 50`, `max-parents = 7`, `max-attributes = 7`, `min-public-methods = 2`, `max-public-methods = 20`, `max-bool-expr = 5`, `max-nested-blocks = 5`, `output-format = text` / `colorized` / `json` / `parseable` / `msvs` / `github`, `reports = no`, `score = yes`, `suggestion-mode = yes`, `persistent = yes`, `py-version = 3.11`, `ignore = CVS`, `ignore-paths = …`, `ignore-patterns = …`, `load-plugins = …`, `argument-naming-style = snake_case`, `function-naming-style = snake_case`, `class-naming-style = PascalCase`, `const-naming-style = UPPER_CASE`, `logging-format-style = new` / `old`, `confidence = HIGH,CONTROL_FLOW`, `notes = FIXME,TODO`, `min-similarity-lines = 4`, `ignore-comments = yes`, `ignore-docstrings = yes`, `ignore-imports = yes`, `ignore-signatures = yes`, `docstring-min-length = -1`, `indent-string = '    '`, `indent-after-paren = 4`, `max-module-lines = 1000`, `expected-line-ending-format = LF` / empty, `single-line-if-stmt = no`, `overgeneral-exceptions = …`, `ignored-classes = …`, `dummy-variables-rgx = …`, `init-import = no`, `allow-wildcard-with-all = no`, `recursive = no`, `unsafe-load-any-extension = no`, `limit-inference-results = 100`, `timeout = 0`, `missing-member-hint = yes`, `spelling-dict =` (empty ok), `max-spelling-suggestions = 4`, `msg-template = …`. Exact pylint keys (kebab-case). INI `key = value` or `key=value`. Optional `#` / `;` comment. flake8 / mypy / pytest / isort / coverage / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pylint]` / Desktop Entry / pip.conf stay out.

## [0.3.656] — 2026-11-06

View:

- **Function List** lists coverage.py `.coveragerc` / `setup.cfg` `[coverage:…]` sections and fields: `[run]` / `[report]` / `[html]` / `[xml]` / `[json]` / `[lcov]` / `[paths]`, `[coverage:run]` / `[coverage:report]` / `[coverage:html]` / `[coverage:xml]` / `[coverage:json]` / `[coverage:lcov]` / `[coverage:paths]`, `branch = true`, `parallel = true`, `source = src`, `source_pkgs = mypkg`, `omit = */tests/*`, `include = src/*`, `concurrency = multiprocessing` / `thread` / `gevent` / `eventlet` / `greenlet` / `multiprocess` / `threadsafe`, `data_file = .coverage`, `plugins = …`, `cover_pylib = false`, `relative_files = true`, `context = unit`, `dynamic_context = test_function` / `test_method`, `sigterm = true`, `command_line = -m pytest`, `debug = trace`, `disable_warnings = no-data-collected`, `note = nightly`, `fail_under = 80` / `80.5`, `show_missing = true`, `skip_covered = true`, `skip_empty = true`, `precision = 2`, `ignore_errors = true`, `exclude_lines = pragma: no cover`, `exclude_also = def __repr__`, `partial_branches = pragma: no branch`, `include_namespace_packages = true`, `sort = Name` / `Cover` / `-Name`, `directory = htmlcov`, `title = Coverage`, `extra_css = extra.css`, `show_contexts = true`, `output = coverage.xml`, `package_depth = 99`, `pretty_print = true`. Exact coverage keys (snake_case). INI `key = value` or `key=value`. Optional `#` / `;` comment. flake8 / mypy / pytest / isort / tox / setuptools / Cargo / Pipfile / pyproject `[tool.coverage]` / Desktop Entry / pip.conf stay out.

## [0.3.655] — 2026-11-06

View:

- **Function List** lists isort `.isort.cfg` / `setup.cfg` / `tox.ini` `[isort]` sections and fields: `[isort]`, `profile = black` / `django` / `pycharm` / `google` / `open_stack` / `plone` / `attrs` / `hug` / `wemake` / `appnexus` / `pep8`, `multi_line_output = 3`, `line_length = 88`, `known_first_party = mypkg`, `known_third_party = requests`, `known_standard_library = typing_extensions`, `known_local_folder = src`, `extra_standard_library = …`, `sections = FUTURE,STDLIB,THIRDPARTY,FIRSTPARTY,LOCALFOLDER`, `default_section = THIRDPARTY`, `force_single_line = true`, `force_grid_wrap = 0`, `include_trailing_comma = true`, `use_parentheses = true`, `ensure_newline_before_comments = true`, `combine_as_imports = true`, `combine_star = false`, `force_sort_within_sections = true`, `lexicographical = true`, `order_by_type = true`, `atomic = true`, `balanced_wrapping = true`, `color_output = true`, `honor_noqa = true`, `from_first = true`, `lines_after_imports = 2`, `lines_before_imports = 0`, `lines_between_types = 1`, `lines_between_sections = 1`, `skip = .git,.venv`, `skip_glob = */migrations/*`, `extend_skip = build`, `extend_skip_glob = **/generated/*`, `src_paths = src,tests`, `filter_files = true`, `skip_gitignore = true`, `reverse_relative = true`, `group_by_package = true`, `case_sensitive = true`, `remove_redundant_aliases = true`, `float_to_top = true`, `treat_all_comments_as_code = true`, `ignore_whitespace = true`, `length_sort = true`, `length_sort_straight = true`, `force_alphabetical_sort = true`, `force_alphabetical_sort_within_sections = true`, `honor_case_in_force_sorted_sections = true`, `sort_reexports_by_module = true`, `only_sections = true`, `no_lines_before = LOCALFOLDER`, `indent = '    '`, `py_version = 39`, `virtual_env = .venv`, `conda_env = base`, `quiet = true`, `verbose = true`, `overwrite_in_place = true`, `dont_rename_py_typed = true`. Exact isort keys (snake_case). INI `key = value` or `key=value`. Optional `#` / `;` comment. flake8 / mypy / pytest / tox / setuptools / Cargo / Pipfile / pyproject `[tool.isort]` / Desktop Entry / pip.conf stay out.

## [0.3.654] — 2026-11-06

View:

- **Function List** lists flake8 `.flake8` / `setup.cfg` / `tox.ini` `[flake8]` sections and fields: `[flake8]`, `max-line-length = 88`, `max-doc-length = 72`, `indent-size = 4`, `max-complexity = 10`, `ignore = E203,W503`, `select = E,F,W`, `extend-ignore = E501`, `extend-select = B950`, `exclude = .git,.venv,build,dist`, `extend-exclude = docs`, `filename = *.py`, `format = default` / `pylint`, `hang-closing = true`, `per-file-ignores = __init__.py:F401`, `statistics = true`, `count = true`, `show-source = true`, `jobs = auto` / `4`, `doctests = true`, `builtins = _`, `disable-noqa = true`, `enable-extensions = G`, `exit-zero = true`, `tee = true`, `output-file = flake8.out`, `quiet = 1`, `verbose = 1`. Exact flake8 keys (kebab-case). INI `key = value` or `key=value`. Optional `#` / `;` comment. mypy / pytest / tox / setuptools / Cargo / Pipfile / pyproject `[tool.flake8]` / Desktop Entry / pip.conf stay out.

## [0.3.653] — 2026-11-06

View:

- **Function List** lists mypy `mypy.ini` / `.mypy.ini` / `setup.cfg` `[mypy]` sections and fields: `[mypy]` / `[mypy-packagename]` / `[mypy-packagename.*]` / `[mypy-foo,bar]`, `python_version = 3.11`, `warn_return_any = True`, `warn_unused_configs = True`, `disallow_untyped_defs = True`, `disallow_incomplete_defs = True`, `disallow_untyped_calls = True`, `disallow_untyped_decorators = True`, `disallow_any_generics = True`, `disallow_subclassing_any = True`, `check_untyped_defs = True`, `ignore_missing_imports = True`, `no_implicit_optional = True`, `no_implicit_reexport = True`, `strict_optional = True`, `strict_equality = True`, `strict = True`, `local_partial_types = True`, `allow_redefinition = True`, `show_error_codes = True`, `show_column_numbers = True`, `show_error_context = True`, `pretty = True`, `color_output = True`, `error_summary = True`, `incremental = True`, `sqlite_cache = True`, `namespace_packages = True`, `explicit_package_bases = True`, `scripts_are_modules = True`, `warn_redundant_casts = True`, `warn_unused_ignores = True`, `warn_no_return = True`, `warn_unreachable = True`, `follow_imports = normal` / `silent` / `skip` / `error`, `mypy_path = src`, `cache_dir = .mypy_cache`, `files = src`, `packages = mypkg`, `modules = mypkg.cli`, `exclude = …`, `plugins = pydantic.mypy`, `platform = linux`, `python_executable = python3`, `custom_typeshed_dir = stubs`, `custom_typing_module = typing_extensions`, `always_true` / `always_false`, `enable_error_code` / `disable_error_code`, `force_union_syntax` / `force_uppercase_builtins`, `follow_imports_for_stubs` / `follow_untyped_imports`, `ignore_errors`, `allow_untyped_globals`, `show_error_end` / `show_absolute_path`, `cache_fine_grained`, `skip_version_check` / `skip_cache_mtime_checks`, `warn_incomplete_stub`. Exact mypy keys (snake_case). INI `key = value` or `key=value`. Optional `#` / `;` comment. Flake8 / pytest / tox / setuptools / Cargo / Pipfile / pyproject `[tool.mypy]` / Desktop Entry / pip.conf stay out.

## [0.3.652] — 2026-11-06

View:

- **Function List** lists pytest `pytest.ini` / `setup.cfg` `[tool:pytest]` sections and fields: `[pytest]` / `[tool:pytest]`, `minversion = 6.0`, `addopts = -ra -q`, `testpaths = tests`, `python_files = test_*.py`, `python_classes = Test*`, `python_functions = test_*`, `norecursedirs = .* build dist`, `log_cli = true`, `log_cli_level = INFO`, `asyncio_mode = auto` / `strict` / `legacy`, `junit_family = xunit2`, `junit_logging = all`, `junit_log_passing_tests = true`, `junit_duration_report = total` / `call`, `junit_suite_name = pytest`, `xfail_strict = true`, `strict_markers = true`, `strict_config = true`, `consider_namespace_packages = true`, `pythonpath = src`, `confcutdir = tests`, `cache_dir = .pytest_cache`, `importmode = importlib`, `required_plugins = pytest-cov`, `usefixtures = clean_db`, `faulthandler_timeout = 5`, `tmp_path_retention_count = 3`, `tmp_path_retention_policy = failed`, `empty_parameter_set_mark = xfail`, `console_output_style = progress`, `doctest_optionflags = …`, `doctest_encoding = utf-8`, `doctest_glob = test_*.txt`, `log_cli_format` / `log_cli_date_format` / `log_file` / `log_file_level` / `log_file_format` / `log_file_date_format` / `log_level` / `log_format` / `log_date_format`, `asyncio_default_fixture_loop_scope = function`, `disable_test_id_escaping_and_forfeit_all_rights_to_community_support = true`. Exact pytest keys (snake_case). INI `key = value` or `key=value`. Optional `#` / `;` comment. Free-form `markers` / `filterwarnings` blobs, flake8 / mypy / tox / setuptools / Cargo / Pipfile / pyproject `[tool.pytest.ini_options]` / Desktop Entry / pip.conf stay out.

## [0.3.651] — 2026-11-06

View:

- **Function List** lists setuptools `setup.cfg` sections and fields: `[metadata]` / `[options]` / `[options.packages.find]` / `[options.package_data]` / `[options.exclude_package_data]` / `[options.extras_require]` / `[options.entry_points]` / `[options.data_files]` / `[sdist]` / `[bdist_wheel]` / `[egg_info]` / `[aliases]` / `[build_ext]`, `python_requires = >=3.8`, `long_description_content_type = text/markdown`, `long_description = file: README.md`, `install_requires = requests`, `setup_requires = …`, `tests_require = …`, `include_package_data = True`, `namespace_packages = …`, `zip_safe = False`, `package_dir = src`, `py_modules = foo`, `packages = find:`, `license_files = LICENSE`, `license_file = LICENSE`, `project_urls = …`, `download_url = …`, `author_email = …`, `maintainer_email = …`, `provides_extras = …`, `universal = 1`, `py-limited-api = cp38`, `tag_build = …`, `tag_date = false`. Exact setuptools keys (snake_case; `py-limited-api` kebab). INI `key = value` or `key=value`. Optional `#` / `;` comment. Free-form `name` / `version` / `author` / `url` / `license` / `description` / `classifiers` / `keywords` / `where` / `include` / `exclude` / `scripts`, flake8 / mypy / tool:pytest / tox / Cargo / Pipfile / pyproject / Desktop Entry / pip.conf / PDM `[install]` stay out.

## [0.3.650] — 2026-11-06

View:

- **Function List** lists PDM `pdm.toml` sections and fields: `[venv]` / `[python]` / `[install]` / `[pypi]` / `[pypi.name]` / `[strategy]` / `[scripts]` / `[lock]` / `[global_project]` / `[theme]` / `[repository.name]`, `build_isolation = true`, `use_uv = false`, `check_update = true`, `request_timeout = 15`, `in_project = true`, `with_pip = false`, `backend = "virtualenv"` / `"venv"` / `"conda"`, `cache_method = "symlink"` / `"hardlink"`, `use_pyenv = true`, `use_venv = true`, `use_python_version = true`, `verify_ssl = true`, `ignore_stored_index = false`, `json_api = false`, `show_header = false`, `resolve_max_rounds = 10000`, `inherit_metadata = true`, `fallback = false`, `fallback_verbose = true`, `user_site = false`, `exclude-newer = "2024-01-01"`, `format = "pdm"` / `"pylock"`, `update = "reuse"` / `"eager"` / `"all"`, `save = "minimum"` / `"compatible"` / `"exact"` / `"wildcard"`, `prompt = "{project_name}-{python_version}"`, `install_root = "…"`, `ca_certs = "…"`, `client_cert = "…"`, `client_key = "…"`. Exact PDM keys (snake_case; `exclude-newer` kebab). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`providers = […]`) / nested maps / free-form `url` / `username` / `password` / `path` / `location` / `cache` / `parallel` / `type` / prose / wrong case / poetry / hatch / pixi / mise / supabase / samconfig / nixpacks / railway / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml / pyproject `[tool.pdm]` stay out.

## [0.3.649] — 2026-11-06

View:

- **Function List** lists Poetry local `poetry.toml` sections and fields: `[virtualenvs]` / `[virtualenvs.options]`, `[installer]`, `[solver]`, `[experimental]`, `[warnings]`, `[keyring]`, `[repositories.name]` / `[certificates.name]` / `[http-basic.name]`, `in-project = true`, `prefer-active-python = true`, `use-poetry-python = false`, `always-copy = false`, `system-site-packages = false`, `no-pip = false`, `max-workers = 4`, `no-binary = ":none:"`, `re-resolve = true`, `system-git-client = false`, `lazy-wheel = true`, `export = false`, `prompt = "{project_name}-py{python_version}"`, `cert = "path/to/ca.pem"`, `client-cert = "path/to/client.pem"`. Exact Poetry keys (kebab-case). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays / nested maps / free-form `url` / `username` / `password` / `create` / `path` / `parallel` / `enabled` / prose / wrong case / hatch / pixi / mise / supabase / samconfig / nixpacks / railway / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml / pyproject `[tool.poetry]` stay out.

## [0.3.648] — 2026-11-06

View:

- **Function List** lists Hatch `hatch.toml` sections and fields: `[envs.default]` / `[envs.hatch-test]` / `[envs.coverage]` / `[envs.docs]` / `[envs.default.scripts]` / `[envs.docs.env-vars]` / `[envs.types.extra-scripts]` / `[envs.test.overrides]` / `[[envs.test.matrix]]`, `[build]` / `[build.targets.wheel]` / `[build.targets.sdist]` / `[build.targets.wheel.force-include]` / `[build.targets.wheel.sources]` / `[build.hooks.custom]` / `[build.targets.wheel.hooks.custom]`, `[version]`, `[publish.index]`, `detached = true`, `skip-install = true`, `dev-mode = false`, `locked = true`, `lock-envs = true`, `installer = "uv"` / `"pip"`, `locker = "uv"` / `"pip"`, `lock-filename = "locks/test.lock"`, `template = "default"`, `type = "virtual"` / `"container"`, `config-path = "ruff_defaults.toml"`, `uv-path = "uv"`, `system-packages = false`, `matrix-name-format = "{variable}_{value}"`, `ignore-vcs = true`, `only-packages = true`, `dev-mode-exact = true`. Exact Hatch keys (kebab-case). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`dependencies = […]`, `extra-dependencies = […]`, `features = […]`, `platforms = […]`) / nested maps / free-form `[envs.x.scripts]` / `[envs.x.env-vars]` entries (`run = "pytest"`, `FOO = "bar"`) / `python = "3.12"` / `path = "…"` / prose / wrong case / pixi / mise / supabase / samconfig / nixpacks / railway / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.647] — 2026-11-06

View:

- **Function List** lists Pixi `pixi.toml` sections and fields: `[workspace]` / `[workspace.build-variants]` / `[workspace.conda-pypi-map]` / `[workspace.exclude-newer]` / `[workspace.exclude-newer.exemptions]` / `[activation]` / `[activation.env]` / `[pypi-dependencies]` / `[pypi-options]` / `[system-requirements]` / `[constraints]` / `[environments]` / `[environments.dev]` / `[environments.dev.dependencies]` / `[feature.cuda]` / `[feature.cuda.dependencies]` / `[feature.cuda.pypi-dependencies]` / `[feature.cuda.tasks]` / `[feature.cuda.activation]` / `[feature.cuda.target.osx-arm64.dependencies]` / `[target.linux-64.dependencies]` / `[target.win-64.activation]` / `[target.win-64.tasks]` / `[target.linux-64.activation.env]` / `[package]` / `[package.build]` / `[package.build.config]` / `[package.build.target.linux-64.config]` / `[package.run-exports]` / `[build-dependencies]` / `[host-dependencies]` / `[run-dependencies]` / `[run-constraints]`, `channel-priority = "strict"` / `"flexible"` / `"disabled"`, `solve-strategy = "highest"` / `"lowest"` / `"lowest-direct"`, `requires-pixi = ">=0.40.0"`, `exclude-newer = "2024-01-01"`, `index-strategy = "first-index"` / `"unsafe-best-match"` / `"unsafe-first-match"`, `prerelease-mode = "disallow"` / `"allow"` / `"if-necessary"` / `"explicit"` / `"if-necessary-or-explicit"`, `skip-wheel-filename-check = true`, `index-url = "https://…"`, `pypi-exclude-newer = "7 days"`, `no-build = true`, `no-binary = true`, `no-build-isolation = true`, `build-variants-files = "variants.yaml"`, `macos = "12.0"`. Exact Pixi keys (kebab-case). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`channels = […]`, `platforms = […]`, `extra-index-urls = […]`, `find-links = […]`, `preview = […]`) / nested maps / free-form `[dependencies]` / `[tasks]` / `[pypi-dependencies]` package entries (`python = "3.12"`, `start = "python main.py"`) / `port = …` / prose / wrong case / mise / supabase / samconfig / nixpacks / railway / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.


## [0.3.646] — 2026-11-06

View:

- **Function List** lists mise `mise.toml` / `.mise.toml` sections and fields: `[tools]` / `[tasks]` / `[tasks.build]` / `[settings]` / `[settings.upgrade]` / `[settings.cargo]` / `[settings.python]` / `[settings.status]` / `[settings.aqua]` / `[settings.node]` / `[settings.rust]` / `[alias]` / `[plugins]` / `[vars]` / `[hooks]` / `[shell_alias]` / `[tool_alias]` / `[task_config]` / `[deps]` / `[oci]` / `[bootstrap]` / `[history]` / `[secrets]` / `[wrappers]` / `[monorepo]` / `[dotfiles]`, `min_version = "2024.1.1"`, `experimental_monorepo_root = true`, `monorepo_root = true`, `env_file = ".env"`, `dotenv = ".env"`, `not_found_auto_install = true`, `asdf_compat = false`, `always_keep_download = false`, `activate_shims = true`, `legacy_version_file = true`, `plugin_autoupdate_last_check_duration = "7d"`, `default_config_filename = "mise.toml"`, `use_versions_host = true`, `auto_prune = false`, `binstall = true`, `uv_venv_auto = true`, `missing_tools = "if_other_versions_installed"`, `wait_for = "build"`, `depends_post = "lint"`, `run_windows = "…"`. Exact mise keys (snake_case). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`depends = […]`, `idiomatic_version_file_enable_tools = […]`) / nested maps / free-form `[tools]` / `[env]` / `[tasks]` entries (`node = "20"`, `FOO = "bar"`, `build = "cargo build"`) / `port = …` / prose / wrong case / supabase / samconfig / nixpacks / railway / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.645] — 2026-11-06

View:

- **Function List** lists Supabase CLI `supabase/config.toml` sections and fields: `[api]` / `[db]` / `[db.pooler]` / `[db.migrations]` / `[db.seed]` / `[realtime]` / `[studio]` / `[inbucket]` / `[storage]` / `[storage.image_transformation]` / `[auth]` / `[auth.email]` / `[auth.sms]` / `[auth.external.github]` / `[auth.mfa]` / `[auth.mfa.totp]` / `[analytics]` / `[experimental]` / `[edge_runtime]` / `[functions]` / `[functions.hello]` / `[remotes.production]`, `project_id = "my-app"`, `shadow_port = 54320`, `major_version = 15`, `max_rows = 1000`, `pool_mode = "transaction"` / `"session"`, `default_pool_size = 20`, `max_client_conn = 100`, `site_url = "http://127.0.0.1:3000"`, `jwt_expiry = 3600`, `enable_signup = true`, `enable_anonymous_sign_ins = false`, `double_confirm_changes = true`, `enable_confirmations = false`, `file_size_limit = "50MiB"`, `api_url = "http://127.0.0.1"`, `smtp_port = 54325`, `pop3_port = 54326`, `backend = "postgres"` / `"bigquery"`, `verify_jwt = true`, `inspector_port = 8083`, `enable_refresh_token_rotation = true`, `refresh_token_reuse_interval = 10`. Exact Supabase keys (snake_case). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`schemas = […]`, `extra_search_path = […]`, `additional_redirect_urls = […]`) / nested maps / free-form env / `port = …` (MySQL option-file collision) / bare `[build]` / `[package]` / prose / wrong case / samconfig / nixpacks / railway / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.644] — 2026-11-06

View:

- **Function List** lists AWS SAM `samconfig.toml` sections and fields: `[default]` / `[default.build]` / `[default.build.parameters]` / `[default.deploy.parameters]` / `[default.validate.parameters]` / `[default.package.parameters]` / `[default.sync.parameters]` / `[default.global.parameters]` / `[default.local_start_api.parameters]` / `[default.local_start_lambda.parameters]` / `[default.local_invoke.parameters]` / `[env.command.parameters]`, `version = 0.1`, `stack_name = "my-app"`, `resolve_s3 = true`, `s3_bucket = "…"`, `s3_prefix = "my-app"`, `region = "us-east-1"`, `profile = "default"`, `confirm_changeset = true`, `capabilities = "CAPABILITY_IAM"`, `parameter_overrides = "…"`, `template_file = "template.yaml"`, `cached = true`, `parallel = true`, `watch = true`, `lint = true`, `warm_containers = "EAGER"` / `"LAZY"`, `role_arn = "…"`, `on_failure = "DELETE"` / `"ROLLBACK"` / `"DO_NOTHING"`, `debug_port = 5890`, `host = "127.0.0.1"`, `use_container = true`, `beta_features = true`. Exact SAM keys (snake_case + `version`). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`capabilities = […]`, `image_repositories = […]`, `tags = […]`, `notification_arns = […]`, `parameter_overrides = […]`) / nested maps / free-form env / `port = …` (MySQL option-file collision) / bare `[build]` / `[package]` / prose / wrong case / nixpacks / railway / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.643] — 2026-11-06

View:

- **Function List** lists Nixpacks `nixpacks.toml` sections and fields: `[phases.setup]` / `[phases.install]` / `[phases.build]` / `[phases.name]`, `[start]` / `[variables]` / `[staticAssets]`, `buildImage = "ghcr.io/railwayapp/nixpacks:latest"`, `nixpkgsArchive = "5148520bfab61f99fd25fb9ff7bfbb50dad3c9db"`, `cmd = "yarn run start"`, `runImage = "debian:bullseye-slim"`. Exact Nixpacks keys (camelCase + `cmd` / `nixpkgsArchive`). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`providers = […]`, `cmds = […]`, `nixPkgs = […]`, `nixLibs = […]`, `nixOverlays = […]`, `aptPkgs = […]`, `dependsOn = […]`, `cacheDirectories = […]`, `onlyIncludeFiles = […]`, `paths = […]`) / free-form `[variables]` / `[staticAssets]` entries / nested maps / prose / wrong case / railway / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.642] — 2026-11-06

View:

- **Function List** lists Railway `railway.toml` sections and fields: `[build]` / `[deploy]` / `[environments.name]` / `[environments.name.build]` / `[environments.name.deploy]`, `builder = "RAILPACK"` / `"DOCKERFILE"` / `"NIXPACKS"` / `"HEROKU"` / `"PAKETO"`, `buildCommand = "npm run build"`, `dockerfilePath = "Dockerfile"`, `nixpacksConfigPath = "nixpacks.toml"`, `nixpacksVersion = "1.21.3"`, `railpackVersion = "0.7.0"`, `buildEnvironment = "V2"` / `"V3"`, `startCommand = "node index.js"`, `preDeployCommand = "…"`, `preDeployTimeoutSeconds = 300`, `numReplicas = 1`, `healthcheckPath = "/health"`, `healthcheckTimeout = 100`, `sleepApplication = false`, `runtime = "V2"` / `"LEGACY"` / `"UNSPECIFIED"`, `restartPolicyType = "ON_FAILURE"` / `"ALWAYS"` / `"NEVER"`, `restartPolicyMaxRetries = 10`, `cronSchedule = "0 * * * *"`, `region = "us-west2"`, `requiredMountPath = "/data"`, `overlapSeconds = 0`, `drainingSeconds = 10`, `ipv6EgressEnabled = false`. Exact Railway keys (camelCase). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`watchPatterns = […]`, `preDeployCommand = […]`) / nested maps (`nixpacksPlan = {…}`, `multiRegionConfig = {…}`, `limitOverride = {…}`, `registryCredentials = {…}`) / `$schema` / prose / wrong case / fly / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.


## [0.3.641] — 2026-11-06

View:

- **Function List** lists Fly.io `fly.toml` sections and fields: `[build]` / `[deploy]` / `[http_service]` / `[http_service.concurrency]` / `[vm]` / `[metrics]` / `[experimental]` / `[processes]` / `[checks.name]`, `[[http_service.checks]]` / `[[services]]` / `[[services.ports]]` / `[[services.tcp_checks]]` / `[[services.http_checks]]` / `[[vm]]` / `[[mounts]]` / `[[statics]]` / `[[files]]`, `app = "my-app"`, `primary_region = "iad"`, `kill_signal = "SIGTERM"`, `kill_timeout = "5s"`, `swap_size_mb = 512`, `console_command = "/bin/bash"`, `dockerfile = "Dockerfile"`, `ignorefile = ".dockerignore"`, `image = "…"`, `builder = "…"`, `build-target = "runner"`, `strategy = "rolling"` / `"bluegreen"` / `"canary"` / `"immediate"`, `release_command = "…"`, `wait_timeout = "5m"`, `max_unavailable = 1`, `internal_port = 8080`, `force_https = true`, `auto_stop_machines = "stop"` / `"suspend"` / `"off"` / true / false, `auto_start_machines = true`, `min_machines_running = 0`, `idle_timeout = "60s"`, `type = "requests"` / `"connections"`, `hard_limit = 25`, `soft_limit = 20`, `interval = "15s"`, `timeout = "2s"`, `grace_period = "5s"`, `method = "GET"`, `path = "/health"`, `protocol = "tcp"` / `"udp"` / `"http"` / `"https"`, `tls_skip_verify = false`, `cpu_kind = "shared"`, `cpus = 1`, `memory_mb = 256`, `memory = "1gb"`, `size = "shared-cpu-1x"`, `gpu_kind = "a10"`, `gpus = 1`, `source = "data"`, `destination = "/data"`, `initial_size = "1gb"`, `guest_path = "/app/public"`, `url_prefix = "/static"`, `local_path = "./secret"`, `raw_value = "…"`, `secret_name = "…"`, `auto_rollback = true`. Exact Fly keys (snake_case + `build-target`). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware). Arrays (`processes = […]`, `handlers = […]`, `buildpacks = […]`) / nested maps (`args = {…}`, `headers = {…}`) / `[env]` / free-form env keys / `port = …` (MySQL option-file collision) / prose / wrong case / netlify / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.640] — 2026-11-06

View:

- **Function List** lists Netlify `netlify.toml` sections and fields: `[build]` / `[build.environment]` / `[build.processing]` / `[build.processing.css|js|html|images]`, `[dev]` / `[functions]` / `[functions.name]`, `[context.production]` / `[context.deploy-preview]` / `[context.branch-deploy]` / `[context.name]` / `[context.name.environment]` / `[context.name.processing…]`, `[[redirects]]` / `[[headers]]` / `[[plugins]]` / `[[edge_functions]]`, `command = "npm run build"`, `publish = "dist"`, `functions = "netlify/functions"`, `edge_functions = "netlify/edge-functions"`, `base = "."`, `ignore = "…"`, `from = "/*"`, `to = "/index.html"`, `status = 200`, `force = false`, `for = "/*"`, `package = "@netlify/plugin-lighthouse"`, `directory = "netlify/functions"`, `node_bundler = "esbuild"` / `"zisi"` / `"nft"`, `skip_processing = false`, `bundle = true`, `minify = true`, `pretty_urls = true`, `compress = true`, `framework = "#custom"`, `targetPort = 3000`, `autoLaunch = false`, `functionsPort = 34567`, `path = "/api/*"`, `function = "api"`. Exact Netlify keys (snake_case + known camelCase). TOML `key = value` with spaces around `=`. Optional `#` comment (quote-aware so `"#custom"` works). Arrays (`included_files = […]`, `external_node_modules = […]`) / nested maps / `port = …` (MySQL option-file collision) / prose / wrong case / wrangler / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.639] — 2026-11-06

View:

- **Function List** lists Cloudflare Workers Wrangler `wrangler.toml` sections and fields: `[env.name]` / `[dev]` / `[triggers]` / `[vars]` / `[assets]` / `[placement]` / `[observability]` / `[limits]` / `[durable_objects]` / `[queues]` / `[browser]` / `[ai]` / `[site]`, `[[kv_namespaces]]` / `[[r2_buckets]]` / `[[d1_databases]]` / `[[vectorize]]` / `[[services]]` / `[[durable_objects.bindings]]` / `[[queues.producers]]` / `[[queues.consumers]]` / `[[tail_consumers]]` / `[[workflows]]` / `[[analytics_engine_datasets]]` / `[[rules]]` / `[[migrations]]` / `[[hyperdrive]]` / `[[send_email]]` / `[[mtls_certificates]]` / `[[ai_search_namespaces]]` / `[[secrets_store_secrets]]`, `name = "my-worker"`, `main = "src/index.ts"`, `compatibility_date = "2024-01-01"`, `account_id = "…"`, `workers_dev = true`, `preview_urls = false`, `no_bundle = false`, `find_additional_modules = true`, `preserve_file_names = false`, `keep_names = true`, `logpush = false`, `keep_vars = true`, `send_metrics = false`, `jsx_factory = "React.createElement"`, `jsx_fragment = "React.Fragment"`, `usage_model = "bundled"` / `"unbound"`, `base_dir = "src"`, `tsconfig = "./tsconfig.json"`, `route = "example.com/*"`. Exact snake_case Wrangler keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Arrays (`compatibility_flags = […]`, `routes = […]`, `crons = […]`) / nested maps / bare `minify` / prose / wrong case / dioxus / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.638] — 2026-11-06

View:

- **Function List** lists Dioxus CLI `Dioxus.toml` sections and fields: `[application]` / `[web.app]` / `[web.watcher]` / `[web.resource]` / `[web.resource.dev]`, `[[web.proxy]]` / `[[web.https]]`, `asset_dir = "public"`, `sub_package = "my-crate"`, `out_dir = "dist"`, `public_dir = "public"`, `default_platform = "web"` / `"desktop"` / `"mobile"`, `base_path = "my_application"`, `reload_html = true`, `index_on_404 = true`, `backend = "http://localhost:8000/api/"`, `mkcert = true`, `key_path = "…"`, `cert_path = "…"`. Exact snake_case Dioxus keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Arrays (`watch_path = […]`, `style = […]`, `script = […]`, `icon = […]`) / nested maps / bare `name` / `title` / `enabled` / prose / wrong case / uniffi / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.637] — 2026-11-06

View:

- **Function List** lists Mozilla UniFFI `uniffi.toml` sections and fields: `[bindings.swift]` / `[bindings.kotlin]` / `[bindings.python]` / `[bindings.ruby]`, `cdylib_name = "…"`, `cdylib_path = "…"`, `module_name = "…"`, `ffi_module_name = "…"`, `ffi_module_filename = "…"`, `package_name = "…"`, `generate_module_map = true`, `omit_argument_labels = false`, `generate_immutable_records = false`, `omit_localized_error_conformance = false`, `generate_case_iterable_conformance = false`, `generate_codable_conformance = false`, `omit_checksums = false`, `android = false`, `android_cleaner = true`, `disable_java_cleaner = false`, `kotlin_target_version = "1.9.0"`, `type_name = "URL"`, `into_custom = "…"`, `from_custom = "…"`, `lift = "…"`, `lower = "…"`. Exact snake_case UniFFI keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Arrays / nested maps (`mutable_records = […]`, `link_frameworks = […]`, `custom_types` / `external_packages` / `rename` / `exclude` / `[bindings.*.custom_types.*]`) / prose / wrong case / cbindgen / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.636] — 2026-11-06

View:

- **Function List** lists cbindgen `cbindgen.toml` sections and fields: `[defines]` / `[export]` / `[export.rename]` / `[export.body]` / `[export.mangle]` / `[export.pre_body]` / `[fn]` / `[struct]` / `[enum]` / `[const]` / `[macro_expansion]` / `[parse]` / `[parse.expand]` / `[layout]` / `[ptr]` / `[cython]` / `[cython.cimports]`, `language = "C"` / `"C++"` / `"Cython"`, `include_version = false`, `pragma_once = true`, `cpp_compat = true`, `no_includes = false`, `braces = "SameLine"` / `"NextLine"`, `line_length = 100`, `tab_width = 2`, `documentation = true`, `documentation_style = "auto"` / `"c"` / `"c99"` / `"c++"` / `"doxy"`, `documentation_length = "full"` / `"short"`, `line_endings = "LF"` / `"CR"` / `"CRLF"` / `"Native"`, `style = "both"` / `"tag"` / `"type"`, `sort_by = "Name"` / `"None"`, `usize_is_size_t = true`, `include_guard = "…"`, `namespace = "…"`, `header = "…"`, `trailer = "…"`, `autogen_warning = "…"`, `after_includes = "…"`, `renaming_overrides_prefixing = false`, `rename_args = "None"`, `args = "auto"`, `rename_fields = "None"`, `rename_variants = "None"`, `rename_associated_constant = "None"`, `derive_constructor = false`, `derive_eq = false`, `enum_class = true`, `add_sentinel = false`, `prefix_with_name = false`, `allow_static_const = true`, `allow_constexpr = false`, `bitflags = false`, `parse_deps = false`, `clean = false`, `all_features = false`, `default_features = true`, `packed = "…"`, `aligned_n = "…"`, `non_null_attribute = "…"`, `swift_name_macro = "…"`, `associated_constants_in_body = false`. Exact snake_case cbindgen keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Arrays (`namespaces = […]`, `includes = […]`, `exclude = […]`, `features = […]`, `crates = […]`) / nested rename maps / prose / wrong case / trunk / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.635] — 2026-11-06

View:

- **Function List** lists Trunk `trunk.toml` sections and fields: `[build]` / `[watch]` / `[serve]` / `[clean]` / `[tools]`, `target = "index.html"`, `html_output = "index.html"`, `dist = "dist"`, `public_url = "/"`, `filehash = true`, `inject_scripts = true`, `no_sri = false`, `open = false`, `no_autoreload = false`, `no_error_reporting = false`, `ws_protocol = "ws"` / `"wss"`, `tls_key_path = "…"`, `tls_cert_path = "…"`, `prefer_address = "127.0.0.1"`, `cargo = false`, `sass = "1.69.5"`, `wasm_bindgen = "0.2.87"`, `wasm_opt = "version_116"`, `tailwindcss = "3.3.5"`. Exact snake_case Trunk keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Arrays (`watch = […]`, `ignore = […]`, `addresses = […]`) / `port = …` (MySQL option-file collision) / `[[hooks]]` / prose / wrong case / mutants / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.634] — 2026-11-06

View:

- **Function List** lists cargo-mutants `.cargo/mutants.toml` / `mutants.toml` fields: `timeout_multiplier = 2.0`, `build_timeout_multiplier = 1.5`, `minimum_test_timeout = 5.0`, `cap_lints = true`, `copy_vcs = false`, `copy_target = true`, `gitignore = true`, `all_features = true`, `no_default_features = false`, `skip_calls_defaults = true`, `test_workspace = true`, `test_tool = "cargo"` / `"nextest"`, `profile = "mutants"`, `output = "mutants.out"`. Exact snake_case cargo-mutants keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Arrays (`error_values = […]`, `exclude_globs = […]`, `examine_re = […]`, `skip_calls = […]`, `features = […]`, `test_package = […]`, `additional_cargo_args = […]`) / sections / prose / wrong case / audit / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.633] — 2026-11-06

View:

- **Function List** lists cargo-audit `audit.toml` / `.cargo/audit.toml` sections and fields: `[advisories]` / `[database]` / `[output]` / `[target]` / `[yanked]`, `severity_threshold = "low"` / `"none"` / `"medium"` / `"high"` / `"critical"`, `path = "~/.cargo/advisory-db"`, `url = "https://…"`, `fetch = true`, `stale = false`, `format = "terminal"` / `"json"`, `quiet = false`, `show_tree = true`, `enabled = true`, `update_index = true`. Exact snake_case cargo-audit keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Arrays (`ignore = […]`, `informational_warnings = […]`, `deny = […]`, `arch = […]`, `os = […]`) / prose / wrong case / cliff / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.



## [0.3.632] — 2026-11-06

View:

- **Function List** lists git-cliff `cliff.toml` sections and fields: `[changelog]` / `[git]` / `[bump]` / `[remote]` / `[remote.github]` / `[remote.gitlab]` / `[remote.gitea]` / `[remote.bitbucket]` / `[remote.azure_devops]`, `trim = true`, `render_always = false`, `format = true`, `header_marker = "…"`, `output = "CHANGELOG.md"`, `conventional_commits = true`, `require_conventional = false`, `filter_unconventional = true`, `split_commits = false`, `protect_breaking_commits = false`, `filter_commits = false`, `fail_on_unmatched_commit = false`, `tag_pattern = "v[0-9].*"`, `skip_tags = "beta|alpha"`, `ignore_tags = "rc"`, `count_tags = "…"`, `limit_tags = 10`, `use_branch_tags = false`, `topo_order = false`, `topo_order_commits = false`, `sort_commits = "newest"` / `"oldest"`, `limit_commits = 100`, `recurse_submodules = true`, `offline = false`, `owner = "orhun"`, `repo = "git-cliff"`, `api_url = "https://…"`, `native_tls = true`, `features_always_bump_minor = true`, `breaking_always_bump_major = true`, `initial_tag = "0.1.0"`, `custom_major_increment_regex = "…"`, `custom_minor_increment_regex = "…"`, `no_increment_regex = "…"`. Exact snake_case git-cliff keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`postprocessors = […]`, `commit_parsers = […]`, `commit_preprocessors = […]`, `link_parsers = […]`, `include_paths = […]`, `exclude_paths = […]`, `processing_order = […]`) / multiline templates (`header` / `body` / `footer`) / `token` / prose / wrong case / cargo-make / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.



## [0.3.631] — 2026-11-06

View:

- **Function List** lists cargo-make `Makefile.toml` sections and fields: `[config]` / `[config.modify_core_tasks]` / `[tasks.build]` / `[tasks.ci-flow]`, `skip_core_tasks = true`, `default_to_workspace = false`, `time_summary = true`, `reduce_input_verbosity = true`, `skip_git_env_info = true`, `skip_rust_env_info = true`, `skip_crate_env_info = true`, `load_cargo_aliases = true`, `load_script = "…"`, `init_task = "…"`, `end_task = "…"`, `on_error_task = "…"`, `main_project_member = "…"`, `skip_tools_install = true`, `disable_color = true`, `min_version = "0.37.0"`, `script_runner = "@duckscript"`, `script_extension = "sh"`, `install_crate = "cargo-outdated"`, `install_script = "…"`, `ignore_errors = true`, `condition_script = "…"`, `CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = "true"`. Exact snake_case cargo-make keys (and `CARGO_MAKE_*` env keys). TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`args = […]`, `dependencies = […]`, `env = {…}`, `run_task = {…}`, `condition = {…}`) / prose / wrong case / release.toml / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.



## [0.3.630] — 2026-11-06

View:

- **Function List** lists cargo-release `release.toml` / `.release.toml` sections and fields: `[rate-limit]`, `sign-commit = false`, `sign-tag = false`, `release = true`, `shared-version = false` / `"group"`, `dependent-version = "upgrade"` / `"fix"` / `"error"` / `"warn"` / `"ignore"`, `metadata = "optional"` / `"required"` / `"ignore"` / `"persistent"`, `consolidate-commits = true`, `pre-release-commit-message = "chore: Release"`, `tag = true`, `tag-message` / `tag-name` / `tag-prefix`, `push = true`, `push-remote = "origin"`, `publish = true`, `registry = "…"`, `rate-limit.new-packages = 5`, `rate-limit.existing-packages = 30`, `new-packages` / `existing-packages` under `[rate-limit]`, `certs-source = "webpki"` / `"native"`, `verify = true`, `enable-all-features = false`, `target = "x86_64-unknown-linux-gnu"`. Exact kebab-case cargo-release keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Arrays (`allow-branch = […]`, `pre-release-hook = […]`, `owners = […]`, `enable-features = […]`, `push-options = […]`, `pre-release-replacements = […]`) / prose / wrong case / Dist.toml / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.



## [0.3.629] — 2026-11-06

View:

- **Function List** lists cargo-dist `Dist.toml` / `dist-workspace.toml` sections and fields: `[dist]` / `[dist.bin-aliases]` / `[dist.azure-windows-sign]` / `[dist.github-action-commits]` / `[dist.github-custom-runners.TRIPLE]` / `[dist.github-custom-runners.TRIPLE.container]` / `[[dist.extra-artifacts]]`, `cargo-dist-version = "0.32.0"`, `ci = "github"`, `pr-run-mode = "plan"` / `"upload"` / `"skip"`, `hosting = "github"` / `"simple"`, `install-updater = false`, `github-attestations = true`, `create-release = true`, `dist = true`, `checksum = "sha256"` / `"sha512"` / `"blake2b"` / `"false"`, `source-tarball = true`, `recursive-tarball = true`, `ssldotcom-windows-sign = "prod"` / `"test"`, `auto-includes = true`, `unix-archive = ".tar.gz"` / `".tar.xz"` / `".tar.zst"` / `".zip"`, `windows-archive = ".zip"` / `".tar.gz"`, `precise-builds = true`, `cache-builds = false`, `build-local-artifacts = true`, `dispatch-releases = false`, `merge-tasks = false`, `force-latest = false`, `publish-prereleases = false`, `msvc-crt-static = true`, `npm-shrinkwrap = true`, `npm-scope = "@axodotdev"`, `npm-package = "mycoolapp"`, `tap = "owner/homebrew-tap"`, `formula = "myformula"`, `install-path = "CARGO_HOME"` / `"~/.my-app/"`, `install-success-msg = "…"`, `tag-namespace = "…"`, `github-releases-repo = "owner/repo"`, `github-releases-branch = "main"`, `simple-download-url = "https://…"`. Exact kebab-case cargo-dist keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`targets = […]`, `installers = […]`, `packages = […]`, `hosting = […]`, `install-path = […]`, `allow-dirty = […]`) / prose / wrong case / mdBook / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.


## [0.3.628] — 2026-11-05

View:

- **Function List** lists mdBook `book.toml` sections and fields: `[book]` / `[rust]` / `[build]` / `[preprocessor.name]` / `[output.html]` / `[output.html.search]` / `[output.html.print]` / `[output.html.fold]` / `[output.html.playground]` / `[output.markdown]` / `[output.name]`, `title = "Example book"`, `description = "…"`, `src = "src"`, `language = "en"`, `text-direction = "ltr"` / `"rtl"`, `logo = "static/logo.svg"`, `edition = "2018"`, `build-dir = "book"`, `create-missing = true`, `use-default-preprocessors = true`, `theme = "my-theme"`, `default-theme = "light"`, `preferred-dark-theme = "navy"`, `smart-punctuation = true`, `definition-lists = true`, `admonitions = true`, `mathjax-support = false`, `no-section-label = false`, `git-repository-url = "https://…"`, `git-repository-icon = "fab-github"`, `edit-url-template = "https://…/{path}"`, `input-404 = "404.md"`, `site-url = "/book/"`, `canonical-site-url = "https://…"`, `cname = "…"`, `hash-files = true`, `sidebar-header-nav = true`, `zoomable-images = true`, `enable = true`, `page-break = true`, `level = 0`, `editable = false`, `copyable = true`, `copy-js = true`, `line-numbers = false`, `runnable = true`, `limit-results = 30`, `teaser-word-count = 30`, `use-boolean-and = true`, `boost-title = 2`, `boost-hierarchy = 1`, `boost-paragraph = 1`, `expand = true`, `heading-split-level = 3`, `command = "mdbook-foo"`, `optional = true`. Exact kebab-case / snake_case mdBook keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`authors = […]`, `additional-css = […]`, `extra-watch-dirs = […]`, `ignore = […]`, `renderers = […]`) / prose / wrong case / taplo / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.627] — 2026-11-05

View:

- **Function List** lists Taplo `taplo.toml` / `.taplo.toml` sections and fields: `[formatting]` / `[schema]` / `[[rule]]` / `[rule.formatting]` / `[rule.schema]`, `align_entries = true`, `align_comments = true`, `align_single_comments = true`, `array_trailing_comma = true`, `array_auto_expand = true`, `array_auto_collapse = true`, `compact_arrays = true`, `compact_inline_tables = false`, `inline_table_expand = true`, `compact_entries = false`, `column_width = 80`, `indent_tables = false`, `indent_entries = false`, `indent_string = "  "`, `trailing_newline = true`, `reorder_keys = false`, `reorder_arrays = false`, `reorder_inline_tables = false`, `allowed_blank_lines = 2`, `crlf = false`, `enabled = true`, `path = "taplo://taplo.toml"` / `"./schema.json"` / `"https://…"`. Exact snake_case Taplo keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`include = […]`, `exclude = […]`, `keys = […]`) / prose / wrong case / typos / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.626] — 2026-11-05

View:

- **Function List** lists crate-ci typos `typos.toml` / `_typos.toml` / `.typos.toml` sections and fields: `[files]` / `[default]` / `[default.extend-words]` / `[default.extend-identifiers]` / `[type.rust]` / `[type.py]`, `ignore-hidden = true`, `ignore-files = true`, `ignore-dot = true`, `ignore-vcs = true`, `ignore-global = true`, `ignore-parent = true`, `check-filename = true`, `check-file = true`, `locale = "en"` / `"en-us"` / `"en-gb"` / `"en-ca"` / `"en-au"`. Exact kebab-case typos keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`extend-exclude = […]`, `extend-ignore-re = […]`, free-form `teh = "teh"` under extend-words) / prose / wrong case / `unicode` / `binary` (cargo config / generic) / bacon / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.625] — 2026-11-05

View:

- **Function List** lists bacon `bacon.toml` sections and fields: `[jobs.check]` / `[jobs.clippy-all]` / `[keybindings]` / `[exports]` / `[exports.name]` / `[skin]`, `default_job = "check"`, `need_stdout = false`, `analyzer = "cargo_json"`, `on_success = "back"`, `allow_warnings = true`, `allow_failures = false`, `kill_before_restart = false`, `hide_scrollbar = true`, `show_help_panel = false`, `on_change_strategy = "wait_then_restart"`, `scrollback_len = 10000`, `apply_gitignore = true`, `workdir = "."`, `summary = true`, `wrap = false`, `reverse = false`, `listen = false`, `background = false`, `notifications = true`, `help_line = true`, `show_system_info = false`, keybinding `c = "job:clippy-all"`. Exact snake_case bacon keys (and chord keys under `[keybindings]`). TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`command = […]`, `watch = […]`, `env.FOO = …`, `sound.enabled`) / prose / wrong case / Cross / rust-toolchain / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.624] — 2026-11-05

View:

- **Function List** lists rust-toolchain.toml sections and fields: `[toolchain]`, `channel = "stable"` / `"1.70.0"` / `"nightly-2024-01-01"`, `profile = "minimal"` / `"default"` / `"complete"`, `path = "./custom-toolchain"`. Exact lowercase keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`components = […]`, `targets = […]`) / prose / wrong case / Cross / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.623] — 2026-11-05

View:

- **Function List** lists Cross-rs `Cross.toml` sections and fields: `[build]` / `[build.env]` / `[build.dockerfile]` / `[target.TRIPLE]`, `default-target = "x86_64-unknown-linux-gnu"`, `xargo = true`, `build-std = false`, `zig = false` / `zig = "2.17"`, `dockerfile = "./Dockerfile"`, `image = "my/image:latest"` / `":edge"` / `"@sha256:…"`, `runner = "qemu-user"`, `file = "./Dockerfile"`, `context = "."`. Exact kebab-case Cross keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`pre-build = […]`, `build-std = […]`, `image = { … }`) / prose / wrong case / cargo config / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.622] — 2026-11-05

View:

- **Function List** lists Cargo `.cargo/config.toml` / `config.toml` sections and fields: `[alias]` / `[build]` / `[doc]` / `[future-incompat-report]` / `[cargo-new]` / `[http]` / `[install]` / `[net]` / `[registry]` / `[registries.name]` / `[source.name]` / `[target.TRIPLE]` / `[target.'cfg(…)']` / `[term]` / `[term.progress]` / `[unstable]`, `jobs = 8`, `target-dir = "target"`, `rustflags = "-C target-cpu=native"`, `incremental = true`, `git-fetch-with-cli = true`, `offline = false`, `retry = 3`, `protocol = "sparse"`, `linker = "clang"`, `runner = "qemu-arm"`, `hyperlinks = true`, `unicode = true`, `vcs = "git"`, `check-revoke = true`, `multiplexing = true`, `frequency = "always"`, `proxy = "http://…"`, `cainfo = "/etc/ssl/cert.pem"`, `replace-with = "vendored-sources"`, `local-registry = "vendor"`, `index = "sparse+https://…"`, `credential-provider = "cargo:token"`, `dep-info-basedir = "."`, `low-speed-limit = 10`, `ssl-version = "tlsv1.2"`, `user-agent = "cargo"`, `rustc-wrapper = "sccache"`. Exact kebab-case Cargo config keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`rustflags = […]`, `global-credential-providers`) / free-form `[alias]` / `[env]` keys / generic TOML (`name` / `timeout` / `color`) / prose / wrong case / nextest / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo.toml stay out.

## [0.3.621] — 2026-11-05

View:

- **Function List** lists cargo-nextest `nextest.toml` / `.config/nextest.toml` sections and fields: `[store]` / `[profile.default]` / `[profile.ci]` / `[profile.name]` / `[profile.name.junit]`, `dir = "target/nextest"`, `retries = 2`, `test-threads = "num-cpus"`, `threads-required = 1`, `slow-timeout = "60s"`, `leak-timeout = "100ms"`, `status-level = "pass"`, `final-status-level = "fail"`, `failure-output = "immediate"`, `success-output = "never"`, `max-fail = 1`, `fail-fast = true`, `run-ignored = "default"`, `default-filter = "all()"`, `no-run = false`, `force-run-flaky = false`, `path = "junit.xml"`, `report-name = "nextest"`. Exact kebab-case nextest keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`retries = { … }`, `slow-timeout = { … }`) / prose / wrong case / deny / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo stay out.

## [0.3.620] — 2026-11-05

View:

- **Function List** lists cargo-deny `deny.toml` sections and fields: `[graph]` / `[output]` / `[advisories]` / `[licenses]` / `[licenses.private]` / `[bans]` / `[sources]` / `[sources.allow-org]`, `vulnerability = "deny"`, `unmaintained = "warn"`, `unsound = "warn"`, `yanked = "warn"`, `notice = "warn"`, `unlicensed = "deny"`, `copyleft = "warn"`, `allow-osi-fsf-free = "either"`, `confidence-threshold = 0.8`, `multiple-versions = "warn"`, `wildcards = "allow"`, `highlight = "all"`, `workspace-default-features = "deny"`, `external-default-features = "allow"`, `unknown-registry = "warn"`, `unknown-git = "warn"`, `all-features = false`, `no-default-features = false`, `exclude-dev = true`, `feature-depth = 1`, `git-fetch-with-cli = true`, `allow-wildcard-paths = true`, `multiple-versions-include-dev = true`, `ignore = true`, `db-path = "~/.cargo/advisory-db"`. Exact kebab-case deny keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`allow` / `deny` / `ignore` / `targets`) / prose / wrong case / clippy / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo stay out.

## [0.3.619] — 2026-11-05

View:

- **Function List** lists Rust `clippy.toml` / `.clippy.toml` fields: `msrv = "1.70.0"`, `avoid-breaking-exported-api = true`, `cognitive-complexity-threshold = 25`, `too-many-arguments-threshold = 7`, `too-many-lines-threshold = 100`, `type-complexity-threshold = 250`, `excessive-nesting-threshold = 0`, `enum-variant-size-threshold = 200`, `array-size-threshold = 16384`, `allow-expect-in-tests = false`, `allow-unwrap-in-tests = false`, `allow-dbg-in-tests = false`, `allow-print-in-tests = false`, `allow-panic-in-tests = false`, `check-private-items = false`, `warn-on-all-wildcard-imports = false`, `upper-case-acronyms-aggressive = false`, `inherent-impl-lint-scope = "crate"`, `matches-for-let-else = "well-known-types"`, `pub-underscore-fields-behavior = "publicly-exported"`, `trivial-copy-size-limit = 128`. Exact kebab-case Clippy keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`disallowed-names` / `doc-valid-idents`) / prose / wrong case / rustfmt / ruff / uv / bunfig / yarn / npmrc / pip / Cargo stay out.

## [0.3.618] — 2026-11-05

View:

- **Function List** lists Rust `rustfmt.toml` / `.rustfmt.toml` fields: `max_width = 100`, `hard_tabs = false`, `tab_spaces = 4`, `newline_style = "Unix"`, `indent_style = "Block"`, `use_small_heuristics = "Default"`, `reorder_imports = true`, `reorder_modules = true`, `remove_nested_parens = true`, `edition = "2021"`, `version = "Two"`, `wrap_comments = false`, `format_code_in_doc_comments = false`, `normalize_comments = false`, `normalize_doc_attributes = false`, `format_strings = false`, `format_macro_matchers = false`, `format_macro_bodies = true`, `empty_item_single_line = true`, `struct_lit_single_line = true`, `fn_single_line = false`, `where_single_line = false`, `imports_indent = "Block"`, `imports_layout = "Mixed"`, `imports_granularity = "Crate"`, `group_imports = "StdExternalCrate"`, `blank_lines_upper_bound = 1`, `blank_lines_lower_bound = 0`, `trailing_semicolon = true`, `trailing_comma = "Vertical"`, `match_block_trailing_comma = false`, `force_explicit_abi = true`, `condense_wildcard_suffixes = false`, `color = "Auto"`, `required_version = "1.4.38"`, `unstable_features = false`, `disable_all_formatting = false`, `skip_children = false`, `hide_parse_errors = false`, `error_on_line_overflow = false`, `error_on_unformatted = false`, `verbose = "Normal"`, `emit_mode = "Files"`, `make_backup = false`, `merge_derives = true`, `use_try_shorthand = true`, `use_field_init_shorthand = true`, `brace_style = "SameLineWhere"`, `control_brace_style = "AlwaysSameLine"`, `binop_separator = "Front"`, `fn_params_layout = "Tall"`. Exact snake_case rustfmt keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`ignore` / `file_lines`) / prose / wrong case / ruff / uv / bunfig / yarn / npmrc / pip / Cargo stay out.

## [0.3.617] — 2026-10-28

View:

- **Function List** lists Astral `ruff.toml` / `.ruff.toml` sections and fields: `[lint]` / `[lint.isort]` / `[lint.pydocstyle]` / `[lint.pycodestyle]` / `[lint.pyflakes]` / `[lint.pylint]` / `[lint.mccabe]` / `[lint.flake8-…]` / `[lint.pep8-naming]` / `[lint.per-file-ignores]` / `[lint.extend-per-file-ignores]` / `[format]` / `[analyze]`, `line-length = 88`, `indent-width = 4`, `target-version = "py39"`, `preview = true`, `fix = true`, `unsafe-fixes = true`, `output-format = "grouped"`, `required-version = ">=0.4.0"`, `extend = "../ruff.toml"`, `cache-dir = ".ruff_cache"`, `respect-gitignore = true`, `force-exclude = true`, `quote-style = "double"`, `indent-style = "space"`, `skip-magic-trailing-comma = false`, `line-ending = "auto"`, `docstring-code-format = true`, `docstring-code-line-length = "dynamic"`, `convention = "google"`, `max-complexity = 10`, `max-line-length = 88`, `force-single-line = true`, `relative-imports-order = "furthest-to-closest"`, `direction = "dependencies"`. Exact kebab-case Ruff keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays (`select` / `ignore`) / prose / wrong case / uv / bunfig / yarn / npmrc / pip / Cargo stay out.

## [0.3.616] — 2026-10-28

### CLI

- **Column jump:** `-c <N>` / `--column <N>` / `-cN` parks the caret on 1-based column N in the first opened file (with or without `-n`; column alone starts at line 1). Status shows `→ line N col C` when both are set.
- **Skip session:** `-nosession` / `--no-session` skips opt-in session restore on an empty start (Notepad++-style).
- **Read-only alias:** `-readonly` accepted beside `-ro` / `--read-only`.
- `?` → Command Line Arguments and `--help` document the same flags. Unit tests cover parse paths.

## [0.3.615] — 2026-10-26

View:

- **Function List** lists Astral `uv.toml` sections and fields: `[pip]` / `[build-backend]` / `[sources]` / `[[index]]`, `native-tls = true`, `offline = false`, `no-cache = true`, `preview = true`, `python-preference = "managed"`, `python-downloads = "automatic"`, `concurrent-downloads = 50`, `concurrent-builds = 16`, `concurrent-installs = 8`, `keyring-provider = "subprocess"`, `required-version = ">=0.4.0"`, `resolution = "highest"`, `prerelease = "if-necessary-or-explicit"`, `link-mode = "clone"`, `compile-bytecode = true`, `package = false`, `managed = true`, `index-strategy = "first-index"`, `fork-strategy = "requires-python"`, `torch-backend = "auto"`, `add-bounds = "lower"`, `exclude-newer = "7 days"`, `publish-url = "https://…"`, `check-url = "https://…"`, `trusted-publishing = "always"`, `system-certs = true`, `allow-insecure-host = "pypi.example.com"`, `python-install-mirror = "https://…"`, `http-proxy = "http://…"`, `https-proxy = "https://…"`, `no-proxy = "localhost"`, `all-extras = true`, `allow-empty-requirements = true`, `annotation-style = "line"`. Exact kebab-case uv keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays / prose / wrong case / bunfig / yarn / npmrc / pip (`index-url` / `[global]`) / Cargo `[package]` stay out.

## [0.3.614] — 2026-10-26

View:

- **Function List** lists Bun `bunfig.toml` sections and fields: `[install]` / `[install.scopes]` / `[install.lockfile]` / `[run]` / `[test]` / `[debug]` / `[define]` / `[loader]` / `[serve.static]`, `telemetry = false`, `jsx = "react"`, `smol = false`, `logLevel = "warn"`, `frozenLockfile = true`, `exact = true`, `registry = "https://registry.npmjs.org"`, `backend = "clonefile"`, `linker = "isolated"`, `concurrentScripts = 16`, `shell = "bun"`, `silent = false`, `coverage = true`, `updateSnapshots = false`, `editor = "code"`, `coverageDir = "coverage"`. Exact camelCase / lowercase Bun keys. TOML `key = value` with spaces around `=`. Optional `#` comment. Nested maps / arrays / prose / wrong case / `.npmrc` `registry=` / Yarn YAML / pip / neighbouring families stay out.

## [0.3.613] — 2026-10-26

View:

- **Function List** lists Yarn Berry `.yarnrc.yml` fields: `nodeLinker: node-modules` / `pnp` / `pnpm`, `yarnPath: .yarn/releases/yarn-4.0.0.cjs`, `npmRegistryServer: "https://registry.yarnpkg.com"`, `enableGlobalCache: true`, `enableImmutableInstalls: false`, `compressionLevel: mixed`, `httpProxy: http://…`, `httpsProxy: https://…`, `preferOffline: true`, `enableStrictSsl: true`, `cacheFolder: ./.yarn/cache`, `httpTimeout: 60000`, `checksumBehavior: throw`, `nmMode: classic`, `pnpMode: strict`, `npmPublishAccess: public`, `winLinkType: junctions`. Exact camelCase Yarn config keys. Optional `#` comment. Nested maps / arrays / prose / wrong case / `.npmrc` `registry=` / pip / neighbouring families stay out.

## [0.3.612] — 2026-10-26

View:

- **Function List** lists `.npmrc` fields: `registry=https://registry.npmjs.org/`, `@myorg:registry=https://npm.pkg.github.com`, `always-auth=true`, `save-exact=true`, `engine-strict=true`, `legacy-peer-deps=true`, `fund=false`, `audit=false`, `strict-ssl=true`, `proxy=http://…`, `https-proxy=…`, `noproxy=localhost,127.0.0.1`, `prefix=/usr/local`, `cache=/tmp/npm-cache`, `node-options=--max-old-space-size=4096`, `fetch-retries=2`, `loglevel=warn`, `omit=dev`, `package-lock=true`, plus common pnpm keys (`shamefully-hoist=true`, `node-linker=hoisted`, `auto-install-peers=true`). Exact lowercase keys (npm config / CLI names). Optional `#` / `;` comment. Bare keys / prose / wrong case / `user=alice` / `tag=foo` / pip `index-url` / yum `[main]` / pacman `[options]` / apt `deb` / neighbouring families stay out.

## [0.3.611] — 2026-10-26

View:

- **Function List** lists pip.conf / pip.ini sections and fields: `[global]` / `[install]` / `[download]` / `[list]` / `[freeze]` / `[wheel]` / `[hash]` / `[search]` / `[uninstall]` / `[debug]`, `index-url = https://pypi.org/simple`, `extra-index-url = https://…`, `trusted-host = pypi.org`, `timeout = 60`, `retries = 5`, `require-virtualenv = true`, `no-cache-dir = true`, `disable-pip-version-check = true`, `find-links = /srv/wheels`, `cache-dir = /var/cache/pip`, `no-deps = true`, `user = true`, `upgrade-strategy = only-if-needed`, `force-reinstall = true`, `break-system-packages = true`, `format = columns`. Exact lowercase keys (CLI long-option names). Optional `#` / `;` comment. Bare keys / prose / wrong case / `user = alice` / `path = tmp` / yum `[main]` / pacman `[options]` / apt `deb` / neighbouring families stay out.

## [0.3.610] — 2026-10-26

View:

- **Function List** lists yum / dnf `.repo` files (`/etc/yum.repos.d/*.repo`) and `/etc/dnf/dnf.conf` / `/etc/yum.conf`: `[main]`, stock `[fedora]` / `[updates]` / `[epel]` / …, free-form lowercase ids with a separator or digit (`[google-chrome]`, `[docker-ce-stable]`, `[fedora39]`), `name=Fedora $releasever - $basearch` (display names need a space or `$`), `baseurl=` / `metalink=` / `mirrorlist=`, `enabled=1`, `gpgcheck=1`, `gpgkey=file:///etc/pki/rpm-gpg/…`, `repo_gpgcheck=0`, `countme=1`, `skip_if_unavailable=False`, `metadata_expire=7d`, `module_hotfixes=1`, `installonly_limit=3`, `clean_requirements_on_remove=True`, `best=False`, `max_parallel_downloads=10`, `cachedir=/var/cache/dnf`. Exact lowercase keys. Optional `#` / `;` comment. Bare keys / prose / wrong case / `[Main]` / `name=arch` / Cargo `[dev-dependencies]` / pacman `[options]` / apt `deb` / neighbouring families stay out.

## [0.3.609] — 2026-10-26

View:

- **Function List** lists apt `sources.list` / `sources.list.d/*.list` one-line entries and deb822 `*.sources` fields: `deb http://… bookworm main`, `deb-src http://… jammy main`, `deb [arch=amd64 signed-by=…] https://… stable main`, `deb file:/srv/local-repo ./`, `deb cdrom:[label]/debian/ bookworm main` (space-free cdrom labels), `Types: deb`, `URIs: http://…`, `Suites: bookworm`, `Components: main`, `Signed-By: /usr/share/keyrings/…`, `Architectures: amd64`, `Enabled: yes`, `Trusted: yes`, `Allow-Insecure: yes`, `Check-Valid-Until: no`, `Snapshot: …`. Exact `deb` / `deb-src` / Title-Case deb822 keys. Optional `#` comment. Bare `deb`, prose (`deb the team`), wrong case, space-in-cdrom labels, Dockerfile `FROM`, pacman / paru / neighbouring families stay out.

## [0.3.608] — 2026-10-25

View:

- **Function List** lists `/etc/paru.conf` / `~/.config/paru/paru.conf` sections and fields: `[options]`, `[bin]`, `PgpFetch`, `Devel`, `Provides`, `BottomUp`, `AurOnly`, `CombinedUpgrade`, `CleanAfter`, `BatchInstall`, `NewsOnUpgrade`, `SkipReview`, `SudoLoop`, `LocalRepo`, `Chroot`, `KeepRepoCache`, `DevelSuffixes = -git -cvs -svn`, `Mode = aur,repo`, `RemoveMake = ask`, `Redownload = all`, `Rebuild = tree`, `AurUrl = https://aur.archlinux.org`, `CloneDir = /var/cache/paru`, `SortBy = votes`, `SearchBy = name-desc`, `Limit = 50`, `CompletionInterval = 7`, `PacmanConf = /etc/pacman.conf`, `MakepkgConf = /etc/makepkg.conf`, `FileManager = vifm`, `MFlags = --skippgpcheck`, `Sudo = doas`, `Git = /usr/bin/git`, `Pager = less`. Exact stock key casing. Optional `#` / `;` comment. Bare keys / prose / wrong case / free-form `[env]` / PKGBUILD repo sections / `PATH=` / pacman / makepkg / neighbouring families stay out.

## [0.3.607] — 2026-10-25

View:

- **Function List** lists `/etc/pacman.conf` / `/etc/pacman.d` sections and fields: `[options]`, `[core]` / `[extra]` / `[multilib]` / `[community]` / testing / unstable repos, `HoldPkg = pacman glibc`, `IgnorePkg` / `IgnoreGroup`, `Architecture = auto`, `CheckSpace` / `Color` / `ILoveCandy` / `VerbosePkgLists` (flag form), `ParallelDownloads = 5`, `SigLevel = Required DatabaseOptional`, `LocalFileSigLevel` / `RemoteFileSigLevel`, `Include = /etc/pacman.d/mirrorlist`, `Server = https://…/$repo/os/$arch`, `RootDir` / `DBPath` / `CacheDir` / `LogFile` / `GPGDir` / `HookDir`, `CleanMethod`, `Usage`, `DownloadUser`. Exact stock key casing. Optional `#` / `;` comment. Bare keys / prose / wrong case / custom `[reponame]` / `PATH=` / makepkg / neighbouring families stay out.

## [0.3.606] — 2026-10-25

View:

- **Function List** lists `/etc/makepkg.conf` / `makepkg.conf.d/*.conf` fields: `CARCH="x86_64"`, `CHOST="x86_64-pc-linux-gnu"`, `CFLAGS="…"`, `CXXFLAGS="$CFLAGS"`, `LDFLAGS="…"`, `LTOFLAGS="…"`, `RUSTFLAGS="…"`, `DEBUG_CFLAGS="…"`, `MAKEFLAGS="-j$(nproc)"`, `NPROC=8`, `BUILDENV=(!distcc color !ccache check !sign)`, `OPTIONS=(strip docs !libtool staticlibs emptydirs !zipman !purge !debug lto)`, `INTEGRITY_CHECK=(sha256)`, `STRIP_BINARIES="--"`, `MAN_DIRS=(…)`, `DOC_DIRS=(…)`, `LIB_DIRS=(…)`, `PURGE_TARGETS=(…)`, `DBGSRCDIR="/usr/src/debug"`, `PKGDEST=/var/cache/pacman/pkg`, `SRCDEST=/var/cache/makepkg`, `PACKAGER="Name <user@example.com>"`, `GPGKEY="…"`, `PKGEXT='.pkg.tar.zst'`, `SRCEXT='.src.tar.gz'`, `COMPRESSXZ=(xz -c -z -)`, `COMPRESSZST=(zstd -c -z -q -)`, `DLAGENTS=('http::/usr/bin/curl …')`, `VCSCLIENTS=('git::git' 'hg::mercurial')`. Stock UPPER_SNAKE KEY=`VALUE` (exact case). Optional double-quoted value (escapes ok; empty / `()` / single-quoted ok). Optional `#` comment outside quotes. Bare keys / prose / `PATH=` / grub / mkinitcpio / dracut / neighbouring families stay out.

## [0.3.605] — 2026-10-25

View:

- **Function List** lists `/etc/dracut.conf` / `dracut.conf.d/*.conf` fields: `add_dracutmodules+=" nfs "`, `omit_dracutmodules+=" plymouth "`, `dracutmodules+=" base "`, `add_drivers+=" ahci "`, `omit_drivers+=" nouveau "`, `filesystems+=" ext4 xfs "`, `install_items+=" /etc/foo /bin/bar "`, `compress="xz"`, `compress=zstd`, `hostonly="yes"`, `hostonly_mode="sloppy"`, `hostonly_cmdline="yes"`, `persistent_policy="by-uuid"`, `tmpdir="/var/tmp"`, `use_fstab="yes"`, `mdadmconf="yes"`, `lvmconf="yes"`, `kernel_cmdline="quiet splash"`, `early_microcode="yes"`, `show_modules="yes"`, `do_strip="yes"`, `uefi="yes"`, `reproducible="yes"`, `stdloglvl="3"`, `sshkey="/root/.ssh/id_rsa"`. Stock lowercase snake_case KEY=`VALUE` (exact case); optional trailing `+` before `=` for append form. Optional double-quoted value (escapes ok; empty allowed). Optional `#` comment outside quotes. Bare keys / prose / `PATH=` / mkinitcpio / grub / neighbouring families stay out.


## [0.3.604] — 2026-10-25

View:

- **Function List** lists `/etc/mkinitcpio.conf` / `mkinitcpio.d/*.preset` fields: `MODULES=()`, `MODULES=(ext4)`, `BINARIES=()`, `FILES=()`, `HOOKS=(base udev autodetect …)`, `COMPRESSION="zstd"`, `COMPRESSION=xz`, `COMPRESSION_OPTIONS=()`, `MODULES_DECOMPRESS="no"`, `PRESETS=('default' 'fallback')`, `ALL_config="/etc/mkinitcpio.conf"`, `ALL_kver="/boot/vmlinuz-linux"`, `default_image="/boot/initramfs-linux.img"`, `fallback_image="/boot/initramfs-linux-fallback.img"`, `fallback_options="-S autodetect"`. Stock mkinitcpio KEY=`VALUE` (exact case), optional double-quoted value (escapes ok; empty / `()` allowed). Optional `#` comment outside quotes. Bare keys / prose / `PATH=` / grub / vconsole / locale / neighbouring families stay out.

## [0.3.603] — 2026-10-25

View:

- **Function List** lists `/etc/default/grub` fields: `GRUB_DEFAULT=0`, `GRUB_DEFAULT=saved`, `GRUB_TIMEOUT=5`, `GRUB_TIMEOUT_STYLE=menu`, `GRUB_DISTRIBUTOR="Arch"`, `GRUB_CMDLINE_LINUX_DEFAULT="quiet splash"`, `GRUB_CMDLINE_LINUX=""`, `GRUB_DISABLE_OS_PROBER=false`, `GRUB_GFXMODE=auto`, `GRUB_THEME="/usr/share/grub/themes/…"`, `GRUB_INIT_TUNE="480 440 1"`, `GRUB_COLOR_NORMAL="light-gray/black"`, `GRUB_SAVEDEFAULT=true`, `GRUB_ENABLE_CRYPTODISK=y`. Stock `GRUB_*` KEY=`VALUE` (exact case), optional double-quoted value (escapes ok; empty value allowed). Optional `#` comment outside quotes. Bare keys / prose / `PATH=` / vconsole / locale / os-release / neighbouring families stay out.

## [0.3.602] — 2026-10-25

View:

- **Function List** lists `/etc/vconsole.conf` fields: `KEYMAP=us`, `KEYMAP_TOGGLE=us`, `FONT=lat9w-16`, `FONT_MAP=8859-2`, `FONT_UNIMAP=latarcyrheb-sun`, `XKBLAYOUT=us,de`, `XKBMODEL=pc105`, `XKBVARIANT=nodeadkeys`, `XKBOPTIONS=grp:alt_shift_toggle`. Stock vconsole KEY=`VALUE` (exact case), optional double-quoted value (escapes ok). Optional `#` comment outside quotes. Bare keys / prose / `PATH=` / locale.conf / os-release / machine-info / neighbouring families stay out.

## [0.3.601] — 2026-10-25

View:

- **Function List** lists `/etc/locale.conf` fields: `LANG=en_US.UTF-8`, `LANGUAGE=en_US:en`, `LC_CTYPE=en_US.UTF-8`, `LC_NUMERIC=de_DE.UTF-8`, `LC_TIME=C`, `LC_COLLATE=POSIX`, `LC_MONETARY=fr_FR.UTF-8`, `LC_MESSAGES="en_US.UTF-8"`, `LC_PAPER=en_GB.UTF-8`, `LC_NAME=…`, `LC_ADDRESS=…`, `LC_TELEPHONE=…`, `LC_MEASUREMENT=…`, `LC_IDENTIFICATION=…`, `LC_ALL=C.UTF-8`. Stock locale KEY=`VALUE` (exact case), optional double-quoted value (escapes ok). Optional `#` comment outside quotes. Bare keys / prose / `PATH=` / os-release / machine-info / login.defs / neighbouring families stay out.

## [0.3.600] — 2026-10-25

View:

- **Function List** lists `/etc/machine-info` fields: `PRETTY_HOSTNAME=studio`, `ICON_NAME=computer-laptop`, `CHASSIS=laptop`, `DEPLOYMENT=development`, `LOCATION="Server Room 2"`, `HARDWARE_VENDOR=LENOVO`, `HARDWARE_MODEL="ThinkPad X1"`, `HARDWARE_SKU=20KH`, `HARDWARE_VERSION=1.0`, `FIRMWARE_VENDOR=LENOVO`, `FIRMWARE_VERSION=N23ET76W`, `FIRMWARE_DATE=2024-03-15`. Stock UPPER_SNAKE key, `=`, optional double-quoted value (escapes ok). Optional `#` comment outside quotes. Bare keys / prose / `PATH=` / `LANG=` / os-release / login.defs space form / neighbouring families stay out.

## [0.3.599] — 2026-10-25

View:

- **Function List** lists `/etc/os-release` / `/usr/lib/os-release` fields: `NAME="Arch Linux"`, `PRETTY_NAME="Arch Linux"`, `ID=arch`, `ID_LIKE="rhel fedora"`, `VERSION_ID="39"`, `VERSION_CODENAME=bookworm`, `BUILD_ID=rolling`, `HOME_URL="https://…"`, `ANSI_COLOR="38;2;23;147;209"`, `LOGO=archlinux-logo`, `VARIANT_ID=server`, `CPE_NAME="cpe:/o:…"`. Stock UPPER_SNAKE key, `=`, optional double-quoted value (escapes ok). Optional `#` comment outside quotes. Bare keys / prose / `PATH=` / `LANG=` / login.defs space form / neighbouring families stay out.


## [0.3.598] — 2026-10-25

View:

- **Function List** lists `/etc/login.defs` parameters: `FAIL_DELAY 3`, `LOG_UNKFAIL_ENAB no`, `MAIL_DIR /var/spool/mail`, `ENCRYPT_METHOD YESCRYPT`, `ENV_PATH PATH=/usr/local/sbin:…`, `UMASK 022`, `UID_MIN 1000`, `USERGROUPS_ENAB yes`, `HOME_MODE 0700`, `PASS_MIN_LEN 5`, `SUB_UID_COUNT 65536`, `YESCRYPT_COST_FACTOR 5`. Stock UPPER_SNAKE key, whitespace, non-empty value (values may contain `=`). Optional `#` comment. Bare keys / `KEY=value` / prose / neighbouring families (sysctl, …) stay out.


## [0.3.597] — 2026-10-25

View:

- **Function List** lists `/etc/networks` entries: `default 0.0.0.0`, `loopback 127.0.0.0`, `link-local 169.254.0.0`, `localnet 10.0.0.0 local`, `mynet 192.168.1.0/24`, `classb 172.16.0.0/12`, `shortnet 10.0`, `midnet 10.0.0`. Name, dotted network number (optional `/prefix` 0–32), optional aliases. Requires a `.` in the number so `/etc/protocols` `name 0–255` stays on that path. Optional `#` comment. Bare / prose / incomplete / neighbouring families (`/etc/protocols`, hosts, resolv, nsswitch, …) stay out.

## [0.3.596] — 2026-10-25

View:

- **Function List** lists `/etc/ethers` entries: `08:00:20:0a:8c:6d basil`, `00-11-22-33-44-55 host`, `aa:bb:cc:dd:ee:ff host.example.com alias`, `08:00:2b:00:01:02 myhost`, `a:b:c:d:e:f short`, `FF:EE:DD:CC:BB:AA uppercase`. Ethernet MAC (six hex octets, `:` or `-` separator), hostname, optional aliases. Optional `#` comment. Bare / prose / incomplete / neighbouring families (`/etc/hosts`, `/etc/rpc`, nsswitch, …) stay out.


## [0.3.595] — 2026-10-25

View:

- **Function List** lists `/etc/rpc` entries: `portmapper 100000 portmap sunrpc rpcbind`, `nfs 100003 nfsprog`, `ypbind 100007`, `x25.inr 100022`, `3270_mapper 100013`, `rstatd 100001 rstat rup`, `mountd 100005 mount showmount`, `yppasswdd 100009 yppasswd`, `nlockmgr 100021`. Name, RPC program number ≥ 100000, optional aliases. Optional `#` comment. Bare / prose / incomplete / neighbouring families (`/etc/protocols`, `/etc/services`, hosts, nsswitch, …) stay out.

## [0.3.594] — 2026-10-25

View:

- **Function List** lists `/etc/protocols` entries: `ip 0 IP`, `icmp 1 ICMP`, `tcp 6 TCP`, `udp 17 UDP`, `ipv6 41 IPv6`, `sctp 132 SCTP`, `ax.25 93 AX.25`, `ipv6-icmp 58 IPv6-ICMP`, `manet 138`, `ipencap 4 IP-ENCAP`. Name, protocol number 0–255, optional aliases. Optional `#` comment. Bare / prose / incomplete / neighbouring families (`/etc/services`, hosts, nsswitch, …) stay out.


## [0.3.593] — 2026-10-25

View:

- **Function List** lists `/etc/services` entries: `ssh 22/tcp`, `http 80/tcp www www-http`, `https 443/tcp`, `domain 53/udp`, `syslog 514/udp`, `echo 7/tcp`, `sctp-test 9899/sctp`, `dccp-test 9899/dccp`. Name, `port/protocol` (`tcp` / `udp` / `sctp` / `dccp`), optional aliases. Optional `#` comment. Bare / prose / incomplete / neighbouring families (hosts, resolv, nsswitch, …) stay out.

## [0.3.592] — 2026-10-25

View:

- **Function List** lists systemd `binfmt.d` / `binfmt.d/*.conf` register lines: `:DOSWin:M::MZ::/usr/bin/wine:`, `:Python:E::py::/usr/bin/python:`, `:CLR:M::MZ::/usr/bin/mono:`, `:WASM:M::\x00asm::/usr/bin/wasmtime:`, `:qemu-i386:M::\x7fELF…:\xff…:/usr/bin/qemu-i386-static:OC`. Format `:name:type:offset:magic:mask:interpreter:flags` (`type` = `M` / `E`; offset/mask/flags may be empty; magic required; interpreter absolute). Optional `#` / `;` comment. Bare / prose / incomplete / neighbouring families (sysusers, tmpfiles, …) stay out.

## [0.3.591] — 2026-10-25

View:

- **Function List** lists systemd `sysusers.d` / `sysusers.d/*.conf` entries: `u httpd 404 "HTTP User"`, `u! alpm - "Arch Linux Package Management" -`, `u root 0:0 "Super User" /root`, `u! nobody 65534:65534 "Kernel Overflow User" -`, `u _authd /usr/bin/authd "Authorization user"`, `u git - "git daemon user" / /usr/bin/git-shell`, `g input - - -`, `g polkitd 102`, `m cups lp`, `r - 500-999`, `r - 1000-`. Types `u` / `u!` / `g` / `m` / `r`; Name + ID required; optional GECOS (quoted ok) / Home / Shell. Optional `#` / `;` comment. Bare / prose / incomplete / neighbouring families (tmpfiles, passwd, …) stay out.

## [0.3.590] — 2026-10-25

View:

- **Function List** lists systemd `tmpfiles.d` / `tmpfiles.d/*.conf` entries: `d /run/user 0755 root root -`, `f /etc/hostname - - - -`, `L /etc/os-release - - - - /usr/lib/os-release`, `L+ /run/lock - - - -`, `C /etc/pam.d - - - -`, `z /home 0755 - - -`, `Z /var/lib/machines 0700 - - -`, `e /var/cache - - - -`, `x /tmp/.X11-unix - - - -`, `X /dev - - - -`, `r! /tmp/foo - - - -`, `R /tmp/bar - - - -`, `w /etc/motd - - - - Hello`, `p /run/fifo 0660 root root -`, `c /dev/mynode 0660 root root - 1:2`, `b /dev/myblock 0660 root root - 8:0`, `v` / `q` / `Q` machine dirs, `a+ /home/user - - - - u::rwx`, `h /srv/www - - - - +i`, `t /var/lib/foo - - - - security.selinux=…`, `D` / `F` / `A`. Type letter (+ optional `+` / `-` / `=` / `~` / `^` / `!`); absolute path; Mode/User/Group/Age/Argument optional. Optional `#` / `;` comment. Bare / prose / incomplete / neighbouring families (modprobe, fstab, …) stay out.

## [0.3.589] — 2026-10-25

View:

- **Function List** lists `/etc/modprobe.d` / `modprobe.conf` directives: `alias char-major-10-175 thinkpad_acpi`, `alias scsi_hostadapter aic7xxx`, `options snd_hda_intel index=0`, `blacklist nouveau`, `blacklist usb_storage`, `install snd-emu10k1 /sbin/modprobe --ignore-install …`, `remove snd-emu10k1 /sbin/modprobe -r …`, `softdep mlx4_core post: mlx4_en`, `softdep cifs pre: sha512 post: hmac`. Exact lowercase commands (`alias` / `blacklist` / `install` / `remove` / `options` / `softdep`); optional `#` comment. Bare / prose / shell `alias ls=…` / Crystal `alias Name =` / neighbouring families (sysctl, …) stay out.

## [0.3.588] — 2026-10-25

View:

- **Function List** lists `/etc/sysctl.conf` / `sysctl.d` entries: `net.ipv4.ip_forward = 1`, `vm.swappiness=10`, `kernel.printk = 3 4 1 3`, `fs.inotify.max_user_watches = 524288`, `fs.file-max = 2097152`, `kernel.domainname = example.com`, `user.max_user_namespaces = 15000`, `-net.ipv4.conf.all.rp_filter = 1` (leading `-` ignores missing keys). Stock roots (`abi` / `crypto` / `debug` / `dev` / `fs` / `kernel` / `net` / `sunrpc` / `user.max_*` / `vm`); optional `#` comment. Bare / prose / undotted keys (`PATH=…`) / git-style `user.name` / neighbouring families stay out.

## [0.3.587] — 2026-10-25

View:

- **Function List** lists `/etc/subuid` / `/etc/subgid` entries: `alice:100000:65536`, `root:165536:65536`, `user1:100000:65536`, `bob:200000:1000`, `_uuidd:300000:65536`. Three colon fields (login name, subordinate UID/GID start, count). Optional `#` comment. Bare / prose / incomplete / neighbouring families (passwd / group / shadow / gshadow, …) stay out.

## [0.3.586] — 2026-10-25

View:

- **Function List** lists `/etc/shadow` / `/etc/gshadow` entries: `root:$6$rounds=5000$saltsalt$…:19000:0:99999:7:::`, `daemon:*:19000:0:99999:7:::`, `alice:!:19000:0:99999:7:::`, `locked:!!:19100:0:99999:7:::`, `nobody:*:::::::`, `sudo:!:alice,bob:alice,bob`, `docker:!:alice:alice`, `root:*::`, `users:x::alice`. Shadow = 9 colon fields (name, password, lastchg, min, max, warn, inactive, expire, flag; numeric fields may be empty); gshadow = 4 (name, password, admins, members; admins/members comma lists, empty OK). Passwd / group lines still match. Optional `#` comment. Bare / prose / incomplete / neighbouring families (nsswitch `shadow: files`, …) stay out.

## [0.3.585] — 2026-10-25

View:

- **Function List** lists `/etc/passwd` / `/etc/group` entries: `root:x:0:0:root:/root:/bin/bash`, `alice:x:1000:1000:Alice User,,,:/home/alice:/bin/zsh`, `nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin`, `www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin`, `user:!:1001:1001::/home/user:`, `locked:!!:1002:1002::/home/locked:/bin/false`, `sudo:x:27:alice,bob`, `docker:x:999:`, `root:x:0:`, `users:x:100:alice`, `_uuidd:x:104:104::/run/uuidd:/usr/sbin/nologin`. Passwd = 7 colon fields (name, password, UID, GID, GECOS, home, shell); group = 4 (name, password, GID, member list). Optional `#` comment. Bare / prose / incomplete / neighbouring families (nsswitch `passwd: files`, hosts.allow, …) stay out.

## [0.3.584] — 2026-10-25

View:

- **Function List** lists `/etc/nsswitch.conf` databases: `passwd: files systemd`, `group: files systemd`, `shadow: files`, `hosts: files dns`, `hosts: mymachines resolve [!UNAVAIL=return] files myhostname dns`, `networks: files`, `protocols: files`, `services: files`, `ethers: files`, `rpc: files`, `netgroup: files`, `sudoers: files`, `aliases: files`, `gshadow: files`, `initgroups: files`, `automount: files`, `publickey: files`, `group: files [SUCCESS=merge] systemd`, `hosts: mdns4_minimal [NOTFOUND=return] dns`. Optional `#` comment. Bare / prose / incomplete / neighbouring families (hosts.allow `ALL:`, access.conf, resolv.conf, …) stay out.

## [0.3.583] — 2026-10-25

View:

- **Function List** lists `/etc/resolv.conf` directives: `nameserver 192.168.1.1`, `nameserver 2001:db8::1`, `search example.com corp.example.com`, `domain example.com`, `options timeout:2 attempts:3 rotate`, `options edns0 trust-ad`, `sortlist 130.155.160.0/255.255.240.0 130.155.0.0`, `sortlist 192.168.0.0/16`. Optional `#` or `;` comment. Bare / prose / incomplete / neighbouring families (hosts, named `options {`, syslog-ng `options {`, …) stay out.

## [0.3.582] — 2026-10-25

View:

- **Function List** lists `/etc/hosts` entries: `127.0.0.1 localhost`, `::1 localhost ip6-localhost`, `192.168.1.10 server.example.com server`, `ff02::1 ip6-allnodes`, `0.0.0.0 ads.example.com`, `2001:db8::1 www.example.com`, `::ffff:192.0.2.1 mapped.host`. Address then one or more hostnames. Optional `#` comment. Bare / prose / incomplete / neighbouring families (hosts.allow, fstab, crypttab, …) stay out.

## [0.3.581] — 2026-10-25

View:

- **Function List** lists `/etc/crypttab` entries: `cr_home UUID=… none luks`, `home /dev/sda3 /etc/luks-keys/home luks`, `swap /dev/sda2 /dev/urandom swap,cipher=aes-xts-plain64,size=256`, `data /dev/disk/by-uuid/… - luks,discard,nofail`, `vault PARTUUID=… none`. Name, device, key file (`none` / `-` / path), optional options. Optional `#` comment. Bare / prose / incomplete / neighbouring families (incl. fstab) stay out.

## [0.3.580] — 2026-10-24

View:

- **Function List** lists `/etc/fstab` entries: `UUID=… / ext4 defaults 0 1`, `/dev/sda1 /boot ext4 defaults 0 2`, `tmpfs /tmp tmpfs defaults 0 0`, `LABEL=home /home xfs defaults 0 2`, `//server/share /mnt/cifs cifs credentials=… 0 0`, `server:/export /mnt/nfs nfs defaults 0 0`, `/swapfile none swap sw 0 0`. Spec, mount point, type, options, optional dump + pass. Optional `#` comment. Bare / prose / incomplete / neighbouring families stay out.

## [0.3.579] — 2026-10-24

View:

- **Function List** lists `/etc/hosts.allow` / `/etc/hosts.deny` (tcp_wrappers) rules: `ALL: LOCAL`, `sshd: 192.168.1.0/24`, `ALL: .example.com EXCEPT bad.example.com`, `sshd,vsftpd: ALL`, `in.ftpd: ALL: spawn /bin/echo …`, `sshd: /etc/hosts.allow.sshd`, `sshd: @admins`. Daemon list then client list, colon-separated; optional third field = shell command. Optional `#` comment. pam_access `+/- : …` stays on access.conf. Bare / prose / incomplete / neighbouring families stay out.

## [0.3.578] — 2026-10-24

View:

- **Function List** lists `/etc/security/access.conf` (pam_access) rules: `+ : root : LOCAL`, `- : ALL EXCEPT wheel : ALL`, `+ : @admins : .example.com`, `+ : (wheel) : ALL`, `+:root:LOCAL`. Permission `+` / `-`, then users, then origins, colon-separated. Optional `#` comment. Bare / prose / incomplete / neighbouring families stay out.

## [0.3.577] — 2026-10-24

View:

- **Function List** lists `/etc/security/limits.conf` rules: `* soft nofile 1024`, `* hard nofile 2048`, `@student hard nproc 20`, `% hard maxlogins 4`, `%faculty soft maxlogins 2`, `ftp hard nproc 0`, `alice - priority 10`. Domain = user / `@group` / `*` / `%` / `%group`. Type = `soft` / `hard` / `-`. Item = stock ulimit names (`core` / `nofile` / `nproc` / `maxlogins` / …). Value = number / `unlimited` / `-`. Optional `#` comment. Bare / prose / incomplete / neighbouring families stay out.

## [0.3.576] — 2026-10-24

View:

- **Function List** lists sudoers rules: `#include /etc/sudoers.local`, `#includedir /etc/sudoers.d`, `@include …` / `@includedir …`, `Defaults env_reset`, `Defaults:alice !requiretty`, `Defaults@host timestamp_timeout=15`, `Defaults!/usr/bin/sudoedit !authenticate`, `User_Alias` / `Runas_Alias` / `Host_Alias` / `Cmnd_Alias NAME = …`, privilege lines `root ALL=(ALL:ALL) ALL`, `%sudo ALL=(ALL) ALL`, `alice ALL=(ALL) NOPASSWD: /usr/bin/systemctl`. Optional `#` comment after a rule. Bare keywords, prose (`root the ALL`), C `#include <stdio.h>`, Sass `@include mixin`, PAM rules, and incomplete lines stay out.

## [0.3.575] — 2026-10-24

View:

- **Function List** lists Linux-PAM `pam.conf` / `pam.d` rules: `auth required pam_unix.so`, `account requisite pam_unix.so`, `-password optional pam_gnome_keyring.so`, `session [success=1 default=ignore] pam_succeed_if.so …`, `login auth required pam_unix.so` (pam.conf with service), `auth include system-auth`, `auth substack password-auth`, `@include common-auth`. Type/control case-insensitive; optional leading `-` on type. Optional `#` comment. Bare types, prose (`auth the unix`), incomplete, wrong `@Include`, malformed bracket control, and non-PAM lines stay out.

## [0.3.574] — 2026-10-24

View:

- **Function List** lists Linux `audit.rules` / auditctl lines: `-w /etc/passwd -p wa -k identity`, `-a always,exit -F arch=b64 -S open -k access`, `-A never,task`, `-a exit,always` (either order), controls `-D` / `-b 8192` / `-f 1` / `-e 1` / `-r 0` / `-i` / `--loginuid-immutable`. Exact leading `-` option letter (or `--loginuid-immutable`). Optional `#` comment. Bare flags, prose (`-w the passwd`), wrong-case (`-W` / `-d`), incomplete (`-a always`), and non-audit lines stay out.

## [0.3.573] — 2026-10-24

View:

- **Function List** lists udev rules: `KERNEL=="sda"`, `SUBSYSTEM=="usb", ATTR{idVendor}=="1234", MODE="0666"`, `ACTION=="add", RUN+="/usr/local/bin/…"`, `LABEL="end"`, `GOTO="end"`, `ENV{DEVTYPE}=="disk"`. Match/assign keys with `==` / `!=` / `=` / `+=` / `-=` / `:=`. Optional `{…}` on ATTR / ATTRS / ENV / TAG / SYSCTL / SECLABEL / IMPORT. Optional `#` comment. Comma-separated fields. Bare keys, prose, wrong-case keys, and non-udev lines stay out.

## [0.3.572] — 2026-10-24

View:

- **Function List** lists logrotate path-block openers and `include`: `/var/log/messages {`, `/var/log/httpd/*log {`, `/var/log/a.log /var/log/b.log {`, `"/var/log/odd name.log" {`, `include /etc/logrotate.d`. Exact lowercase `include`. Path blocks must start with `/` or `"` and end with `{` (optional `#` after). Directives inside blocks (`rotate`, `weekly`, `postrotate` / `endscript`), nginx `location / {`, syslog-ng / named braces, rsyslog `include(file=…)`, prose (`include the logs`), and wrong case stay out.

## [0.3.571] — 2026-10-24

View:

- **Function List** lists rsyslog RainerScript objects and classic `$ModLoad` / `$IncludeConfig`: `module(load="imudp")`, `input(type="imudp" port="514")`, `action(type="omfile" file="…")`, `parser(name="json" type="mmjsonparse")`, `timezone(id="UTC" type="offset" offset="+00:00")`, `template(name="tpl" type="string" string="%msg%\\n")`, `ruleset(name="remote") {`, `global(workDirectory="/var/lib/rsyslog")`, `main_queue(queue.size="10000")`, `include(file="/etc/rsyslog.d/*.conf")`, `$ModLoad imudp`, `$IncludeConfig /etc/rsyslog.d/*.conf`. Exact lowercase object names. Params must include a known key (`load=` / `type=` / `name=` / `id=` / `file=` / `=`). Optional `{` or `#` after `)`. Bare `input(` / `module(`, Python `input("…")`, GraphQL `input UserInput {`, syslog-ng `source s_sys {`, prose, and wrong case stay out.


## [0.3.570] — 2026-10-24

View:

- **Function List** lists systemd-networkd `.network` / `.netdev` / `.link` sections: `[Match]`, `[Network]`, `[Link]`, `[Address]` / `[Route]` / `[Neighbor]` / `[NextHop]` / `[RoutingPolicyRule]`, `[DHCPv4]` / `[DHCPv6]` / `[DHCPPrefixDelegation]` / `[DHCPServer]` / `[DHCPServerStaticLease]`, `[IPv6AcceptRA]` / `[IPv6SendRA]` / `[IPv6Prefix]` / `[IPv6RoutePrefix]` / `[IPv6AddressLabel]` / `[IPv6PREF64Prefix]`, `[LLDP]` / `[CAN]` / `[Bridge]` / `[BridgeFDB]` / `[BridgeMDB]` / `[BridgeVLAN]` / `[BridgePort]`, `[NetDev]`, `[VLAN]` / `[MACVLAN]` / `[MACVTAP]` / `[IPVLAN]` / `[IPVTAP]` / `[VXLAN]` / `[GENEVE]` / `[L2TP]` / `[L2TPSession]` / `[MACsec]` / `[MACsecReceiveChannel]` / `[MACsecReceiveAssociation]`, `[Tunnel]` / `[FooOverUDP]` / `[VXCAN]` / `[Tun]` / `[Tap]`, `[WireGuard]` / `[WireGuardPeer]`, `[Bond]` / `[Xfrm]` / `[VRF]` / `[WLAN]` / `[BatmanAdvanced]`, `[SR-IOV]`. Exact Title Case. Optional `#` / `;` comment. Wrong case, NetworkManager lowercase, systemd unit `[Unit]` / `[Service]`, WireGuard `[Interface]` / `[Peer]` (veth `[Peer]` stays on WireGuard), SDDM / LightDM / GDM / AWS / Samba / Fail2ban / SSSD / Kerberos / PHP capitals, Cargo / Grafana tables, incomplete brackets, and key lines stay out.

## [0.3.569] — 2026-10-24

View:

- **Function List** lists NetworkManager connection keyfile sections: `[connection]`, `[ethernet]` / `[802-3-ethernet]`, `[wifi]` / `[802-11-wireless]`, `[wifi-security]` / `[802-11-wireless-security]`, `[ipv4]` / `[ipv6]`, `[proxy]`, `[bond]` / `[team]` / `[bridge]` (+ port/slave aliases), `[vlan]` / `[vxlan]` / `[macvlan]` / `[tun]` / `[tap]` / `[veth]` / `[vrf]` / `[ip-tunnel]` / `[dummy]` / `[loopback]` / `[generic]` / `[hsr]`, `[adsl]` / `[bluetooth]` / `[gsm]` / `[cdma]` / `[serial]` / `[ppp]` / `[pppoe]` / `[infiniband]` / `[olpc-mesh]` / `[wimax]` / `[wpan]` / `[6lowpan]` / `[wifi-p2p]` / `[macsec]` / `[wireguard]` / `[vpn]` / `[dcb]` / `[sriov]` / `[tc]` / `[hostname]` / `[user]` / `[ethtool]`, OVS `[ovs-bridge]` / `[ovs-port]` / `[ovs-interface]` / `[ovs-patch]` / `[ovs-dpdk]`. Exact lowercase. Optional `#` / `;` comment. Wrong case, SDDM / LightDM / GDM / AWS / Samba / Fail2ban / SSSD / Kerberos / WireGuard `[Interface]` / systemd / PHP capitals, Cargo / Grafana tables, incomplete brackets, and key lines stay out.

## [0.3.568] — 2026-10-24

View:

- **Function List** lists SDDM `sddm.conf` sections: `[General]`, `[Theme]`, `[Users]`, `[X11]`, `[Wayland]`, `[Autologin]`. Exact stock casing. Optional `#` / `;` comment. Wrong case, LightDM / GDM / AWS / Samba / Fail2ban / SSSD / Kerberos / WireGuard / systemd / PHP capitals, Cargo / Grafana tables, incomplete brackets, and key lines stay out.

## [0.3.567] — 2026-10-24

View:

- **Function List** lists GDM `custom.conf` / `gdm.conf` sections: `[daemon]`, `[security]`, `[xdmcp]`, `[chooser]`, `[debug]`, optional `[greeter]`. Exact lowercase. Optional `#` / `;` comment. Wrong case, LightDM / AWS / Samba / Fail2ban / SSSD / Kerberos / WireGuard / systemd / PHP capitals, Cargo / Grafana tables, incomplete brackets, and key lines stay out.

## [0.3.566] — 2026-10-24

View:

- **Function List** lists LightDM `lightdm.conf` sections: `[LightDM]`, `[Seat:*]` / `[Seat:seat0]`, `[XDMCPServer]`, `[VNCServer]`, obsolete `[SeatDefaults]`. Exact stock casing. Optional `#` / `;` comment. Wrong case, bare `[Seat:]`, AWS / Samba / Fail2ban / SSSD / Kerberos / WireGuard / systemd / PHP capitals, Cargo / Grafana tables, incomplete brackets, and key lines stay out.

## [0.3.565] — 2026-10-24

View:

- **Function List** lists Xorg `xorg.conf` / `xorg.conf.d` section openers: `Section "ServerLayout"`, `Section "Files"`, `Section "Module"`, `Section "InputDevice"`, `Section "InputClass"`, `Section "Device"`, `Section "VideoAdaptor"`, `Section "Monitor"`, `Section "Modes"`, `Section "Screen"`, `Section "Vendor"`, `Section "ServerFlags"`, `Section "Display"`, `Section "Extensions"`, `Section "DRI"`, plus obsolete `Section "Pointer"` / `Section "Keyboard"`. Exact `Section` casing and quoted stock names. Optional `#` comment. Wrong case, missing quotes, unknown names, `EndSection`, Apache `<…>` tags, and prose stay out.

## [0.3.564] — 2026-10-24

View:

- **Function List** lists Unbound `unbound.conf` / NSD `nsd.conf` top-level clauses: `server:`, `remote-control:`, Unbound `stub-zone:` / `forward-zone:` / `auth-zone:` / `view:` / `python:` / `dynlib:` / `dnscrypt:` / `cachedb:` / `dnstap:` / `rpz:`, NSD `verify:` / `key:` / `pattern:` / `zone:` / `tls-auth:`. Exact lowercase. Clause alone on the line (optional `#` comment). Same-line values, wrong case, nested attrs, BIND `zone "x" {`, nginx `server {`, and prose stay out.

## [0.3.563] — 2026-10-24

View:

- **Function List** lists SSSD `sssd.conf` sections: `[sssd]`, `[nss]`, `[pam]`, `[sudo]`, `[autofs]`, `[ssh]`, `[pac]`, `[ifp]`, `[secrets]`, `[session_recording]`, `[domain/NAME]`. Exact lowercase. Optional `#` / `;` comment. Wrong case, bare `[domain/]`, AWS / Samba / Fail2ban / Kerberos / WireGuard / systemd / PHP capitals, Cargo / Grafana tables, incomplete brackets, and key lines stay out.

## [0.3.562] — 2026-10-24

View:

- **Function List** lists Fail2ban `jail.conf` / `fail2ban.conf` / filter / action sections: `[DEFAULT]`, `[Definition]`, `[Init]`, `[Init?family=inet4]`, `[INCLUDES]`. Exact stock casing. Optional `#` / `;` comment. Free-form jail names (`[sshd]`), wrong case, AWS `[default]`, Samba / WireGuard / systemd / PHP capitals, Cargo / Grafana tables, incomplete brackets, and key lines stay out.

## [0.3.561] — 2026-10-24

View:

- **Function List** lists MIT Kerberos `krb5.conf` / `kdc.conf` sections: `[libdefaults]`, `[realms]`, `[domain_realm]`, `[capaths]`, `[appdefaults]`, `[logging]`, `[kdcdefaults]`, `[dbdefaults]`, `[dbmodules]`, `[otp]`, `[login]`, `[password_quality]`, `[plugins]`. Exact lowercase. Optional `#` / `;` comment. Wrong case, AWS / Samba / WireGuard / systemd / PHP capitals, Cargo / Grafana tables, incomplete brackets, and key lines stay out.

## [0.3.560] — 2026-10-24

View:

- **Function List** lists Samba `smb.conf` special sections: `[global]`, `[homes]`, `[printers]`, `[print$]`, `[IPC$]`, `[netlogon]`, `[sysvol]`. Optional `#` / `;` comment. Wrong case, free-form share names, Cargo / Grafana tables, WireGuard / systemd / PHP capitals, incomplete brackets, and key lines stay out.

## [0.3.559] — 2026-10-24

View:

- **Function List** lists OpenVPN inline blocks: `<ca>`, `<cert>`, `<key>`, `<dh>`, `<pkcs12>`, `<extra-certs>`, `<crl-verify>`, `<tls-auth>`, `<tls-crypt>`, `<tls-crypt-v2>`, `<auth-user-pass>`, `<http-proxy-user-pass>`, `<auth-gen-token-secret>`, `<peer-fingerprint>`, `<verify-hash>`, `<connection>`. Exact lowercase. Optional `#` or `;` after `>`. Closing tags, wrong case, plist `<key>name</key>`, and trailing payload stay out.

## [0.3.558] — 2026-10-24

View:

- **Function List** lists strongSwan / Libreswan `ipsec.conf` section headers: `config setup`, `conn %default`, `conn vpn`, `ca rootca`. Exact lowercase. Optional `#` after the name. `%` magics, idents, or quoted names. Wrong case, prose (`conn the`), bare keywords, and in-section settings stay out.

## [0.3.557] — 2026-10-24

View:

- **Function List** lists FreeRADIUS config block openers: `client localhost {`, `client 192.168.0.0/24 {`, `client "nas-1" {`, `home_server localhost {`, `home_server_pool my_auth_failover {`, `realm example.com {`, `server default {`, `modules {`, `instantiate {`, `listen {`, `policy {`, `files {`, `authorize {`, `authenticate {`, `preacct {`, `accounting {`, `session {`, `post-auth {`, `pre-proxy {`, `post-proxy {`, `recv-coa {`, `send-coa {`. Exact lowercase. Optional `#` after `{`. Wrong case, prose (`client the {`), bare keywords, nginx bare `server {`, HAProxy `listen name`, and in-block settings stay out.

## [0.3.556] — 2026-10-24

View:

- **Function List** lists dovecot.conf block openers: `protocol imap {`, `service auth {`, `passdb {`, `userdb {`, `namespace inbox {`, `mailbox Drafts {`, `plugin {`, `dict {`, `local_name mail.example.com {`, `inet_listener imap {` / `unix_listener auth {` / `fifo_listener x {`, `metric auth_success {`, `local {` / `local 192.168.0.0/24 {`, `remote {` / `remote 10.0.0.0/8 {`. Exact lowercase keywords. Optional `#` after `{`. Wrong case, prose (`protocol the {`), bare keywords, and in-block settings stay out.

## [0.3.555] — 2026-10-24

View:

- **Function List** lists syslog-ng.conf block openers: `source s_sys {`, `destination d_mesg {`, `filter f_kernel {`, `parser p_json {`, `rewrite r_set {`, `template t_demo {`, `log {`, `options {`, `block source s_tcp {` / `block destination d_file(…) {`. Exact lowercase. Optional `(…)` params after the name. Optional `#` after `{`. Wrong case, prose (`source the {`), bare keywords, Nim `template`, and in-block drivers stay out.

## [0.3.554] — 2026-10-24

View:

- **Function List** lists ISC dhcpd.conf block openers: `subnet 192.168.1.0 netmask 255.255.255.0 {`, `subnet6 2001:db8:1::/64 {`, `host printer {`, `group {` / `group lab {`, `shared-network office {`, `pool {`, `class "Vendor" {`, `failover peer "dhcp-failover" {`, `on commit {` / `on expiry {` / `on release {`. Exact lowercase. Optional `#` after `{`. Wrong case, prose (`host the {`), bare keywords, Java `class Foo {`, and in-block statements stay out.

## [0.3.553] — 2026-10-23

View:

- **Function List** lists keepalived.conf block openers: `global_defs {`, `static_ipaddress {`, `static_routes {`, `static_rules {`, `vrrp_instance VI_1 {`, `vrrp_script chk_http {`, `vrrp_sync_group VG1 {`, `vrrp_track_file tf1 {`, `vrrp_track_process nginx {`, `virtual_server 192.168.1.10 80 {`, `virtual_server fwmark 1 {`, `virtual_server group G1 {`. Exact lowercase. Optional `#` after `{`. Wrong case, prose (`vrrp_instance the {`), bare keywords, and in-block statements stay out.

## [0.3.552] — 2026-10-23

View:

- **Function List** lists nftables block openers: `table inet filter {`, `table ip nat {`, `chain input {`, `set blacklist {`, `map portfwd {`, `flowtable f {`, `counter http {`, `quota q {`, `limit rate_limit {`, `secmark m {`, `synproxy s {`, `meter flood {`, `ct helper ftp {`, `ct timeout t {`, `ct expectation e {`. Exact lowercase. Families: `ip` / `ip6` / `inet` / `arp` / `bridge` / `netdev`. Optional `#` after `{`. FlatBuffers `table Monster {`, nginx `map $a $b {`, wrong case, prose, bare keywords, and in-block statements stay out.

## [0.3.551] — 2026-10-23

View:

- **Function List** lists BIND / named.conf block openers: `options {`, `logging {`, `controls {`, `statistics-channels {`, `managed-keys {`, `trust-anchors {`, `zone "example.com" {` / `zone example.com IN {`, `view "internal" {`, `acl "trusted" {`, `key "ddns-key" {`, `server 192.0.2.1 {`, `primaries name {` / `masters name {`, `tls "name" {`, `http "name" {`, `dnssec-policy "name" {`, `parental-agents "name" {`. Exact lowercase keywords. Optional `#` after `{`. nginx bare `http {` / `server {`, wrong case, prose (`zone the team {`), bare keywords, and in-block statements stay out.

## [0.3.550] — 2026-10-23

View:

- **Function List** lists supervisord.conf sections: `[unix_http_server]`, `[inet_http_server]`, `[supervisord]`, `[supervisorctl]`, `[include]`, `[program:name]`, `[group:name]`, `[eventlistener:name]`, `[fcgi-program:name]`, `[rpcinterface:name]`. Exact lowercase fixed headers; named kinds need a non-empty segment. Optional `#` / `;` comment. Key lines (`command = …` / `autostart = true`), wrong case, Grafana / PHP / MySQL / Fluent Bit / WireGuard / Telegraf / Vector / AWS / git / systemd / Desktop Entry / tox / Cargo / EditorConfig tables, incomplete brackets, bare `[program:]`, and prose stay out.

## [0.3.549] — 2026-10-23

View:

- **Function List** lists Grafana.ini sections: `[server]`, `[database]`, `[auth]` / `[auth.github]`, `[log.console]`, `[unified_alerting]`, `[tracing.opentelemetry.otlp]`, and other stock `defaults.ini` headers. Exact lowercase. Optional `#` / `;` comment. Key lines (`http_port = 3000` / `type = sqlite3`), wrong case, PHP / MySQL / Fluent Bit / WireGuard / Telegraf / Vector / AWS / git / systemd / Desktop Entry / tox / Cargo / EditorConfig tables, incomplete brackets, and prose stay out.

## [0.3.548] — 2026-10-23

View:

- **Function List** lists Telegraf.conf plugin tables: `[[inputs.cpu]]`, `[[outputs.influxdb]]`, `[[processors.regex]]`, `[[aggregators.basicstats]]`, `[[secretstores.jose]]`, bare `[agent]` / `[global_tags]`. Exact lowercase. Optional `#` comment. Key lines (`interval = "10s"` / `percpu = true`), wrong case, Vector / Fluent Bit / Cargo / Pipfile tables, incomplete brackets, bare `[[inputs]]` without a plugin name, and prose stay out.

## [0.3.547] — 2026-10-23

View:

- **Function List** lists Vector.toml component tables: `[sources.name]`, `[transforms.name]`, `[sinks.name]`, `[enrichment_tables.name]`, `[tests.name]`, `[secrets.name]`, bare `[api]`. Exact lowercase kinds. Optional `#` comment. Key lines (`type = "stdin"` / `inputs = ["in"]`), wrong case, Fluent Bit / PHP / MySQL / WireGuard / Cargo / tox tables, incomplete brackets, bare `[sources]` without a name, and prose stay out.

## [0.3.546] — 2026-10-23

View:

- **Function List** lists Fluent Bit classic sections: `[SERVICE]`, `[INPUT]`, `[FILTER]`, `[OUTPUT]`, `[PARSER]`, `[MULTILINE_PARSER]`, `[CUSTOM]`, `[PROCESSOR]`, `[CHECK]`. Exact uppercase. Optional `#` / `;` comment. Key lines (`Name cpu` / `Flush 1`), wrong case, PHP / MySQL / WireGuard / AWS / git / systemd / Desktop Entry / tox / Cargo / EditorConfig tables, incomplete brackets, and prose stay out.

## [0.3.545] — 2026-10-23

View:

- **Function List** lists HAProxy section headers: `global`, `defaults` / `defaults name`, `traces`, `frontend name`, `backend name`, `listen name`, `resolvers name`, `cache name`, `peers name`, `mailers name`, `userlist name`, `http-errors name`, `ring name`, `program name`, `fcgi-app name`. Optional `#` comment. Exact lowercase. nginx `server {` / `listen 80;`, wrong case, prose (`frontend the team`), and in-section directives stay out.

## [0.3.544] — 2026-10-23

View:

- **Function List** lists Apache httpd section containers: `<VirtualHost *:80>`, `<Directory /var/www>`, `<DirectoryMatch "^/var/www">`, `<Files "*.php">`, `<FilesMatch "\.php$">`, `<Location /api>`, `<LocationMatch "^/api">`, `<IfModule mod_ssl.c>`, `<IfDefine SSL>`, `<If "…">`, `<ElseIf "…">`, `<Else>`, `<Proxy *>`, `<ProxyMatch …>`, `<RequireAll>`, `<RequireAny>`, `<RequireNone>`, `<Limit GET POST>`, `<LimitExcept GET>`, `<AuthnProviderAlias ldap myldap>`, `<Macro VHost $domain>`. Optional `#` comment after `>`. Tag name case-insensitive. Closing tags, HTML / Ant / unknown tags, bare containers that need args, and prose (`<Directory the team>`) stay out.

## [0.3.543] — 2026-10-23

View:

- **Function List** lists nginx config block openers: `events {`, `http {`, `server {`, `mail {`, `stream {`, `types {`, `upstream name {`, `location / {` (modifiers `=` / `~` / `~*` / `^~` / `@name`), `map $a $b {`, `geo $out {` / `geo $addr $out {`, `split_clients … $out {`, `limit_except GET {`. Optional `#` comment after `{`. Exact lowercase. Directives without `{` (`listen 80;`, `server_name …;`), wrong case, prose, and HCL / other braces stay out.

## [0.3.542] — 2026-10-23

View:

- **Function List** lists PHP.ini sections: `[PHP]`, `[CLI Server]`, `[Date]`, `[Session]`, `[opcache]`, `[MySQLi]`, `[mail function]`, plus common extensions (`[xdebug]`, `[apcu]`, …). Exact stock casing. Optional `#` / `;` comment. Key lines (`engine =` / `memory_limit =`), wrong case, MySQL / WireGuard / AWS / git / systemd / Desktop Entry / tox / Cargo / EditorConfig tables, incomplete brackets, and prose stay out.

## [0.3.541] — 2026-10-23

View:

- **Function List** lists MySQL / MariaDB option-file sections: `[mysqld]`, `[mysql]`, `[client]`, `[mysqldump]`, `[mariadb]`, `[client-server]`, versioned `[mysqld-8.0]` / `[mariadb-10.11]`, …. Optional `#` / `;` comment. Key lines (`port =`), wrong case, WireGuard / AWS / git / systemd / Desktop Entry / tox / Cargo / EditorConfig tables, incomplete brackets, and prose stay out.

## [0.3.540] — 2026-10-23

View:

- **Function List** lists WireGuard / `wg-quick` sections: `[Interface]`, `[Peer]`. Optional `#` / `;` comment. Key lines (`PrivateKey =` / `Address =` / `AllowedIPs =`), wrong case, AWS / git / systemd / Desktop Entry / tox / Cargo / EditorConfig tables, incomplete brackets, and prose stay out.

## [0.3.539] — 2026-10-23

View:

- **Function List** lists AWS config sections: `[default]`, `[profile name]`, `[sso-session name]`, `[services name]`, `[plugins]`. Optional `#` / `;` comment. Key lines (`region =`), bare `[profile]`, git / systemd / Desktop Entry / tox / Cargo / EditorConfig tables, incomplete brackets, and prose stay out.

## [0.3.538] — 2026-10-23

View:

- **Function List** lists SSH config `Host` / `Match` / `Include` lines: `Host github.com`, `Host * *.internal`, `Match host *.corp`, `Match all`, `Match exec "…"`, `Include ~/.ssh/config.d/*`. Optional `#` comment. Key lines (`HostName` / `User` / `IdentityFile` / `Port`), bare keywords, `Match the host` prose, and wrong case stay out.

## [0.3.537] — 2026-10-23

View:

- **Function List** lists Git config sections: `[core]`, `[user]`, `[remote "origin"]`, `[branch "main"]`, `[includeIf "gitdir:…"]`, `[filter "lfs"]`, `[url "…"]`, `[branch.main]`. Optional `#` / `;` comment. Known lowercase section names; optional quoted or dotted subsection. Key lines (`user.name =`), systemd / Desktop Entry / tox / Cargo / Pipfile / pyproject / EditorConfig tables, incomplete brackets, and prose stay out.

## [0.3.536] — 2026-10-23

View:

- **Function List** lists EditorConfig glob sections: `[*]`, `[*.rs]`, `[*.{js,ts}]`, `[lib/**/*.js]`, `[{package.json,.travis.yml}]`, `[src/main.rs]`. Optional `#` / `;` comment. Header must contain `*` / `?` / `{` or `/`. Key lines (`indent_size =`), bare `[Makefile]`, systemd / Desktop Entry / tox / Cargo / Pipfile / pyproject tables, incomplete brackets, and prose stay out.

## [0.3.535] — 2026-10-23

View:

- **Function List** lists systemd unit sections: `[Unit]`, `[Service]`, `[Install]`, `[Socket]`, `[Timer]`, `[Mount]`, `[Automount]`, `[Swap]`, `[Path]`, `[Slice]`, `[Scope]`. Optional `#` / `;` comment. Key lines (`Description=` / `ExecStart=`), Desktop Entry / tox / Cargo / Pipfile / pyproject tables, incomplete brackets, lowercase `[unit]`, network/journal sections, and prose stay out.

## [0.3.534] — 2026-10-23

View:

- **Function List** lists FreeDesktop Desktop Entry sections: `[Desktop Entry]`, `[Desktop Action …]` (action id alphanumeric / hyphen). Optional `#` / `;` comment. Key lines (`Name=` / `Exec=`), tox / Cargo / Pipfile / pyproject tables, incomplete brackets, bare `[Desktop Action]`, and prose stay out.

## [0.3.533] — 2026-10-23

View:

- **Function List** lists tox.ini sections: `[tox]`, `[testenv]`, `[testenv:…]` (e.g. `py39`, `lint`), `[gh-actions]`, `[gh-actions:…]`. Optional `#` / `;` comment. Key lines (`envlist =`), Cargo / Pipfile / pyproject / Desktop Entry tables, incomplete brackets, and prose stay out.

## [0.3.532] — 2026-10-23

View:

- **Function List** lists Pipfile (Pipenv) structural tables: `[[source]]`, `[packages]`, `[dev-packages]`, `[requires]`, `[scripts]`, `[pipenv]`. Optional `#` comment. Package assignment lines (`requests = "*"`), Cargo / pyproject tables, incomplete brackets, and prose stay out.

## [0.3.531] — 2026-10-23

View:

- **Function List** lists CocoaPods Podfile directives: `pod "…"` (optional version / kwargs), `target "…" do`, `abstract_target "…" do`, `use_frameworks!` / `use_modular_headers!` / `inhibit_all_warnings!` (optional kwargs), `install! "cocoapods"`, `pre_install` / `post_install` / `pre_integrate` / `post_integrate` hooks (optional `|installer|`). Optional `#` comment. Bare keywords, prose (`pod the alamofire`), and wrong case stay out. xmake `target("…")` stays on its path; Gemfile `gem` / Brewfile `brew` stay on theirs.

## [0.3.530] — 2026-10-23

View:

- **Function List** lists Homebrew Brewfile directives: `tap "…"`, `brew "…"` (optional kwargs), `cask "…"`, `mas "…", id: …` (bare quoted name also ok), `whalebrew "…"`, `vscode "…"`. Optional `#` comment. Bare keywords, prose (`brew the package`), `brew install …`, and wrong case stay out. Gemfile `gem` / Rake `task` stay on their paths.

## [0.3.529] — 2026-10-23

View:

- **Function List** lists Bundler Gemfile directives: `source "…"`, `ruby "…"`, `gem "…"` (optional version / kwargs), `group :development do` / `group :development, :test do`, `gemspec` / `gemspec path:`, `platforms :ruby do` / `platform :jruby do`, `git_source(:github)`, `plugin "…"`, `eval_gemfile "…"`. Optional `#` comment. Bare keywords, prose (`gem the rails`, `source the gems`), and wrong case stay out. Rake `task` stays on the Rake path.

## [0.3.528] — 2026-10-22

View:

- **Function List** lists go.work `use` directives: `use (`, `use ./foo`, `use ../bar`, `use .`, `use foo`, `use hello/world`, `use /abs/path`. Shares go.mod `go` / `toolchain` / `replace`. Optional `//` / `/*` comment. Bare `use`, prose (`use the team`), wrong case, and member lines inside a `use` block stay out.

## [0.3.527] — 2026-10-22

View:

- **Function List** lists go.mod directives: `module example.com/foo`, `go 1.22`, `toolchain go1.22.1` / `default`, `require (` / `require path v1.2.3`, `exclude (` / single-line, `replace (` / `replace old => new` (incl. `./` / `../` local), `retract (` / `retract v1.0.0` / `retract [v1.0.0, v1.0.1]`. Optional `//` / `/*` comment. JS `require('fs')`, bare keywords, prose (`module the team`), and dependency lines inside a block stay out.

## [0.3.526] — 2026-10-22

View:

- **Function List** lists composer.json structural keys: `"autoload": {`, `"autoload-dev": {`, `"scripts": {`, `"repositories": [` / `{`, `"config": {`, `"extra": {`, `"bin": [`, `"provide": {`, `"replace": {`, `"conflict": {`, `"suggest": {`. Optional trailing `,` and `//` / `/*` comment. `"require"` / `"require-dev"` and leaf keys (`"name"`, `"type"`, …) stay out. package.json `"bin": {` / `"bin": "…"` stay on the npm path.

## [0.3.525] — 2026-10-22

View:

- **Function List** lists package.json / deno.json structural keys: `"scripts": {`, `"bin": {` / `"bin": "…"`, `"exports": {`, `"imports": {`, `"workspaces": [` / `{`, `"contributes": {`, `"engines": {`, `"overrides": {`, `"resolutions": {`, `"typesVersions": {`, `"publishConfig": {`, `"packageManager": "…"`, `"tasks": {`, `"browser": {` / `"browser": "…"`. Optional trailing `,` and `//` / `/*` comment. `"dependencies"` / `"devDependencies"` / `"peerDependencies"` / `"optionalDependencies"` / `"bundledDependencies"` and leaf keys (`"name"`, `"version"`, …) stay out.

## [0.3.524] — 2026-10-22

View:

- **Function List** lists pyproject.toml structural tables: `[project]` / `[project.scripts]` / `[project.optional-dependencies]` / `[project.urls]` / `[project.entry-points.…]`, `[build-system]`, `[tool]` / `[tool.poetry]` / `[tool.ruff]` / `[tool.pytest.ini_options]` / …, `[dependency-groups]`. Optional trailing `#` comment. Incomplete brackets, array tables, and Cargo.toml `[package]` stay out.

## [0.3.523] — 2026-10-22

View:

- **Function List** lists Cargo.toml structural tables: `[package]`, `[lib]`, `[[bin]]` / `[[example]]` / `[[test]]` / `[[bench]]`, `[workspace]` / `[workspace.…]`, `[features]`, `[profile.…]`, `[lints]` / `[lints.…]`, `[patch.…]`, `[replace]`, `[package.metadata…]`, `[target.…]`. Optional trailing `#` comment. `[dependencies]` / `[dev-dependencies]` / `[build-dependencies]` stay out. Incomplete brackets and prose stay out.

## [0.3.522] — 2026-10-22

View:

- **Function List** lists Cabal `.cabal` stanzas: `library`, `library mylib`, `executable foo`, `test-suite tests`, `benchmark bench`, `foreign-library mylib`, `common shared`, `flag debug`, `source-repository head` / `this`. Space then component name (or bare `library`; `head` / `this` for source-repository). Optional trailing `--` comment. Meson / GN / Solidity brace or paren forms stay on their paths. Prose and bare keywords (except bare `library`) stay out.

## [0.3.521] — 2026-10-22

View:

- **Function List** lists Rake `task` / `multitask` / `file` / `directory` / `rule`: `task :default`, `task :default => [:test]`, `task "build" do`, `task migrate: :environment`, `multitask :parallel => [:a, :b]`, `file "out.txt" => ["in.txt"]`, `directory "tmp"`, `rule '.o' => '.c'`. Symbol or quoted name on the same line. Gradle `task hello {` / `task("hello")` stay on Gradle; xmake `rule("…")` stays on xmake. Prose and bare keywords stay out.

## [0.3.520] — 2026-10-22

View:

- **Function List** lists xmake.lua `target` / `option` / `package` / `rule` / `toolchain` / `task`: `target("app")`, `option("foo")`, `package("zlib")`, `rule("markdown")`, `toolchain("clang")`, `task("hello")`. Quoted name on the same line (single or double). Optional trailing Lua `--` comment. GN `target("type", "name")` stays on GN; Meson / Gradle keep their own shapes. Prose, bare keywords, and unquoted names stay out.

## [0.3.519] — 2026-10-22

View:

- **Function List** lists Premake `workspace` / `project` / `solution`: `workspace "App"`, `project "Lib"`, `solution "Old"` (Premake 4). Space then quoted name on the same line (single or double). Optional trailing Lua `--` comment. Meson `project('…')` stays on Meson. Prose, bare keywords, and call-paren forms stay out.

## [0.3.518] — 2026-10-22

View:

- **Function List** lists Apache Ant / MSBuild XML targets: `<target name="compile">`, `<project name="Foo" default="all">`, `<macrodef name="with-echo">`, `<Target Name="Build">`. Tag is case-insensitive. Quoted `name` / `Name` on the same line (other attrs may precede). Prose, bare tags, closing tags, and non-name elements stay out.

## [0.3.517] — 2026-10-22

View:

- **Function List** lists SCons builders: `Program('hello', 'hello.c')`, `SharedLibrary` / `StaticLibrary` / `Library` / `LoadableModule`, `Object` / `StaticObject` / `SharedObject`, `Command`, `Alias`, `Install` / `InstallAs` / `InstallVersionedLib`, `Package`, `Java` / `JavaJar`. Optional assignment and `env.` receiver. Quoted target on the same line (single or double). Meson single-quoted `library` / `shared_library` stay on Meson; GN brace forms stay on GN. Prose and bare keywords stay out.

## [0.3.516] — 2026-10-22

View:

- **Function List** lists Android.bp / Soong modules: `cc_binary {`, `cc_library` / `cc_library_shared` / `cc_library_static` / `cc_library_headers`, `cc_defaults`, `cc_test`, `cc_object`, `cc_genrule`, `cc_prebuilt_*`, `android_app` / `android_library` / `android_test` / `android_app_certificate`, `java_library` / `java_sdk_library` / `java_defaults`, `rust_binary` / `rust_library` / `rust_proc_macro` / `rust_ffi*`, `python_binary_host`, `sh_binary`, `filegroup`, `genrule` / `gensrcs`, `apex` / `apex_key`, `prebuilt_etc`, `phony`. Optional same-line `name:`. Bazel `cc_binary(name = …)` stays on Bazel. Prose, bare keywords, and wrong case stay out.

## [0.3.515] — 2026-10-22

View:

- **Function List** lists BitBake / Yocto `do_*` tasks: `do_compile()`, `do_configure()`, `do_install()`, `do_fetch()` / `do_unpack()` / `do_patch()`, `do_deploy()`, `do_package()`, `do_populate_sysroot()`, `do_rootfs()`, optional `python` / `fakeroot`, legacy `_append` / `_prepend` and `:append` / `:prepend` / `:class-native` overrides. Prose, bare keywords, wrong case, empty `do_()`, and anonymous `python()` stay out.

## [0.3.514] — 2026-10-22

View:

- **Function List** lists Gentoo ebuild phase functions: `src_unpack()`, `src_prepare()`, `src_configure()`, `src_compile()`, `src_test()`, `src_install()`, `pkg_pretend()`, `pkg_setup()`, `pkg_nofetch()`, `pkg_preinst()`, `pkg_postinst()`, `pkg_prerm()`, `pkg_postrm()`, `pkg_config()`, `pkg_info()`. Prose, bare keywords, wrong case, and unknown names stay out.

## [0.3.513] — 2026-10-22

View:

- **Function List** lists Arch PKGBUILD / Alpine APKBUILD phase functions: `prepare()`, `build()`, `check()`, `package()`, `pkgver()`, and split `package_foo()` / `package_foo-bar()`. Prose, bare keywords, and unknown names stay out.

## [0.3.512] — 2026-10-22

View:

- **Function List** lists Debian `debian/rules` debhelper overrides / hooks: `override_dh_auto_build:`, `override_dh_installdocs:`, `override_dh_auto_test-arch:`, `execute_before_dh_auto_configure:`, `execute_after_dh_install --remaining-packages:`. Prose, bare prefixes, wrong case, and `:=` assignments stay out.

## [0.3.511] — 2026-10-22

View:

- **Function List** lists Autoconf / Automake / Libtool / Autoconf Archive / pkg-config macros: `AC_INIT` / `AC_PROG_CC` / `AC_CONFIG_FILES` / `AC_OUTPUT`, `AM_INIT_AUTOMAKE` / `AM_CONDITIONAL`, `LT_INIT`, `AS_IF`, `AX_*`, `PKG_CHECK_MODULES` / `PKG_PROG_PKG_CONFIG`. Prose and bare prefixes stay out.

## [0.3.510] — 2026-10-22

View:

- **Function List** lists RPM `.spec` `%prep` / `%build` / `%install` / `%check` / `%clean` / `%files` / `%changelog` / `%description` / `%package` / scriptlets (`%pre`…`%posttrans`) / `%trigger*` / `%define` / `%global` / `%undefine` / `%bcond_with` / `%bcond_without`. `%{…}` expansions and Makefile `%.o:` stay out.

## [0.3.509] — 2026-10-22

View:

- **Function List** lists Earthfile `VERSION 0.8`, `FROM alpine:3.18` / `FROM +base` (optional `--platform=…`), `SAVE ARTIFACT` / `SAVE IMAGE`, `BUILD +target`, `IMPORT github.com/…`, `GIT CLONE …`, `LOCALLY`, `PROJECT …`, `COMMAND` / `COMMAND --foo=STRING`, and `DO +INSTALL`. Bare targets (`build:`) stay out. Dockerfile `FROM … AS stage` stays on Dockerfile.

## [0.3.508] — 2026-10-22

View:

- **Function List** lists Justfile `[private]` / `[unix]` / `[confirm("…")]` attributes, `alias f := foo`, `set dotenv-load` / `set shell := […]`, `export FOO := "bar"`, `import "foo.just"`, quiet `@build:` / `@build: clean`, and parameterized recipes (`test *args:`, `hello name="world":`). Bare `build:` without `@` or params stays out. Makefile / Ninja keep their own shapes.

## [0.3.507] — 2026-10-22

View:

- **Function List** lists Makefile `.PHONY: clean`, `.SUFFIXES: .c .o`, other special targets, pattern rules `%.o: %.c`, `include` / `-include` / `sinclude`, and `define NAME` / `endef`. Assignments (`FOO =`, `FOO :=`) stay out. Prose and bare keywords stay out. Ninja keeps its own shapes.

## [0.3.506] — 2026-10-22

View:

- **Function List** lists Ninja `rule cc`, `build foo.o: cc foo.c`, `pool link_pool`, and `default app` (`build out1 out2: link a.o | order || dyndep`). JS/TS `default export` stays out. Prose and bare keywords stay out. Gradle / Meson / GN / Bazel keep their own shapes.

## [0.3.505] — 2026-10-22

View:

- **Function List** lists Gradle `task hello {` / `task hello(type: Copy) {` / `task("hello") {`, `tasks.register("hello")` / `tasks.named("build")` / `tasks.create("x")` / `tasks.registering("hello")`, and Kotlin `val hello by tasks.registering` / `val hello by tasks.creating`. Ada `task Buffer is` stays on Ada. Prose and bare keywords stay out.

## [0.3.504] — 2026-10-22

View:

- **Function List** lists CMake `add_executable` / `add_library` / `add_custom_target` / `add_test` / `project` alongside `function` / `macro` (`add_executable(app main.cpp)`, `add_library(lib STATIC)`, `add_library(Foo::Bar ALIAS Foo)`, `add_custom_target(docs)`, `add_test(NAME t COMMAND app)`, `add_test(t app)`, `project(MyApp)`). Same-line first name (optional `Foo::Bar`). Prose and bare keywords stay out. Meson / GN / Bazel keep their own shapes.

## [0.3.503] — 2026-10-22

View:

- **Function List** lists Bazel / Starlark BUILD rules with same-line `name = "…"` / `name = '…'` (`cc_binary(name = "hello")`, `java_library(name = "lib")`, `py_binary(name = "tool")`, `go_library(name = "pkg")`, `rust_library(name = "mylib")`, `proto_library(name = "api")`, `android_library` / `kt_jvm_library` / `objc_library` / `scala_library` / `sh_binary`, `filegroup` / `genrule` / `alias` / `test_suite` / `config_setting` / `constraint_setting` / `constraint_value` / `platform` / `toolchain`, `lib = cc_library(name = "lib")`). Multi-line rules with `name` on a following line stay out. Prose and bare keywords stay out.

## [0.3.502] — 2026-10-22

View:

- **Function List** lists GN `executable` / `shared_library` / `static_library` / `source_set` / `loadable_module` / `group` / `copy` / `action` / `action_foreach` / `generated_file` / `bundle_data` / `create_bundle` / `config` / `template` / `toolchain` / `rust_library` / `rust_proc_macro` / `rust_executable` / `rust_static_library` / `rust_shared_library` / `rust_test` / `target("type", "name")` (`executable("chrome") {`, `source_set("base") {`, `lib = shared_library("foo") {`, `target("executable", "tool") {`). Double-quoted name on the same line. Meson single-quoted forms stay on Meson. Prose and bare keywords stay out.

## [0.3.501] — 2026-10-22

View:

- **Function List** lists Meson `project` / `executable` / `library` / `shared_library` / `static_library` / `both_libraries` / `shared_module` / `jar` / `custom_target` / `run_target` / `alias_target` / `subproject` (`project('app', 'c')`, `executable('prog', 'main.c')`, `lib = shared_library('foo', 'foo.c')`). Single-quoted name on the same line. `test(` / `benchmark(` stay out (Jest / other runners). Prose and bare keywords stay out.

## [0.3.500] — 2026-10-22

View:

- **Function List** lists Dockerfile `FROM image:tag AS stage` / `FROM scratch AS export` (optional `ONBUILD`, `--platform=…`). Untagged bare names and SQL `FROM users AS u` stay out (image must look like a ref: `:`, `/`, `@`, `$`, or `scratch`).

## [0.3.499] — 2026-10-21

View:

- **Function List** lists Vimscript `function!` / `def!` / `augroup` / `command` (`function! s:Foo(`, `def! Bar(`, `augroup mygroup`, `command! -nargs=1 MyCmd`). Prose and bare keywords stay out. Plain `function` / `def` stay on the generic prefix keys.

## [0.3.498] — 2026-10-21

View:

- **Function List** lists Tcl `oo::class create` / `oo::define` / `coroutine` (`oo::class create Greeter`, `oo::define Greeter`, `coroutine tick apply {…} {…}`). Prose and bare keywords stay out. `proc` / `namespace` stay on the generic prefix keys.

## [0.3.497] — 2026-10-21

View:

- **Function List** lists Fortran `program` / `block data` / `submodule` (`program Main`, `block data`, `block data Init`, `submodule (Parent) Child`). Prose and bare keywords stay out. `subroutine` / `function` / `module` / `interface` stay on the generic prefix keys.

## [0.3.496] — 2026-10-21

View:

- **Function List** lists Ada `task` / `protected` (`task Buffer is`, `task type Worker is`, `task body Buffer is`, `protected Counter is`, `protected type Lock is`, `protected body Counter is`; optional discriminant `(…)` before `is`). Verilog `task do_reset;` stays on Verilog. Prose and bare keywords stay out. `procedure` / `function` / `package` stay on the generic prefix keys.

## [0.3.495] — 2026-10-21

View:

- **Function List** lists D `unittest` / `mixin template` / `static this` / `static ~this` (`unittest {`, `unittest "add" {`, `mixin template Foo()`, `static this()`, `shared static ~this()`). Prose and bare keywords stay out. Nim `template` stays on Nim. `class` / `struct` / `interface` / `enum` stay on the generic prefix keys.

## [0.3.494] — 2026-10-21

View:

- **Function List** lists Standard ML `structure` / `signature` / `functor` (`structure Foo =`, `structure Foo :> FOO =`, `signature FOO = sig`, `functor Make (S: ORD) =`). Lean `structure Point where` stays on Lean. Smithy `structure City {` stays on Smithy. Prose and bare keywords stay out. `fun` / `datatype` / `module` stay on generic / other paths.

## [0.3.493] — 2026-10-21

View:

- **Function List** lists Nim `converter` / `template` (`converter toBool(x: int): bool =`, `template max(x, y): untyped =`, optional export `*` / `[T]`). C++ `template<…>` stays on the template-strip path. Prose and bare keywords stay out. `proc` / `method` / `iterator` / `func` / `macro` stay on the generic prefix keys.

## [0.3.492] — 2026-10-21

View:

- **Function List** lists Cython `cdef` / `cpdef` functions and types (`cdef int add(`, `cpdef float mul(`, `cdef class Point:`, `cdef packed struct Foo:`, `cdef enum Color:`, `cdef extern from "h":`), plus `ctypedef int my_int` / `ctypedef struct Foo:` / `ctypedef fused numeric:`. Optional modifiers `inline` / `nogil` / `gil` / `api` / `public` / `readonly`. Prose and bare keywords stay out. Plain `def` / `class` / `struct` / `typedef` stay on the generic prefix keys.

## [0.3.491] — 2026-10-21

View:

- **Function List** lists Pony `be` / `new` methods with a `=>` body (`be ping()`, `new create(env: Env) =>`, optional capability `iso` / `trn` / `ref` / `val` / `box` / `tag`) and Capitalized `primitive Name` (`primitive Foo`, `primitive Bar[A]`, `primitive Nil is Iterator[None]`). JS `new Foo()` and Verilog `primitive udp_and (` stay out. `actor` / `class` / `trait` / `interface` / `type` / `fun` stay on the generic prefix keys.

## [0.3.490] — 2026-10-21

View:

- **Function List** lists Metal Shading Language `vertex` / `fragment` / `kernel` stage entry points (`vertex float4 main(`, `fragment half4 main(`), plus `device` / `constant` / `threadgroup` / `thread` / `threadgroup_imageblock` storage (`device float *buf;`, `constant float coeffs[8];`, `threadgroup float tile[64];`), plus Metal texture / sampler resources (`texture2d<float> color;`, `sampler samp;`, `depth2d<float> depthTex;`). Optional `static` / `inline`. Bare `kernel` also matches OpenCL. `[[vertex]]` / `[[fragment]]` attributes strip then typed-fn. Prose and bare keywords stay out. Plain `void main(` / `struct` stay on the generic / typed-fn paths.

## [0.3.489] — 2026-10-21

View:

- **Function List** lists OpenCL `__kernel` / bare `kernel` entry points (`__kernel void vector_add(`, `kernel void saxpy(`), plus `__global` / `__local` / `__constant` / `__private` storage (`__constant float coeffs[8];`, `__local float tile[64];`, `__global float *buf;`). Optional `static` / `inline` and OpenCL vector / image / sampler types. Distinct from CUDA `__global__` / `__constant__` (trailing `__`). Prose and bare keywords stay out. Plain `void main(` / `struct` stay on the generic / typed-fn paths.

## [0.3.488] — 2026-10-21

View:

- **Function List** lists CUDA `__global__` / `__device__` / `__host__` kernels and device functions (`__global__ void saxpy(`, `__device__ float add(`, `__host__ __device__ float clamp(`), plus `__shared__` / `__constant__` / `__managed__` storage and `__device__` variables (`__shared__ float tile[64];`, `__constant__ float coeffs[8];`, `__managed__ int counter;`). Optional `__forceinline__` / `__noinline__`. Prose and bare keywords stay out. Plain `void main(` / `struct` stay on the generic / typed-fn paths. GLSL `shared` / HLSL `groupshared` stay on their paths.

## [0.3.487] — 2026-10-21

View:

- **Function List** lists GLSL `layout` / `uniform` / `in` / `out` / `inout` / `attribute` / `varying` / `shared` / `buffer` / `precision` (`layout(location = 0) in vec3 position;`, `layout(local_size_x = 8) in;`, `uniform mat4 mvp;`, `uniform Matrices {`, `out vec4 fragColor;`, `attribute vec3 pos;`, `varying vec2 uv;`, `shared float sData[64];`, `buffer Particles {`, `precision mediump float;`). Optional interpolation / memory / precision qualifiers. Prose and bare keywords stay out. `struct` / typed C entry points stay on the generic / typed-fn paths. HLSL / WGSL stay on their paths.

## [0.3.486] — 2026-10-21

View:

- **Function List** lists HLSL `cbuffer` / `tbuffer` / `Texture2D` / `RWStructuredBuffer` / `SamplerState` / `groupshared` (`cbuffer PerFrame : register(b0) {`, `Texture2D<float4> colorMap;`, `RWStructuredBuffer<Particle> particles : register(u0);`, `SamplerState linearSamp;`, `groupshared float sData[64];`). Optional `<T>` on resource types. Prose and bare keywords stay out. `struct` / typed C entry points stay on the generic / typed-fn paths. WGSL `var` / `const` stay on WGSL.

## [0.3.485] — 2026-10-21

View:

- **Function List** lists WGSL `var` / `const` / `override` / `enable` / `requires` / `diagnostic` (`var<private> counter: i32`, `var tex: texture_2d<f32>`, `const PI: f32 = 3.14`, `override wg: u32 = 64`, `enable f16;`, `requires subgroups;`, `diagnostic(off, derivative_uniformity);`). Optional leading `@group` / `@binding` / `@fragment`. JS `var x = 1` / `const x = 1` stay out. Zig `const Name = struct` stays on Zig. Thrift typed `const` stays on Thrift. `fn` / `struct` stay on the generic prefix keys. Crystal `alias` stays on Crystal.

## [0.3.484] — 2026-10-21

View:

- **Function List** lists WebAssembly Text `(module` / `(func $name` / `(type $t` / `(memory` / `(table` / `(global` / `(import` / `(export` / `(start` / `(tag` / `(elem` / `(data` (`(module $m`, `(func $add (param i32 i32) (result i32)`, `(func (export "add") …`, `(type $t (func …))`, `(memory 1)`, `(table $tbl 2 funcref)`, `(global $g (mut i32) …)`, `(import "env" "memory" (memory 1))`, `(export "add" (func $add))`, `(start $main)`, `(tag $e (param i32))`). Lisp `(defun` / `(defn` stay on the Lisp path. WIT `world` / `variant` / `flags` stay on the WIT path. Prose, bare `(func`, and `;;` comments stay out.

## [0.3.483] — 2026-10-21

View:

- **Function List** lists Azure Bicep `param` / `resource` / `output` / `targetScope` / `metadata` (`param location string`, `resource stg 'Microsoft.Storage/…@2022-09-01' = {`, `resource stg '…' existing = {`, `output id string = stg.id`, `targetScope = 'resourceGroup'`, `metadata description = '…'`). Optional leading `@description(…)` / `@secure()` decorators. Prose and bare keywords stay out. HCL `resource "aws_instance"` stays on the HCL path. `module` / `type` / `func` stay on the generic prefix keys.

## [0.3.482] — 2026-10-21

View:

- **Function List** lists Odin `name :: proc` / `:: struct` / `:: enum` / `:: union` / `:: bit_set` / `:: bit_field` / `:: distinct` (`main :: proc() {`, `Person :: struct {`, `Color :: enum u8 {`, `Flags :: bit_set[Color]`, `MyInt :: distinct int`). Optional leading `@(export)` / `@(private)` attrs. Value consts (`FOO :: 123`) and path uses (`std::vector`, `JSON::Field`) stay out. `package` stays on the generic prefix key.

## [0.3.481] — 2026-10-21

View:

- **Function List** lists Apache Thrift `exception` / `senum` / typed `const` (`exception InvalidOperation {`, `senum Color {`, `const i32 INT32CONSTANT = 9853`, `const list<i32> LIST = [1, 2]`, `const map<string,string> MAPCONSTANT = {"a":"b"}`). Builtin / container types only so C `const int x =` stays out. Prose and bare keywords stay out. `struct` / `enum` / `union` / `service` / `typedef` / `namespace` stay on the generic prefix keys.

## [0.3.480] — 2026-10-21

View:

- **Function List** lists Crystal `annotation` / `lib` / `alias` (`annotation MyAnn`, `annotation JSON::Field`, `lib LibC`, `alias Callback = Int32 -> String`, `alias Nested = Foo::Bar`). Path names may use `::`. Shell-style `alias ls='…'` stays out. Prose and bare keywords stay out. `macro` / `fun` / `class` / `module` / `struct` / `enum` stay on the generic prefix keys. Solidity `library Math` still lists on the Solidity path.

## [0.3.479] — 2026-10-21

View:

- **Function List** lists Elm `port name :`, `port module Name`, and `effect module Name` (`port sendMessage : String -> Cmd msg`, `port module Main exposing (..)`, `port module Nested.Main exposing (main)`, `effect module WebSocket where { command = MyCmd }`). Module names must be Cap.Word (dots ok). Port names need `:` soon. Prose and bare `port` / `effect` stay out. Plain `module Name` stays on the generic prefix keys.

## [0.3.478] — 2026-10-21

View:

- **Function List** lists Gleam `opaque type` / `pub opaque type` (`opaque type User {`, `pub opaque type Id {`, `opaque type Queue(a) {`). Lowercase names need `{` or `(` soon. Prose and bare `opaque` / `opaque type` stay out. Zig `const O = opaque {}` stays on the Zig path. Plain `type` / `pub type` stay on the generic prefix keys.

## [0.3.477] — 2026-10-21

View:

- **Function List** lists Mercury / Prolog `:-` directives (`:- module foo.`, `:- module(foo, [bar/1]).`, `:- interface.`, `:- implementation.`, `:- type list(T) --->`, `:- pred append(`, `:- func length(`, `:- mode append(`, `:- inst list_skel ==`, `:- pragma foreign_decl(`, `:- import_module list.`, `:- use_module require.`, `:- include_module helpers.`, `:- typeclass foo(T)`, `:- instance foo(int)`, `:- initialise init/0.` / `:- initialize`, `:- finalise` / `:- finalize`, `:- mutable(counter, …)`, `:- dynamic foo/1.`, `:- multifile bar/2.`, `:- discontiguous baz/1.`, `:- meta_predicate map(2, +, -).`). Lowercase names need `.` / `(` / `=` / `-` / `:` / `%` / `--` / `/` soon after. Bare `:-`, prose such as `:- type the list`, glued `:-module`, and unknown words after `:-` stay out. Idris `implementation` without `:-` stays on the Idris path.

## [0.3.476] — 2026-10-21

View:

- **Function List** lists Idris `implementation` / `parameters`, plus optional `public export` / `export` / `private` before those or a generic key (`implementation Show Nat where`, `implementation [myShow] Show Nat where`, `implementation eq a where`, `implementation (Cast a b) => Cast (List a) (List b) where`, `parameters (n : Nat)`, `parameters {n : Nat}`, `export implementation Show Nat where`, `public export interface Eq a where`). Lowercase interface names need `where` / `(` / `{` / `[` soon, or a single-letter / uppercase next token. Prose and bare keywords stay out. `interface` / `data` / `record` / `namespace` stay on the generic prefix keys. Objective-C `@implementation` stays on the ObjC path.

## [0.3.475] — 2026-10-21

View:

- **Function List** lists Agda `postulate` / `primitive` / `variable` / `infix` / `infixl` / `infixr`, plus Agda-style `data Name : Set where` for names Haskell `data` misses (lowercase / mixfix operators / non-ASCII) (`postulate foo : Nat`, `postulate ∞ : Set`, `primitive primFloat`, `variable A : Set`, `variable {A : Set}`, `infix 4 _≡_`, `infixl 6 _+_`, `infixr 5 _∷_`, `data nat : Set where`, `data ℕ : Set where`, `data _≡_ {A : Set} (x : A) : A → Set where`). Prose and bare keywords stay out. `module` / `record` / `macro` stay on the generic prefix keys. Kotlin `data class` still lists as a class.

## [0.3.474] — 2026-10-21

View:

- **Function List** lists Isabelle/HOL `locale` / `abbreviation` / `primrec` / `typedef` / `axiomatization` / `interpretation`, plus `theorem` / `lemma` / `corollary` / `proposition` / `definition` / `datatype` / `inductive` / `coinductive` with Isabelle tails (`where` / `::` / `=` / `[simp]` / `:`) (`locale Group =`, `abbreviation id where`, `primrec fact where`, `typedef name =`, `axiomatization where`, `interpretation L =`, `theorem app_nil[simp]:`, `definition id where`, `datatype 'a list =`, `inductive even where`). Optional leading type vars. Lowercase names need `where` / `::` / `=` / `[` / `:`. Prose and bare keywords stay out. `fun` / `function` / `record` stay on the generic prefix keys.

## [0.3.473] — 2026-10-21

View:

- **Function List** lists TLA+ `---- MODULE Spec ----` / `MODULE Spec`, `THEOREM Name ==`, `LEMMA Name ==`, `ASSUME Name`, and `COROLLARY Name ==` (`THEOREM TypeOK == x \\in Nat`, `THEOREM Spec => []TypeOK`, `LEMMA Inv == TypeOK`, `ASSUME N \\in Nat`, `ASSUME N ∈ Nat`). Lowercase names need `==` / `=>` / `\\` / a non-identifier start. Dashed-banner prose and bare keywords stay out. Plain `module Name` still matches the generic prefix key. Sass/CSS `@…` keyword probes no longer panic on mid-line unicode.

## [0.3.472] — 2026-10-20

View:

- **Function List** lists Alloy `sig` / `pred` / `fact` / `assert` (`sig Person {`, `abstract sig Node extends Object {`, `lone sig Root {`, `pred connected[p: Person] {`, `fact Acyclic {`, `fact {`, `assert NoOrphans {`; optional leading `abstract` / `lone` / `one` / `some` / `var` before `sig`). Lowercase names need `{` / `[` / `(` / `extends` / `,`. Prose and bare keywords stay out. `fun` / `enum` stay on the generic prefix keys.

## [0.3.471] — 2026-10-20

View:

- **Function List** lists Lean 4 `theorem` / `lemma` / `example` / `axiom` / `abbrev` / `opaque` / `inductive` / `structure` and `instance` (`theorem add_comm (…) :`, `structure Point where`, `inductive List where`, `axiom em :`, `abbrev Id :=`, `opaque mystery : Nat`, `instance : Add Nat where`, `instance [Monad m] : …`, `example : 1 + 1 = 2 :=`; optional leading `noncomputable` / `partial` / `private` / `protected` / `public` / `unsafe`; names may include `'`). Lowercase names need `:` / `(` / `{` / `[` / `where`. Prose and bare keywords stay out. `def` / `class` stay on the generic prefix keys. Function List keyword scan no longer panics on mid-line unicode when probing ASCII keywords.

## [0.3.470] — 2026-10-20

View:

- **Function List** lists Coq `Theorem` / `Lemma` / `Definition` / `Inductive` / `CoInductive` / `Fixpoint` / `CoFixpoint` / `Corollary` / `Example` / `Fact` / `Remark` / `Proposition` (`Theorem app_nil_r :`, `Definition length :=`, `Inductive nat : Type :=`, `Fixpoint map {A} (…)`; optional leading `Local` / `Global`; names may include `'`). Lowercase names need `:` / `(` / `{`. Prose and bare keywords stay out. `Record` / `Module` / `Class` stay on the generic prefix keys.

## [0.3.469] — 2026-10-20

View:

- **Function List** lists Dafny `lemma` / `predicate` / `datatype` / `codatatype` (`lemma LemmaAppend`, `predicate Sorted(s: seq<int>)`, `datatype List = Nil | Cons(...)`, `codatatype Stream = Cons(...)`; optional leading `ghost`). Lowercase names need `(` / `<` / `=` / `{`. `lemma the proof`, `predicate the property`, `datatype the list`, and bare keywords stay out. `method` / `function` / `class` / `trait` / `iterator` stay on the generic prefix keys.

## [0.3.468] — 2026-10-20

View:

- **Function List** lists SystemVerilog `covergroup` / `property` / `sequence` / `clocking` (`covergroup cg;`, `covergroup cg @(posedge clk);`, `property p_req;`, `sequence s_req;`, `clocking cb @(posedge clk);`). Builds on the existing Verilog `task` / `macromodule` / `program` / `primitive` / `checker` scan. `covergroup the bins`, `property the house`, `sequence the events`, `clocking the bus`, and bare keywords stay out.

## [0.3.467] — 2026-10-20

View:

- **Function List** lists Raku / Perl 6 `grammar` / `token` / `rule` / `regex` / `role` (`grammar JSON {`, `token number {`, `rule object {`, `regex ident {`, `role Mixable {`, `role Mixable[::T]`). Lowercase names need `{` or `[`. `grammar the json`, `token the number`, `rule the object`, `regex the ident`, `role the mixin`, and bare keywords stay out.

## [0.3.466] — 2026-10-20

View:

- **Function List** lists WASM WIT `world` / `variant` / `flags` (`world imports {`, `variant error-code {`, `flags descriptor-flags {`). Kebab-case names ok. `world the imports`, `variant the error`, `flags the bits`, and bare keywords stay out. `interface` / `record` / `func` / `package` stay on generic keys; `resource` stays on Smithy.

## [0.3.465] — 2026-10-20

View:

- **Function List** lists Smithy `operation` / `structure` / `resource` / `intEnum` (`operation GetCurrentTime {`, `structure City {`, `resource Forecast {`, `intEnum Severity {`). `operation the request`, `structure the city`, `resource the forecast`, `intEnum the severity`, and bare keywords stay out. `service` / `enum` / `union` stay on protobuf or generic keys. HCL quoted `resource "…"` stays on the HCL path.

## [0.3.464] — 2026-10-20

View:

- **Function List** lists Prisma `model` / `datasource` / `generator` / `view` (`model User {`, `datasource db {`, `generator client {`, `view UserInfo {`). `model the user`, `datasource the database`, `generator of code`, `view the mountains`, and bare keywords stay out. `enum` / `type` stay on the generic prefix keys.

## [0.3.463] — 2026-10-20

View:

- **Function List** lists FlatBuffers `table` / `rpc_service` (`table Monster {`, `rpc_service MonsterStorage {`). `table of contents`, `table the schema`, `rpc_service the storage`, and bare keywords stay out. `struct` / `enum` / `union` / `namespace` stay on the generic prefix keys.

## [0.3.462] — 2026-10-20

View:

- **Function List** lists PowerShell `filter` / `workflow` / `configuration` (`filter Get-Odd`, `filter Get-Odd($x) {`, `workflow Invoke-Job {`, `configuration WebSite {`). `filter the team`, `workflow the job`, `configuration the site`, and bare keywords stay out. `function` / `class` / `enum` stay on the generic prefix keys.

## [0.3.461] — 2026-10-20

View:

- **Function List** lists CSS `@property` / `@layer` / `@counter-style` (`@property --accent`, `@layer utilities`, `@layer theme, layout`, `@layer framework.layout`, `@counter-style thumbs`). `@property color`, `@layer {`, `@layer;`, `@counter-style the list`, and bare keywords stay out.

## [0.3.460] — 2026-10-20

View:

- **Function List** lists Sass / SCSS `@mixin` / `@function` / `@keyframes` (`@mixin flex-center`, `@mixin flex-center($dir)`, `@function rem($px)`, `@keyframes fade-in`). `@mixin the team`, `@include`, `@media`, and a bare `@mixin` / `@function` stay out.

## [0.3.459] — 2026-10-20

View:

- **Function List** lists Erlang `-module` / `-export` / `-export_type` / `-record` / `-spec` / `-callback` / `-type` / `-opaque` and `name(...) ->` clauses (`-module(foo).`, `-export([hello/0]).`, `-record(person, {name})`, `hello() -> ok.`, `inc(X) when is_integer(X) -> X.`). Bare attributes, prose, and uppercase heads stay out.

## [0.3.458] — 2026-10-20

View:

- **Function List** lists Ruby `attr_reader` / `attr_writer` / `attr_accessor` (`attr_reader :name`, `attr_accessor :x, :y`, `attr_writer(:z)`, `attr_reader :ready?`, `attr_writer :save!`). `attr_reader the name`, `attr_reader :`, and a bare `attr_reader` stay out. `def` stays on the generic prefix keys.

## [0.3.457] — 2026-10-20

View:

- **Function List** lists GDScript `class_name` / `signal` (`class_name Player`, `class_name Player extends Node2D`, `signal hit`, `signal hit(amount)`, `signal hit(amount: int)`). `class_name the player`, `signal the alarm`, and a bare `class_name` / `signal` stay out. `func` stays on the generic prefix keys.

## [0.3.456] — 2026-10-20

View:

- **Function List** lists Objective-C `@implementation` / `@protocol` (`@implementation MyClass`, `@implementation MyClass (Cat)`, `@protocol MyDelegate`, `@protocol MyDelegate <NSObject>`). `@interface` stays on the Java annotation-type path. Bare keywords and prose stay out.

## [0.3.455] — 2026-10-20

View:

- **Function List** lists Julia `baremodule` / `primitive type` (`baremodule Foo`, `primitive type Float16 16 end`, `primitive type Float16 <: AbstractFloat 16`). `baremodule the team`, `primitive type` without bits, and a bare `baremodule` stay out. `function` / `macro` / `struct` / `mutable struct` / `module` / `abstract type` stay on the generic prefix keys.

## [0.3.454] — 2026-10-20

View:

- **Function List** lists Verilog / SystemVerilog `task` / `macromodule` / `program` / `primitive` / `checker` (`task do_reset;`, `task automatic foo(`, `macromodule M;`, `program test;`, `primitive udp_and (`, `checker c1;`; optional `extern` / `export`). `task the team`, `tasks`, and a bare `task` / `program` stay out.

## [0.3.453] — 2026-10-20

View:

- **Function List** lists VHDL `entity` / `architecture` / `component` / `package` / `package body` (`entity Foo is`, `architecture RTL of Foo is`, `component Bar is`, `package Utils is`, `package body Utils is`). `entity the team`, `architecture of Foo`, `entities`, and a bare `entity` / `package` stay out.

## [0.3.452] — 2026-10-20

View:

- **Function List** lists CMake `function(name)` / `macro(name)` (`function(my_func)`, `macro(my_macro arg)`, optional space before `(`). `function()`, `functions(…)`, `endfunction()`, and a bare `function` / `macro` stay out.

## [0.3.451] — 2026-10-20

View:

- **Function List** lists MATLAB / Octave `classdef` declarations (`classdef MyClass`, `classdef MyClass < handle`, `classdef (Sealed) MyClass`). `classdef the team`, `classdefinitions`, and a bare `classdef` stay out.

## [0.3.450] — 2026-10-20

View:

- **Function List** lists Zig `const Name = struct` / `enum` / `union` / `opaque` / `error` (optional `pub`, `packed`, `extern`) and `test` blocks (`const Point = struct {`, `pub const E = enum(u8) {`, `const U = packed union {`, `test "add" {`, `test {`). `const x = 1`, `const Foo = i32`, `test the waters`, and a bare `test` stay out.

## [0.3.449] — 2026-10-20

View:

- **Function List** lists Solidity `contract`, `library`, `modifier`, `error`, and `constructor` (`contract Token is IERC20 {`, `library Math {`, `modifier onlyOwner()`, `error InsufficientBalance(uint256)`, `constructor(uint256 x)`). `contract the team`, `library of tools`, `modifier of state`, `error rate`, and a bare `constructor` stay out.

## [0.3.448] — 2026-10-20

View:

- **Function List** lists Objective-C instance and class methods (`- (void)viewDidLoad`, `+ (instancetype)shared`, `- (NSString *)title`, `- (void)initWithFrame:(NSRect)frame`). The return type is a known C or Objective-C type or an uppercase name. Markdown `- (optional) note`, `- (void) more words`, and a bare `- (void)` stay out.

## [0.3.447] — 2026-10-20

View:

- **Function List** lists Terraform / HCL blocks (`resource "aws_instance" "web" {`, `data "aws_ami" "ubuntu" {`, `variable "region" {`, `output "ip" {`, `provider "aws" {`, `locals {`, `terraform {`). `resource the team`, `variable names`, `output the log`, and a bare `locals` stay out. `data class` still lists as a class.

## [0.3.446] — 2026-10-20

View:

- **Function List** lists GraphQL `input`, `scalar`, `extend`, `fragment`, and named `query` / `mutation` / `subscription` (`input UserInput {`, `scalar DateTime`, `extend type User {`, `fragment UserFields on User {`, `query GetUser {`). `input the name`, `query the logs`, `extend the buffer`, and `fragment of text` stay out.

## [0.3.445] — 2026-10-19

View:

- **Function List** lists protobuf `message`, `service`, and `rpc` declarations (`message HelloRequest {`, `service Greeter {`, `rpc SayHello (`). `messages`, `message the team`, and `service workers` stay out.

## [0.3.444] — 2026-10-19

View:

- **Function List** lists a declaration that starts with a C++ `template<…>` header on the same line (`template<typename T> void id(`, `template<> struct Foo`, `export template<class T> class Box`, `template<typename T> concept C`). A bare `concept C =` counts. `templates`, a bare `template`, and `template<…>` with no declaration stay out.

## [0.3.443] — 2026-10-19

View:

- **Function List** lists a declaration that starts with a Java, Kotlin, or TypeScript annotation or a C# / C++ attribute on the same line (`@Override public void`, `@Composable fun`, `@Component class`, `[Fact] public void`, `[[nodiscard]] int`). Java `@interface Name` counts. A bare `@Override`, `@media`, `[section]`, and `[1, 2]` stay out.

## [0.3.442] — 2026-10-19

View:

- **Function List** lists a declaration that starts with a Rust attribute on the same line (`#[inline] pub fn`, `#[test] fn`, `#[derive(Debug)] struct`, `#[must_use] #[inline] fn`, `#[doc = "see ]"] fn`). A bare `#[derive(Debug)]`, an inner `#![no_std]`, and `#` comments stay out.

## [0.3.441] — 2026-10-19

View:

- **Function List** lists F# members (`member this.Greet(name)`, `static member Create()`, `abstract member Draw : unit -> unit`, `override this.ToString()`, `default this.Add(x)`). `member of`, `members`, and `override ToString()` stay out. Kotlin `override fun` still lists as a function.

## [0.3.440] — 2026-10-19

View:

- **Function List** lists Scala `case class` and `case object` (`case class User(name: String)`, `sealed abstract case class Expr`, `case object Empty`). Python `case Point(x):`, `case 1:`, bare `case class`, and `case classes` stay out.

## [0.3.439] — 2026-10-19

View:

- **Function List** lists parenthesized Lisp, Scheme, and Clojure definitions: `(defun …)`, `(defmacro …)`, `(defmethod …)`, `(defclass …)`, `(defn …)` / `(defn- …)`, `(define (name …)`, `(define name (lambda …)`, `(def name (fn …)`, and `(ns …)`. `(def x 1)`, `(define x 1)`, `(let …)`, `(defunct`, and `(definition` stay out.

## [0.3.438] — 2026-10-19

View:

- **Function List** lists F# / OCaml `let` / `let rec` bindings with parameters (`let greet name =`, `let private helper x =`) and `let name = fun …` / `let name = function | …`. Plain `let x = 1`, bare `let x = function`, `let!`, Rust `let f = |x| x`, and `lettuce` stay out.

## [0.3.437] — 2026-10-19

View:

- **Function List** lists R assignments `name <- function(...)`, `name <<- function(...)`, and the R 4.1 form `name <- \(...)`. Dotted names such as `read.csv` count. `x <- 1`, `x <- functional`, a bare `<- function`, and a spaced `x < - function` stay out.

## [0.3.436] — 2026-10-19

View:

- **Function List** lists SQL `CREATE TABLE` / `VIEW` / `FUNCTION` / `PROCEDURE` / `TRIGGER` / `INDEX` / `SCHEMA` / `TYPE`, including `OR REPLACE`, `TEMP` / `TEMPORARY`, `MATERIALIZED`, `UNIQUE`, and `IF NOT EXISTS`. The name may be dotted or quoted. `created`, `create a note`, a bare `CREATE TABLE`, and plural `create tables` stay out.

## [0.3.435] — 2026-10-19

View:

- **Function List** lists Elixir `defdelegate` / `defmacrop`, Clojure `defn` / `defn-` / `defonce` / `ns`, and Haskell `data` / `newtype` / `instance` when the next token is an uppercase name or `(`. Words such as `defdelegates`, `defns`, `defonces`, `nspace`, `database`, `data stays`, and `instance of` stay out. Kotlin `data class` still lists as a class.

## [0.3.434] — 2026-10-19

View:

- **Function List** lists Elixir `defprotocol` / `defimpl` / `defstruct` / `defguard` / `defguardp`, Pascal / Delphi / Ada `procedure`, and Fortran `subroutine`. Words such as `defprotocols`, `defimpls`, `defstructs`, `defguards`, `defguardps`, `procedures`, and `subroutines` stay out.

## [0.3.433] — 2026-10-19

View:

- **Function List** lists Lua `local function`, Perl `sub`, Nim `proc` / `method` / `iterator`, and Elixir `defmodule` / `defp` / `defmacro`. Words such as `substitute`, `process`, `methods`, `iterators`, `defmodules`, `defps`, and `defmacros` stay out. Bare `local x = 1` stays out.

## [0.3.432] — 2026-10-19

View:

- **Function List** lists C# `event`, Kotlin `typealias`, Dart / Kotlin `mixin`, Swift `init` / `init?` / `init!` / `deinit` / `subscript` (with `convenience` / `required` / `fileprivate`), and C++ `operator` overloads (`operator+`, `bool operator==`, `operator new` / `delete[]`, `Foo::operator()`). Words such as `events`, `typealiases`, `mixing`, `initial`, and `operators` stay out.

## [0.3.431] — 2026-10-19

View:

- **Function List** lists C# `partial` / `readonly` / `ref` / `new` qualifiers, `delegate` declarations, C `typedef`, and Go / Java `package` lines. Words such as `partially`, `delegates`, `typedefs`, and `packaged` stay out. Bare `new Foo();` stays out.

## [0.3.430] — 2026-10-19

View:

- **Function List** lists Java / C# `record` declarations, TypeScript / C# `namespace` / `module` (including `export` / `declare`), and Rust / C `union`. Words such as `recording`, `modules`, `namespaced`, and `unions` stay out.

## [0.3.429] — 2026-10-19

View:

- **Function List** lists classic JavaScript / TypeScript function-expression assignments (`const greet = function(name) {`, `let load = async function(id) {`, `var gen = function*() {`, optional expression name). `functional`, bare `function` without `(`, and `list.map(function…)` stay out.

## [0.3.428] — 2026-10-19

View:

- **Function List** lists function-like C preprocessor macros (`#define MAX(a, b) …`, `# define SQUARE(x) …`). Object-like `#define FOO 1`, `#include`, and `#ifdef` stay out.

## [0.3.427] — 2026-10-19

View:

- **Function List** lists named arrow bindings (`const greet = (name) =>`, `let load = async (id) =>`, `paint = x => x`, including a `: return` type). Bare arrows (`(x) =>`), match arms (`Ok(v) =>`), and calls stay out.

## [0.3.426] — 2026-10-19

View:

- **Function List** lists same-line method and constructor bodies (`greet() {`, `get` / `set`, `Foo::bar() {`, `~Widget() {`, `void Foo::bar() {`), `function*` generators, and `export default` / `declare` function or class lines. Calls (`foo();`), member calls, and arrows (`foo() =>`) stay out.

## [0.3.425] — 2026-10-19

View:

- **Function List** lists C, C++, Java, and C# declarations that start with a return type (`void main(`, `String name(`, `unsigned long parse(`), including `virtual` / `inline` / `constexpr`. Calls (`foo(`), assignments (`int x = f(`), and control-flow (`if`, `return`, `new`) stay out.

## [0.3.424] — 2026-10-19

View:

- **Function List** lists Swift `protocol`, `extension`, and `actor` declarations, Kotlin `suspend fun` / `data class` / `sealed` / `companion object`, and Java `final` / `abstract` class lines. A bare `object` counts only when the next token is `{`, `:`, or an uppercase name, so `object oriented` stays out.

## [0.3.423] — 2026-10-19

View:

- **Function List** lists Go and Swift `func` declarations and Kotlin `fun` declarations, including lines that start with `internal`, `override`, or `open`. `function` (JavaScript) still matches. Words such as `functionally` stay out.

## [0.3.422] — 2026-10-19

View:

- **Function List** lists Rust `macro_rules!` and `macro` items, and `fn` items behind `extern "ABI"` (including `pub unsafe extern "C" fn`). `extern crate` stays out.

## [0.3.421] — 2026-10-19

View:

- **Function List** lists Rust items that start with `pub(crate)`, `pub(super)`, `pub(in …)`, `unsafe`, or `const` (and the same stack on `struct` / `impl` / `enum` / `trait`). Comments and plain `let` lines stay out.

## [0.3.420] — 2026-10-19

Language:

- **Syntax language cycle** is remappable: Preferences → Language shortcut, or `shortcut_language` in `npp-rs/settings.json` (default `Alt+=`). Cycles the active tab through the highlight languages (plain, C, C++, HTML, Java, JavaScript, JSON, Python, Rust, SQL, XML). Shift + that chord cycles backward. Equals stays off Zoom in (`Cmd+=`) and Date Time customized (`Cmd+Alt+=`). Shortcut Mapper and About show the binding.

## [0.3.419] — 2026-10-19

Help:

- **Command Line Arguments** is remappable: Preferences → Command Line shortcut, or `shortcut_cmdline` in `npp-rs/settings.json` (default `Shift+F1`). Opens a read-only tab (? → Command Line Arguments). Shift is part of the chord so it does not steal About (`F1`), Summary (`Alt+F1`), Load Session (`Cmd+F1`), or Sort tabs by name (`Cmd+Alt+F1`). Shortcut Mapper and About show the binding.

## [0.3.418] — 2026-10-18

Help:

- **Releases** is remappable: Preferences → Releases shortcut, or `shortcut_releases` in `npp-rs/settings.json` (default `Cmd+Alt+]`). Opens GitHub Releases (? → Releases). Shift + that chord opens the changelog. The bracket stays off Indent (`Cmd+]`), Hide-equal context increase (`Alt+]`), Outdent (`Cmd+[`), and Hide-equal context decrease (`Alt+[`). Shortcut Mapper and About show the binding.

## [0.3.417] — 2026-10-18

Help:

- **Discussions** is remappable: Preferences → Discussions shortcut, or `shortcut_discussions` in `npp-rs/settings.json` (default `Cmd+Alt+[`). Opens GitHub Discussions (? → Discussions). Shift + that chord opens the Wiki. The bracket stays off Outdent (`Cmd+[`), Hide-equal context decrease (`Alt+[`), Indent (`Cmd+]`), and Hide-equal context increase (`Alt+]`). Shortcut Mapper and About show the binding.

## [0.3.416] — 2026-10-18

Help:

- **npp-rust on GitHub** is remappable: Preferences → GitHub shortcut, or `shortcut_github` in `npp-rs/settings.json` (default `Alt+;`). Opens the project repository (? → npp-rust on GitHub). Shift + that chord opens Issues. Semicolon stays off Tab colour 5 (`Cmd+;` / `Cmd+Shift+;`) and Sort tabs by modified time (`Cmd+Alt+;` / `Cmd+Alt+Shift+;`). Shortcut Mapper and About show the binding.

## [0.3.415] — 2026-10-18

Help:

- **Open npp-rust Logs** is remappable: Preferences → Open Logs shortcut, or `shortcut_open_logs` in `npp-rs/settings.json` (default `Alt+F2`). Opens `logs/*.log` from the process working directory (? → Open npp-rust Logs). Alt is part of the chord so it does not steal Next bookmark (`F2`), Toggle bookmark (`Cmd+F2`), Clear bookmarks (`Cmd+Shift+F2`), Inverse bookmarks (`Alt+Shift+F2`), or Copy bookmarked lines (`Cmd+Alt+F2`). Shortcut Mapper and About show the binding.

## [0.3.414] — 2026-10-18

Help:

- **Debug Info** is remappable: Preferences → Debug Info shortcut, or `shortcut_debug_info` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+Space`). Opens a read-only tab (? → Debug Info). Shift is part of the chord so it does not steal Path autocomplete (`Cmd+Alt+Space`) or Function Call Tip (`Cmd+Shift+Space`). Shortcut Mapper and About show the binding.

## [0.3.413] — 2026-10-18

View:

- **View current file in IE** is remappable: Preferences → View in IE shortcut, or `shortcut_view_in_ie` in `npp-rs/settings.json` (default `Alt+Shift+9`). Opens the saved file in IE (View → View Current File in IE). Shift is part of the chord so it does not steal Proper Case (`Cmd+Alt+9`; Shift = blend), 1st–9th Tab (`Cmd+Shift+1`…`9`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the binding.

## [0.3.412] — 2026-10-18

View:

- **View current file in Edge** is remappable: Preferences → View in Edge shortcut, or `shortcut_view_in_edge` in `npp-rs/settings.json` (default `Alt+Shift+.`). Opens the saved file in Edge (View → View Current File in Edge). Shift is part of the chord so it does not steal Sort tabs by type (`Cmd+Alt+.`; Shift = descending) or Open (`Cmd+O`). Shortcut Mapper and About show the binding.

## [0.3.411] — 2026-10-18

View:

- **View current file in Chrome** is remappable: Preferences → View in Chrome shortcut, or `shortcut_view_in_chrome` in `npp-rs/settings.json` (default `Alt+Shift+B`). Opens the saved file in Chrome (View → View Current File in Chrome). Shift is part of the chord so it does not steal Ignore Blank Lines (`Alt+B`), Begin/End Select (`Cmd+Alt+B`), or Matching brace select (`Cmd+Shift+B`). Shortcut Mapper and About show the binding.

## [0.3.410] — 2026-10-18

View:

- **View current file in Firefox** is remappable: Preferences → View in Firefox shortcut, or `shortcut_view_in_browser` in `npp-rs/settings.json` (default `Cmd+Shift+Y`). Opens the saved file in Firefox (View → View Current File in Firefox). Shift is part of the chord so it does not steal Redo (`Cmd+Y`), Copy Compare Summary (`Alt+Y`), or Save a Copy As / Rename (`Cmd+Alt+Y` / `Cmd+Alt+Shift+Y`). Shortcut Mapper and About show the binding.

## [0.3.409] — 2026-10-18

Edit:

- **Paste HTML** is remappable: Preferences → Paste HTML shortcut, or `shortcut_paste_html` in `npp-rs/settings.json` (default `Cmd+Alt+-`). Pastes clipboard text (Edit → Paste HTML Content). **Shift** flips to Paste RTF Content (`Cmd+Alt+Shift+-`). Does not steal Paste (`Cmd+V`), Paste Binary (`Cmd+Shift+V`), Zoom out (`Cmd+-`), or Date Time customized (`Cmd+Alt+=`). HTML and RTF paste stay a plain-text fallback. Shortcut Mapper and About show the binding.

## [0.3.408] — 2026-10-18

Edit:

- **Paste Binary** is remappable: Preferences → Paste Binary shortcut, or `shortcut_paste_binary` in `npp-rs/settings.json` (default `Cmd+Shift+V`). Pastes hex of UTF-8 bytes as text (Edit → Paste Binary Content). Shift is part of the chord so it does not steal Paste (`Cmd+V`), Show White Space / Show EOL (`Alt+V` / `Alt+Shift+V`), or TAB to Space / Space to TAB Leading (`Cmd+Alt+V` / `Cmd+Alt+Shift+V`). Shortcut Mapper and About show the binding.

## [0.3.407] — 2026-10-18

Plugins / Ollama (issue #21):

- **Ollama Chat** panel: shows the exact prompt sent, a **Thinking…** indicator with elapsed seconds while the worker runs, and the reply in a chat transcript. Follow-up messages via Send; optional attach of selection/buffer. **Open reply in tab** is optional.
- **Ask Ollama** (Plugins and editor right-click) opens that panel and uses Ollama `/api/chat` so multi-turn context stays in the panel. Status still lists models in a tab.

## [0.3.406] — 2026-10-18

Edit:

- **Cut Binary** is remappable: Preferences → Cut Binary shortcut, or `shortcut_cut_binary` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+X`). Cuts the selection as hex of its UTF-8 bytes (Edit → Cut Binary Content). Does not steal Cut (`Cmd+X`), Copy Binary (`Cmd+Shift+X`), Space to TAB (All) (`Cmd+Alt+X`), or Collapse unchanged at caret (`Alt+Shift+X`). Shortcut Mapper and About show the binding.

## [0.3.405] — 2026-10-18

Edit:

- **Copy Binary** is remappable: Preferences → Copy Binary shortcut, or `shortcut_copy_binary` in `npp-rs/settings.json` (default `Cmd+Shift+X`). Copies the selection as hex of its UTF-8 bytes (Edit → Copy Binary Content). Shift is part of the chord so it does not steal Cut (`Cmd+X`), Space to TAB (All) (`Cmd+Alt+X`), or Collapse unchanged at caret (`Alt+Shift+X`). Shortcut Mapper and About show the binding.

## [0.3.404] — 2026-10-18

Edit:

- **Copy / Cut / Paste Binary** uses hex of the selection’s UTF-8 bytes (`48 69` for `Hi`). Paste binary decodes that hex back into text. HTML and RTF paste stay a plain-text fallback. A non-hex clipboard is left unchanged.

## [0.3.403] — 2026-10-18

Edit:

- **System read-only attribute** is remappable: Preferences → System read-only attribute shortcut, or `shortcut_system_readonly` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+F`). Toggles the OS read-only bit on the saved file and the app read-only flag (Edit → Read-Only Attribute in Windows). Shift is part of the chord so it does not steal Find (`Cmd+F`), Replace alternate (`Cmd+Shift+F`), Find in Files (`Cmd+Alt+F`), or Fold current (`Alt+F` / `Alt+Shift+F`). Shortcut Mapper and About show the binding.

## [0.3.402] — 2026-10-18

Tools:

- **SHA-512 generate from file** is remappable: Preferences → SHA-512 from file shortcut, or `shortcut_sha512_file` in `npp-rs/settings.json` (default `Cmd+Shift+F11`). Hashes the saved file and copies the digest (Tools → SHA-512 → Generate from files). Shift is part of the chord so it does not steal SHA-512 generate (`Cmd+F6` / `Cmd+Shift+F6`) or ANSI (`Cmd+F11`). Shortcut Mapper and About show the binding.

## [0.3.401] — 2026-10-18

Tools:

- **SHA-1 generate from file** is remappable: Preferences → SHA-1 from file shortcut, or `shortcut_sha1_file` in `npp-rs/settings.json` (default `Alt+F12`). Hashes the saved file and copies the digest (Tools → SHA-1 → Generate from files). Does not steal Post-It (`F12`), SHA-1 generate (`Cmd+F12` / `Cmd+Shift+F12`), Convert EOL Macintosh (`Alt+Shift+F12`), or Convert EOL Unix/Windows (`Cmd+Alt+F12`). Shortcut Mapper and About show the binding.

## [0.3.400] — 2026-10-18

Tools:

- **SHA-256 generate from file** is remappable: Preferences → SHA-256 from file shortcut, or `shortcut_sha256_file` in `npp-rs/settings.json` (default `Alt+Shift+H`). Hashes the saved file and copies the digest (Tools → SHA-256 → Generate from files). Shift is part of the chord so it does not steal Hide unchanged lines (`Alt+H`) or SHA-256 generate (`Alt+F11` / `Alt+Shift+F11`). Shortcut Mapper and About show the binding.

## [0.3.399] — 2026-10-18

Tools:

- **MD5 generate from file** is remappable: Preferences → MD5 from file shortcut, or `shortcut_hash_file` in `npp-rs/settings.json` (default `Cmd+Shift+H`). Hashes the saved file and copies the digest (Tools → MD5 → Generate from files). Shift is part of the chord so it does not steal Replace (`Cmd+H`) or Hide unchanged lines (`Alt+H`). Shortcut Mapper and About show the binding.

## [0.3.398] — 2026-10-18

Edit:

- **Clipboard History** keeps the last 10 copies from this session. Edit → Clipboard History (default `Cmd+Shift+C`, Preferences `shortcut_clipboard_history`) cycles them into the paste clipboard. The status line shows the slot (`2/10`) and a short preview. There is still no history panel.

## [0.3.397] — 2026-10-18

Search:

- **Find Mark Style** next, previous, and copy use style 1 (the slot Find-bar Mark writes). They no longer walk or copy bookmarks. Menu rows show the style 1 chords from Next Style and Copy Styled Text.

## [0.3.396] — 2026-10-18

Edit:

- **Multi-select All (Match Case + Whole Word)** is remappable: Preferences → Multi-select all match case whole word shortcut, or `shortcut_multi_select_all_case_word` in `npp-rs/settings.json` (default `Alt+Shift+A`). Selects every case-sensitive whole-word match of the current word or selection as multi-carets (Edit → Multi-select All → Match Case Whole Word). Shift is part of the chord so it does not steal Show All Characters (`Alt+A`), Select All (`Cmd+A`), Multi-select all match case (`Cmd+Shift+A`), or Multi-select all / whole word (`Cmd+Alt+A` / `Cmd+Alt+Shift+A`). Shortcut Mapper and About show the binding.

## [0.3.395] — 2026-10-18

Edit:

- **Multi-select All (Match Case)** is remappable: Preferences → Multi-select all match case shortcut, or `shortcut_multi_select_all_case` in `npp-rs/settings.json` (default `Cmd+Shift+A`). Selects every case-sensitive match of the current word or selection as multi-carets (Edit → Multi-select All → Match Case Only). Shift is part of the chord so it does not steal Select All (`Cmd+A`), Show All Characters (`Alt+A`), or Multi-select all / whole word (`Cmd+Alt+A` / `Cmd+Alt+Shift+A`). Shortcut Mapper and About show the binding.

## [0.3.394] — 2026-10-18

Edit:

- **Multi-select Next (Match Case + Whole Word)** is remappable: Preferences → Multi-select next match case whole word shortcut, or `shortcut_multi_select_next_case_word` in `npp-rs/settings.json` (default `Cmd+Shift+N`). Adds the next case-sensitive whole-word match as a caret (Edit → Multi-select Next → Match Case Whole Word). Shift is part of the chord so it does not steal New (`Cmd+N`), Show NPC (`Alt+N`), Multi-select next / skip (`Cmd+Alt+N` / `Cmd+Alt+Shift+N`), or Match Case Only (`Alt+Shift+N`). Shortcut Mapper and About show the binding.

## [0.3.393] — 2026-10-18

Edit:

- **Multi-select Next (Whole Word)** is remappable: Preferences → Multi-select next whole word shortcut, or `shortcut_multi_select_next_word` in `npp-rs/settings.json` (default `Cmd+Shift+W`). Adds the next whole-word match as a caret (Edit → Multi-select Next → Whole Word). Shift is part of the chord so it does not steal Close tab (`Cmd+W`), Close All (`Cmd+Alt+W`), or Ignore whitespace (`Alt+W`). Shortcut Mapper and About show the binding.

## [0.3.392] — 2026-10-18

Edit:

- **Multi-select Next (Match Case)** is remappable: Preferences → Multi-select next match case shortcut, or `shortcut_multi_select_next_case` in `npp-rs/settings.json` (default `Alt+Shift+N`). Adds the next case-sensitive match as a caret (Edit → Multi-select Next → Match Case Only). Shift is part of the chord so it does not steal Show NPC (`Alt+N`) or Multi-select next / skip (`Cmd+Alt+N` / `Cmd+Alt+Shift+N`). Shortcut Mapper and About show the binding.

## [0.3.391] — 2026-10-18

Edit:

- **Block Comment / Block Uncomment** is remappable: Preferences → Block comment shortcut, or `shortcut_stream_comment` in `npp-rs/settings.json` (default `Alt+,`). Wraps the selection in a block comment (Edit → Comment/Uncomment → Block Comment). Shift flips to Block Uncomment. Does not steal Toggle comment (`Cmd+/` / `Cmd+Shift+/`), Single Line Comment (`Alt+/`), Preferences (`Cmd+,`), or Sort tabs by path (`Cmd+Alt+,`). Shortcut Mapper and About show the binding.

## [0.3.390] — 2026-10-17

Edit:

- **Single Line Comment / Uncomment** is remappable: Preferences → Single line comment shortcut, or `shortcut_line_comment` in `npp-rs/settings.json` (default `Alt+/`). Forces a line comment (Edit → Comment/Uncomment → Single Line Comment). Shift flips to Single Line Uncomment. Does not steal Toggle comment (`Cmd+/`), Block Comment (`Cmd+Shift+/`), or Sort tabs by size (`Cmd+Alt+/`). Shortcut Mapper and About show the binding.

## [0.3.389] — 2026-10-17

Macro:

- **Run a Macro Multiple Times** is remappable: Preferences → Run macro multiple times shortcut, or `shortcut_macro_repeat` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+K`). Replays the recorded steps three times (Macro → Run a Macro Multiple Times). Shift is part of the chord so it does not steal Copy Current Directory (`Cmd+Alt+K`), Copy Compare Hunk (`Alt+K`), Shortcut Mapper (`Alt+Shift+K`), or Tab colour 3 (`Cmd+K`). Shortcut Mapper and About show the binding.

## [0.3.388] — 2026-10-17

Macro:

- **Save Current Recorded Macro** is remappable: Preferences → Save recorded macro shortcut, or `shortcut_macro_save` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+S`). Writes the recorded steps (Macro → Save Current Recorded Macro). Shift is part of the chord so it does not steal Save (`Cmd+S`), Save As (`Cmd+Shift+S`), Save All (`Cmd+Alt+S`), Swap Compare (`Alt+S`), or Compare to Saved (`Alt+Shift+S`). Shortcut Mapper and About show the binding.

## [0.3.387] — 2026-10-17

Macro:

- **Start/Stop Recording** is remappable: Preferences → Start/Stop Recording shortcut, or `shortcut_macro_record` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+R`). The same chord starts recording when idle and stops while recording (Macro → Start Recording / Stop Recording). Shift is part of the chord so it does not steal Reload (`Cmd+R`), Run (`Cmd+Shift+R`), Toggle Read-Only (`Cmd+Alt+R`), or RTL/LTR (`Alt+R` / `Alt+Shift+R`). Shortcut Mapper and About show the binding.

## [0.3.386] — 2026-10-17

Macro:

- **Playback** is remappable: Preferences → Macro Playback shortcut, or `shortcut_macro_playback` in `npp-rs/settings.json` (default `Cmd+Shift+P`). Plays the recorded macro (Macro → Playback). Shift is part of the chord so it does not steal Print (`Cmd+P`). Shortcut Mapper and About show the binding.

## [0.3.385] — 2026-10-17

Edit:

- **Date Time (customized)** is remappable: Preferences → Date Time customized shortcut, or `shortcut_insert_datetime_custom` in `npp-rs/settings.json` (default `Cmd+Alt+=`). Inserts the ISO-style local timestamp (Edit → Insert). Does not collide with Date Time short/long (`F5` / `Shift+F5`) or Zoom in (`Cmd+=`). Shortcut Mapper and About show the binding.

## [0.3.384] — 2026-10-17

View:

- **1st–9th Tab** is remappable: Preferences → 1st–9th tab shortcut, or `shortcut_goto_tab` in `npp-rs/settings.json` (default `Cmd+Shift+1`). Digit keys 1 through 9 with those modifiers jump to View → Tab → 1st…9th Tab. A letter remap jumps to the 1st tab only. Does not collide with Mark Style (`Cmd+1`…`5`), Fold Level (`Alt+1`…`8`), or Clear all styles (`Cmd+Shift+0`). Shortcut Mapper and About show the binding.

## [0.3.383] — 2026-10-17

View:

- **Move to New Instance / Open in New Instance** is remappable: Preferences → New instance shortcut, or `shortcut_new_instance` in `npp-rs/settings.json` (default `F9`). Shift opens another window and keeps this tab. Does not collide with Find characters in range (`Alt+F9`), UTF-8 (`Cmd+F9`), read-only for all (`F10`), fullscreen (`F11`), or Post-It (`F12`). Shortcut Mapper and About show both bindings.

## [0.3.382] — 2026-10-17

Edit:

- **Undo the latest added multi-select** is remappable: Preferences → Multi-select undo shortcut, or `shortcut_multi_select_undo` in `npp-rs/settings.json` (default `Cmd+Alt+Backspace`). Drops the newest extra caret. Does not also delete a character, and does not collide with Undo (`Cmd+Z`) or bare Backspace. Shortcut Mapper and About show the binding.

## [0.3.381] — 2026-10-17

Edit:

- **Function Parameters Next / Previous** is remappable: Preferences → Function parameters shortcut, or `shortcut_calltip_nav` in `npp-rs/settings.json` (default `Alt+Down`). Steps the in-file function call tip. Opposite Up is the previous hint. Letter remaps use Shift for previous. Does not collide with Move Line (`Cmd+Shift+Up`/`Down`), plain arrow motion, or rect select (`Alt+Shift+arrows`). Shortcut Mapper and About show both bindings.

## [0.3.380] — 2026-10-17

File:

- **Move to Recycle Bin** is remappable: Preferences → Move to Recycle Bin shortcut, or `shortcut_delete_file` in `npp-rs/settings.json` (default `Cmd+Shift+Delete`). Moves the saved active file to the user Trash and closes the tab. Does not also delete a character, and does not collide with forward Delete. Shortcut Mapper and About show the binding.

## [0.3.379] — 2026-10-17

Help:

- **About** is remappable: Preferences → About shortcut, or `shortcut_about` in `npp-rs/settings.json` (default `F1`). Opens ? → About npp-rs. Does not collide with Summary (`Alt+F1`), Load Session (`Cmd+F1`), or Sort tabs by name (`Cmd+Alt+F1`). Shortcut Mapper and About show the binding.

## [0.3.378] — 2026-10-17

File:

- **Open in Default Viewer** is remappable: Preferences → Open in Default Viewer shortcut, or `shortcut_open_viewer` in `npp-rs/settings.json` (default `Cmd+Shift+O`). Opens the active file in the OS default viewer. Does not collide with Open (`Cmd+O`), Move/Clone Other View (`Alt+O`), or Sort Lines (`Cmd+Alt+O`). Shortcut Mapper and About show the binding.

## [0.3.377] — 2026-10-17

Run:

- **Run…** is remappable: Preferences → Run shortcut, or `shortcut_run` in `npp-rs/settings.json` (default `Cmd+Shift+R`). Runs Run → Run… (pick a program to spawn; cancel opens a shell in the active file’s folder). Does not collide with Reload (`Cmd+R`), Toggle Read-Only (`Cmd+Alt+R`), or RTL (`Alt+R`). Shortcut Mapper and About show the binding.

## [0.3.376] — 2026-10-17

Edit:

- **Clipboard History** is remappable: Preferences → Clipboard History shortcut, or `shortcut_clipboard_history` in `npp-rs/settings.json` (default `Cmd+Shift+C`). Runs Edit → Clipboard History (shows the last copied entry in the status line; 1-entry MVP). Does not collide with Copy (`Cmd+C`), Column Editor (`Cmd+Alt+C`), or Copy Compare Diff (`Alt+C`). Shortcut Mapper and About show the binding.

## [0.3.375] — 2026-10-17

View:

- **Tab colour 5 / clear** is remappable: Preferences → Tab colour 5 shortcut, or `shortcut_tab_colour_5` in `npp-rs/settings.json` (default `Cmd+;`). Sets View → Apply Color 5 on the active tab. Shift + that chord removes the tab colour. Does not collide with Sort tabs by modified (`Cmd+Alt+;`), Preferences (`Cmd+,`), or Tab colour 4 (`Cmd+M`). Shortcut Mapper and About show both bindings.

## [0.3.374] — 2026-10-17

View:

- **Tab colour 4 / clear** is remappable: Preferences → Tab colour 4 shortcut, or `shortcut_tab_colour_4` in `npp-rs/settings.json` (default `Cmd+M`). Sets View → Apply Color 4 on the active tab. Shift + that chord removes the tab colour. Does not collide with Bookmark Compare Diffs (`Alt+M`), Copy All Paths (`Cmd+Alt+M`), or Tab colour 3 (`Cmd+K`). Shortcut Mapper and About show both bindings.

## [0.3.373] — 2026-10-17

View:

- **Tab colour 3 / clear** is remappable: Preferences → Tab colour 3 shortcut, or `shortcut_tab_colour_3` in `npp-rs/settings.json` (default `Cmd+K`). Sets View → Apply Color 3 on the active tab. Shift + that chord removes the tab colour. Does not collide with Copy Compare Hunk (`Alt+K`), Shortcut Mapper (`Alt+Shift+K`), or Copy Current Directory (`Cmd+Alt+K`). Shortcut Mapper and About show both bindings.

## [0.3.372] — 2026-10-17

View:

- **Tab colour 2 / clear** is remappable: Preferences → Tab colour 2 shortcut, or `shortcut_tab_colour_2` in `npp-rs/settings.json` (default `Cmd+F4`). Sets View → Apply Color 2 on the active tab. Shift + that chord removes the tab colour. Does not collide with Next/Prev Search Result (`F4`), Find All (`Cmd+Alt+F4`), or Find Bookmark (`Alt+Shift+F4`). Shortcut Mapper and About show both bindings.

## [0.3.371] — 2026-10-17

View:

- **Tab colour 1 / clear** is remappable: Preferences → Tab colour shortcut, or `shortcut_tab_colour` in `npp-rs/settings.json` (default `Cmd+F5`). Sets View → Apply Color 1 on the active tab. Shift + that chord removes the tab colour. Does not collide with Date Time (`F5`), Sort Lines Integer (`Cmd+Alt+F5`), or Find Copy (`Alt+Shift+F5`). Shortcut Mapper and About show both bindings.

## [0.3.370] — 2026-10-17

Settings:

- **Shortcut Mapper** is remappable: Preferences → Shortcut Mapper shortcut, or `shortcut_mapper` in `npp-rs/settings.json` (default `Alt+Shift+K`). Opens Settings → Shortcut Mapper (static dump of hard-wired and remappable chords). Does not collide with Copy Compare Hunk (`Alt+K`), Copy Current Directory Path (`Cmd+Alt+K`), or Preferences (`Cmd+,`). Shortcut Mapper and About show the binding.

## [0.3.369] — 2026-10-17

Tools:

- **SHA-512 generate / clipboard** is remappable: Preferences → SHA-512 shortcut, or `shortcut_sha512` in `npp-rs/settings.json` (default `Cmd+F6`). Hashes the selection, or the whole document, and inserts the digest (Tools → SHA-512 → Generate). Shift + that chord copies the digest instead. Does not collide with Focus on Another View (`F6`), Find Cut (`Alt+Shift+F6`), or Sort Lines Lex Ignoring Case (`Cmd+Alt+F6`). Shortcut Mapper and About show both bindings.

## [0.3.368] — 2026-10-17

Tools:

- **SHA-1 generate / clipboard** is remappable: Preferences → SHA-1 shortcut, or `shortcut_sha1` in `npp-rs/settings.json` (default `Cmd+F12`). Hashes the selection, or the whole document, and inserts the digest (Tools → SHA-1 → Generate). Shift + that chord copies the digest instead. Does not collide with Post-It (`F12`), Convert EOL Macintosh (`Alt+Shift+F12`), or Convert EOL Unix/Windows (`Cmd+Alt+F12`). Shortcut Mapper and About show both bindings.

## [0.3.367] — 2026-10-17

Tools:

- **SHA-256 generate / clipboard** is remappable: Preferences → SHA-256 shortcut, or `shortcut_sha256` in `npp-rs/settings.json` (default `Alt+F11`). Hashes the selection, or the whole document, and inserts the digest (Tools → SHA-256 → Generate). Shift + that chord copies the digest instead. Does not collide with Fullscreen (`F11`), ANSI (`Cmd+F11`), or EOL to Space (`Cmd+Alt+F11`). Shortcut Mapper and About show both bindings.

## [0.3.366] — 2026-10-17

Tools:

- **MD5 generate / clipboard** is remappable: Preferences → MD5 shortcut, or `shortcut_md5` in `npp-rs/settings.json` (default `Alt+F10`). Hashes the selection, or the whole document, and inserts the digest (Tools → MD5 → Generate). Shift + that chord copies the digest instead. Does not collide with Read-only for all documents (`F10` / Shift+F10), UTF-16 (`Cmd+F10`), or Sort Lines by Length (`Cmd+Alt+F10`). Shortcut Mapper and About show both bindings.

## [0.3.365] — 2026-10-17

Encoding:

- **ANSI** is remappable: Preferences → ANSI shortcut, or `shortcut_encoding_ansi` in `npp-rs/settings.json` (default `Cmd+F11`). Sets the active tab to ANSI (Encoding → ANSI; save writes Windows-1252). Does not collide with Fullscreen (`F11`) or EOL to Space (`Cmd+Alt+F11`). Shortcut Mapper and About show the binding.

## [0.3.364] — 2026-10-17

Encoding:

- **UTF-16 LE / UTF-16 BE** is remappable: Preferences → UTF-16 shortcut, or `shortcut_encoding_utf16` in `npp-rs/settings.json` (default `Cmd+F10`). Sets the active tab to UTF-16 LE BOM (Encoding → UTF-16 LE BOM). Shift + that chord sets UTF-16 BE BOM. Does not collide with Read-only for all documents (`F10` / Shift+F10) or Sort Lines by Length (`Cmd+Alt+F10`). Shortcut Mapper and About show both bindings.

## [0.3.363] — 2026-10-16

Encoding:

- **UTF-8 / UTF-8-BOM** is remappable: Preferences → UTF-8 shortcut, or `shortcut_encoding_utf8` in `npp-rs/settings.json` (default `Cmd+F9`). Sets the active tab to UTF-8 without BOM (Encoding → UTF-8). Shift + that chord sets UTF-8-BOM. Does not collide with Find characters in range (`Alt+F9`) or Sort Lines Decimal Comma (`Cmd+Alt+F9`). Shortcut Mapper and About show both bindings.

## [0.3.362] — 2026-10-16

Window:

- **Sort tabs by modified time** is remappable: Preferences → Sort tabs by modified time shortcut, or `shortcut_sort_tabs_modified` in `npp-rs/settings.json` (default `Cmd+Alt+;`). Sorts open tabs by file modified time, oldest first (Window, Modified Time Ascending). Shift + that chord sorts newest first. Does not collide with Sort tabs by size (`Cmd+Alt+/`) or Toggle comment (`Cmd+/`). Shortcut Mapper and About show both bindings.

## [0.3.361] — 2026-10-16

Window:

- **Sort tabs by size** is remappable: Preferences → Sort tabs by size shortcut, or `shortcut_sort_tabs_size` in `npp-rs/settings.json` (default `Cmd+Alt+/`). Sorts open tabs by buffer size, smallest first (Window, Size Ascending). Shift + that chord sorts largest first. Does not collide with Sort tabs by type (`Cmd+Alt+.`) or Toggle comment (`Cmd+/`). Shortcut Mapper and About show both bindings.

## [0.3.360] — 2026-10-16

Window:

- **Sort tabs by type** is remappable: Preferences → Sort tabs by type shortcut, or `shortcut_sort_tabs_type` in `npp-rs/settings.json` (default `Cmd+Alt+.`). Sorts open tabs by type A to Z (Window, Type A to Z). Shift + that chord sorts Z to A. Does not collide with Sort tabs by path (`Cmd+Alt+,`) or Preferences (`Cmd+,`). Shortcut Mapper and About show both bindings.

## [0.3.359] — 2026-10-16

Window:

- **Sort tabs by path** is remappable: Preferences → Sort tabs by path shortcut, or `shortcut_sort_tabs_path` in `npp-rs/settings.json` (default `Cmd+Alt+,`). Sorts open tabs by path A to Z (Window, Path A to Z). Shift + that chord sorts Z to A. Does not collide with Sort tabs by name (`Cmd+Alt+F1`) or Preferences (`Cmd+,`). Shortcut Mapper and About show both bindings.

## [0.3.358] — 2026-10-16

Window:

- **Sort tabs by name** is remappable: Preferences → Sort tabs by name shortcut, or `shortcut_sort_tabs` in `npp-rs/settings.json` (default `Cmd+Alt+F1`). Sorts open tabs A to Z (Window, Name A to Z). Shift + that chord sorts Z to A. Does not collide with Load Session (`Cmd+F1`) or Summary (`Alt+F1`). Shortcut Mapper and About show both bindings.

## [0.3.357] — 2026-10-16

File:

- **Load Session / Save Session** is remappable: Preferences → Load Session shortcut, or `shortcut_load_session` in `npp-rs/settings.json` (default `Cmd+F1`). Loads the saved session (File → Load Session). Shift + that chord saves the session. Does not collide with Summary (`Alt+F1`) or Find (`Cmd+F`). Shortcut Mapper and About show both bindings.

## [0.3.356] — 2026-10-16

File:

- **Exit** is remappable: Preferences → Exit shortcut, or `shortcut_exit` in `npp-rs/settings.json` (default `Cmd+Q`). Quits (File → Exit) and still prompts when a document is dirty. Does not collide with Paste Bookmarked Lines (`Cmd+Alt+Q`) or Sync Vertical Scroll (`Alt+Q`). Toggle Comment stays `Cmd+/`. Shortcut Mapper and About show the binding.

## [0.3.355] — 2026-10-16

Settings:

- **Preferences / Style Configurator** is remappable: Preferences → Preferences shortcut, or `shortcut_preferences` in `npp-rs/settings.json` (default `Cmd+,`). Opens Settings → Preferences. Shift + that chord opens Style Configurator. Does not collide with Summary (`Alt+F1`), Print (`Cmd+P`), or Project Panel (`Alt+P`). Shortcut Mapper and About show both bindings.

## [0.3.354] — 2026-10-16

View:

- **Summary / Document List** is remappable: Preferences → Summary shortcut, or `shortcut_summary` in `npp-rs/settings.json` (default `Alt+F1`). Opens View → Summary. Shift + that chord opens View → Document List. Does not collide with Find characters in range (`Alt+F9`) or Read-only for all documents (`F10`). Shortcut Mapper and About show both bindings.

## [0.3.353] — 2026-10-16

Edit:

- **Read-only for all documents** is remappable: Preferences → Read-only for all documents shortcut, or `shortcut_readonly_all` in `npp-rs/settings.json` (default `F10`). Sets read-only on every open document. Shift + that chord clears read-only on every open document. Does not collide with Toggle Read-Only (`Cmd+Alt+R`), RTL (`Alt+R`), Reload (`Cmd+R`), or Sort Lines by Length (`Cmd+Alt+F10`). Shortcut Mapper and About show the effective binding.

## [0.3.352] — 2026-10-16

View:

- **Show Wrap Symbol** is remappable: Preferences → Show wrap symbol shortcut, or `shortcut_show_wrap_symbol` in `npp-rs/settings.json` (default `Alt+Shift+G`). Runs View → Show Wrap Symbol. Does not collide with Show Fold Margin (`Alt+G`), Open Folder as Workspace (`Cmd+Alt+G`), word wrap (`Alt+Z`), or Zoom Sync (`Alt+Shift+Z`). Shortcut Mapper and About show the effective binding.

## [0.3.351] — 2026-10-16

View:

- **Show Wrap Symbol** is a real toggle: View → Show Wrap Symbol, or Preferences → Show wrap symbol, or `show_wrap_symbol` in `npp-rs/settings.json`. When on (the default), a wrapped line draws a wrap mark. Turning it off keeps word wrap and hides the mark. The menu shows a check when the mark is on.

## [0.3.350] — 2026-10-16

View:

- **Hide Lines** is remappable: Preferences → Hide lines shortcut, or `shortcut_hide_lines` in `npp-rs/settings.json` (default `Alt+Shift+L`). Runs View → Hide Lines (current line, or the selected line range). Does not collide with Show Line Numbers (`Alt+L`), Go to line (`Cmd+L`), Delete line (`Cmd+Shift+L`), or Open Containing Folder (`Cmd+Alt+L`). Shortcut Mapper and About show the effective binding.

## [0.3.349] — 2026-10-16

Format:

- **Convert EOL Macintosh (CR)** is remappable: Preferences → Convert EOL Macintosh shortcut, or `shortcut_convert_eol_mac` in `npp-rs/settings.json` (default `Alt+Shift+F12`). Runs Format → Convert to Macintosh. Does not collide with Post-It (`F12`), Convert EOL Unix/Windows (`Cmd+Alt+F12` / `Cmd+Alt+Shift+F12`), or Find Purge (`Alt+Shift+F8`). Shortcut Mapper and About show the effective binding.

## [0.3.348] — 2026-10-16

Edit:

- **Search on Internet / Change Search Engine** is remappable: Preferences → Search on Internet shortcut, or `shortcut_search_internet` in `npp-rs/settings.json` (default `Alt+U`). Runs Edit → Search on Internet (selection required). **Shift** flips to Change Search Engine. Does not collide with Invert Case (`Cmd+Alt+U`), lowercase (`Cmd+U`), or Redact Selection (`Alt+J`). Shortcut Mapper and About show both bindings.

## [0.3.347] — 2026-10-16

Edit:

- **Trim Leading Space** is remappable: Preferences → Trim Leading shortcut, or `shortcut_trim_leading` in `npp-rs/settings.json` (default `Alt+Shift+T`). Runs Edit → Blank Operations → Trim Leading Space. Does not collide with Trim Trailing (`Cmd+Alt+T`), Always on Top (`Alt+T`), or Toggle log tail (`Cmd+Shift+T`). Shortcut Mapper and About show the effective binding.

## [0.3.346] — 2026-10-16

Edit:

- **Redact Selection** is remappable: Preferences → Redact Selection shortcut, or `shortcut_redact` in `npp-rs/settings.json` (default `Alt+J`). Runs Edit → On Selection → Redact Selection █. **Shift** flips to ● fill (matches the menu label). Does not collide with Join Lines (`Cmd+J`), RTL (`Alt+R`), Toggle Read-Only (`Cmd+Alt+R`), or Invert Case (`Cmd+Alt+U`). Shortcut Mapper and About show both bindings.

## [0.3.345] — 2026-10-16

Edit:

- **Random Case** is remappable: Preferences → Random Case shortcut, or `shortcut_random_case` in `npp-rs/settings.json` (default `Cmd+Alt+0`). Runs Edit → Convert Case to → Random Case. Does not collide with Fold All (`Alt+0`), Copy Styled Text all (`Cmd+Alt+Shift+0`), Proper Case (`Cmd+Alt+9`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective binding.

## [0.3.344] — 2026-10-16

Edit:

- **Proper Case Force / Blend** is remappable: Preferences → Proper Case shortcut, or `shortcut_proper_case` in `npp-rs/settings.json` (default `Cmd+Alt+9`). Runs Edit → Convert Case to → Proper Case. **Shift** flips to Proper Case blend. Invert Case + Shift still aliases Proper Case force. Does not collide with Sentence Case (`Cmd+Alt+8`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective bindings.

## [0.3.343] — 2026-10-16

Edit:

- **Sentence Case Force / Blend** is remappable: Preferences → Sentence Case shortcut, or `shortcut_sentence_case` in `npp-rs/settings.json` (default `Cmd+Alt+8`). Runs Edit → Convert Case to → Sentence case. **Shift** flips to Sentence case blend. Does not collide with Sort Lines Locale (`Cmd+Alt+7`), Invert Case (`Cmd+Alt+U`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective bindings.

## [0.3.342] — 2026-10-16

Edit:

- **Sort Lines Locale Asc / Desc** is remappable: Preferences → Sort Lines Locale shortcut, or `shortcut_sort_lines_locale` in `npp-rs/settings.json` (default `Cmd+Alt+7`). Runs Edit → Line Operations → Sort Lines Locale Ascending. **Shift** flips to descending. Does not collide with Sort Lines Reverse (`Cmd+Alt+6`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective bindings.

## [0.3.341] — 2026-10-16

Edit:

- **Sort Lines Reverse Order / Randomly** is remappable: Preferences → Sort Lines Reverse shortcut, or `shortcut_sort_lines_reverse` in `npp-rs/settings.json` (default `Cmd+Alt+6`). Runs Edit → Line Operations → Sort Lines in Reverse Order. **Shift** flips to Sort Lines Randomly. Does not collide with Next Style (`Cmd+Alt+1`…`5`) or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective bindings.

## [0.3.340] — 2026-10-16

Format:

- **Convert EOL Unix (LF) / Windows (CRLF)** is remappable: Preferences → Convert EOL shortcut, or `shortcut_convert_eol` in `npp-rs/settings.json` (default `Cmd+Alt+F12`). Runs Format → Convert to Unix. **Shift** flips to Convert to Windows. Macintosh (CR) stays menu-only. Does not collide with Post-It (`F12`) or EOL to Space (`Cmd+Alt+F11`). Shortcut Mapper and About show the effective bindings.

## [0.3.339] — 2026-10-16

Edit:

- **EOL to Space / Trim Both and EOL to Space** is remappable: Preferences → EOL to Space shortcut, or `shortcut_eol_to_space` in `npp-rs/settings.json` (default `Cmd+Alt+F11`). Runs Edit → Blank Operations → EOL to Space. **Shift** flips to Trim Both and EOL to Space. Does not collide with Fullscreen (`F11`), Sort Lines by Length (`Cmd+Alt+F10`), or Trim Trailing (`Cmd+Alt+T`). Shortcut Mapper and About show the effective bindings.

## [0.3.338] — 2026-10-16

Edit:

- **Sort Lines by Length Asc / Desc** is remappable: Preferences → Sort Lines by Length shortcut, or `shortcut_sort_lines_length` in `npp-rs/settings.json` (default `Cmd+Alt+F10`). Runs Edit → Line Operations → Sort Lines by Length Ascending. **Shift** flips to descending. Does not collide with Sort Lines Decimal Comma (`Cmd+Alt+F9`) or First Hidden Equal (`Cmd+Alt+F7`). Shortcut Mapper and About show the effective bindings.

## [0.3.337] — 2026-10-16

Edit:

- **Sort Lines Decimal Comma Asc / Desc** is remappable: Preferences → Sort Lines Decimal Comma shortcut, or `shortcut_sort_lines_decimal_comma` in `npp-rs/settings.json` (default `Cmd+Alt+F9`). Runs Edit → Line Operations → Sort Lines As Decimals (Comma) Ascending. **Shift** flips to descending. Does not collide with Find characters in range (`Alt+F9`) or Sort Lines Decimal Dot (`Cmd+Alt+F8`). Shortcut Mapper and About show the effective bindings.

## [0.3.336] — 2026-10-16

Edit:

- **Sort Lines Decimal Dot Asc / Desc** is remappable: Preferences → Sort Lines Decimal Dot shortcut, or `shortcut_sort_lines_decimal_dot` in `npp-rs/settings.json` (default `Cmd+Alt+F8`). Runs Edit → Line Operations → Sort Lines As Decimals (Dot) Ascending. **Shift** flips to descending. Does not collide with Next Change (`F8`), Find Purge (`Alt+Shift+F8`), Clear Change History (`Cmd+Shift+F8`), or Sort Lines Ignoring Case (`Cmd+Alt+F6`). Shortcut Mapper and About show the effective bindings.

## [0.3.335] — 2026-10-15

Edit:

- **Sort Lines Lex Ignoring Case Asc / Desc** is remappable: Preferences → Sort Lines Ignoring Case shortcut, or `shortcut_sort_lines_case_insensitive` in `npp-rs/settings.json` (default `Cmd+Alt+F6`). Runs Edit → Line Operations → Sort Lines Lex. Ascending Ignoring Case. **Shift** flips to descending ignoring case. Does not collide with Focus on Another View (`F6`), Find Cut (`Alt+Shift+F6`), or Sort Lines Integer (`Cmd+Alt+F5`). Shortcut Mapper and About show the effective bindings.

## [0.3.334] — 2026-10-15

Edit:

- **Sort Lines Integer Asc / Desc** is remappable: Preferences → Sort Lines Integer shortcut, or `shortcut_sort_lines_integer` in `npp-rs/settings.json` (default `Cmd+Alt+F5`). Runs Edit → Line Operations → Sort Lines As Integers Ascending. **Shift** flips to descending. Does not collide with Date Time (`F5`), Find Copy (`Alt+Shift+F5`), or Sort Lines Lexicographic (`Cmd+Alt+O`). Shortcut Mapper and About show the effective bindings.

## [0.3.333] — 2026-10-15

Search:

- **Find Next / Previous (Volatile)** uses the current selection as an ephemeral literal query and does **not** change the Find dialog / `find_query`. Honors Case / Word; ignores Re and Sel (whole document). Remappable via Preferences → Find Next (Volatile) shortcut, or `shortcut_find_volatile` in `npp-rs/settings.json` (default `Cmd+Alt+F3`; **Shift** = previous). Distinct from Select and Find (`Cmd+F3`), which copies the selection into the Find query. Does not collide with Find next (`F3`), Find Mark (`Alt+Shift+F3`), or Find All (`Cmd+Alt+F4`). Shortcut Mapper and About show the effective bindings.

## [0.3.332] — 2026-10-15

Search:

- **Find characters in range** is remappable: Preferences → Find characters in range shortcut, or `shortcut_find_char_range` in `npp-rs/settings.json` (default `Alt+F9`). Runs Search → Find characters in range (Find text = `ascii` / `non-ascii` / `start-end` such as `65-90`). **Shift** flips to the previous character in range. Chord parsing now accepts `F9` / `F10`. Does not collide with Find Purge (`Alt+Shift+F8` / `Alt+F8`), Next Change (`F8`), or Fullscreen (`F11`). Shortcut Mapper and About show the effective bindings.

## [0.3.331] — 2026-10-15

Search:

- **Find Purge** is remappable: Preferences → Find Purge shortcut, or `shortcut_find_purge` in `npp-rs/settings.json` (default `Alt+Shift+F8`). Opens Find, seeds from selection, deletes every line that contains a Find match with Case/Word/Re/Sel (same as Find-bar **Purge**; no clipboard). **Shift** flips to Purge matching substrings (Find-bar **Shift+Purge**). Does not collide with Next Hidden Equal (`Alt+F7` / `Alt+Shift+F7`), Next Change (`F8`), Clear Change History (`Cmd+Shift+F8`), or Find Cut (`Alt+Shift+F6`). Shortcut Mapper and About show the effective bindings.

## [0.3.330] — 2026-10-15

Search:

- **Find Cut** is remappable: Preferences → Find Cut shortcut, or `shortcut_find_cut` in `npp-rs/settings.json` (default `Alt+Shift+F6`). Opens Find, seeds from selection, cuts matching substrings to the clipboard and deletes them with Case/Word/Re/Sel (same as Find-bar **Cut**). **Shift** flips to Cut matching lines (Find-bar **Shift+Cut**). Does not collide with Focus on Another View (`F6`), Find Copy (`Alt+Shift+F5`), or Find Bookmark (`Alt+Shift+F4`). Shortcut Mapper and About show the effective bindings.

## [0.3.329] — 2026-10-15

Search:

- **Find Copy** is remappable: Preferences → Find Copy shortcut, or `shortcut_find_copy` in `npp-rs/settings.json` (default `Alt+Shift+F5`). Opens Find, seeds from selection, copies matching substrings with Case/Word/Re/Sel (same as Find-bar **Copy**). **Shift** flips to Copy matching lines (Find-bar **Lines**). Does not collide with Date Time (`F5`), Find Mark (`Alt+Shift+F3`), or Find Bookmark (`Alt+Shift+F4`). Shortcut Mapper and About show the effective bindings.

## [0.3.328] — 2026-10-15

Search:

- **Find Bookmark** is remappable: Preferences → Find Bookmark shortcut, or `shortcut_find_bookmark` in `npp-rs/settings.json` (default `Alt+Shift+F4`). Opens Find, seeds from selection, bookmarks all matching lines with Case/Word/Re/Sel (same as Find-bar **Bookmark**; keeps existing). Does not collide with Next Search Result (`F4`), Find All (`Cmd+Alt+F4`), Find Mark (`Alt+Shift+F3`), or Inverse Bookmarks (`Alt+Shift+F2`). Shortcut Mapper and About show the effective binding.

## [0.3.327] — 2026-10-15

Search:

- **Find Mark** is remappable: Preferences → Find Mark shortcut, or `shortcut_find_mark` in `npp-rs/settings.json` (default `Alt+Shift+F3`). Runs Search → Mark (opens Find, seeds from selection, Style 1 mark-all with Case/Word/Re/Sel; same as Find-bar **Mark**). Does not collide with Find next (`F3`), Select and Find (`Cmd+F3`), or Inverse Bookmarks (`Alt+Shift+F2`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.326] — 2026-10-15

Search:

- **Mark current line Style 1…5** is remappable: Preferences → Mark current line style shortcut, or `shortcut_mark_one_style` in `npp-rs/settings.json` (default `Alt+Shift+1`). Digit-family chords accept `1`…`5` with those modifiers (mark the caret line with that style; Search → Mark → Using 1st…5th Style). Letter remaps mark Style 1 only. Does not collide with Mark Style (`Cmd+1`…`5`), Next Style (`Cmd+Alt+1`…`5`), Copy Styled (`Cmd+Alt+Shift+1`…`5`), or Fold Level (`Alt+1`…`8`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.325] — 2026-10-15

Search:

- **Copy Styled Text** is remappable: Preferences → Copy styled text shortcut, or `shortcut_style_to_clip` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+1`). Digit-family chords accept `1`…`5` with those modifiers (copy that style's lines to the clipboard). Same mods + `0` copies all styled lines (Search → Copy All Styles to Clipboard). Letter remaps: base = Style 1; Shift = all. Does not collide with Mark Style (`Cmd+1`…`5`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.324] — 2026-10-15

Search:

- **Clear Change History** is remappable: Preferences → Clear change history shortcut, or `shortcut_clear_change_history` in `npp-rs/settings.json` (default `Cmd+Shift+F8`). Clears amber/green change-history marks on the active tab (Search → Clear Change History). Stays near Next/Prev Change (`F8` / `Shift+F8`) without colliding (those chords have no Cmd). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.323] — 2026-10-15

Search:

- **Incremental Search** is remappable: Preferences → Incremental Search shortcut, or `shortcut_find_incremental` in `npp-rs/settings.json` (default `Alt+Shift+I`). Opens Find and enables as-you-type jump (same as Search → Incremental Search / Find-bar **Inc**). Does not collide with Indent Guide (`Alt+I`), Format Document (`Cmd+Shift+I`), or Close All to the Left (`Cmd+Alt+I`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.322] — 2026-10-15

Search:

- **Remove Unmarked Lines** is remappable: Preferences → Remove unmarked lines shortcut, or `shortcut_remove_unmarked` in `npp-rs/settings.json` (default `Cmd+Alt+Z`). Runs Search → Remove Unmarked Lines (keeps bookmarked lines only). Does not collide with Word Wrap (`Alt+Z`), Zoom Sync (`Alt+Shift+Z`), or Paste Bookmarked (`Cmd+Alt+Q`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.321] — 2026-10-15

Search:

- **Incremental Search**: Search → Incremental Search (or Find-bar **Inc**) enables as-you-type jump: each Find-field edit selects the first match at or after the selection start (growing the query lengthens in place). Closing Find clears Inc. Case / Word / Re / Sel / Wrap changes re-run while Inc is on.

## [0.3.320] — 2026-10-15

Search:

- **Find Results / Find in Files Shift+Enter**: with focus on a results tab (primary or dual-view secondary), **Shift+Enter** runs Previous Search Result (same as Shift+F4). Plain **Enter** still jumps the hit under the caret.

## [0.3.319] — 2026-10-15

Search:

- **Next / Previous Style marker** is remappable: Preferences → Next style marker shortcut, or `shortcut_next_style` in `npp-rs/settings.json` (default `Cmd+Alt+1`). Digit-family chords accept `1`…`5` with those modifiers (next that style). **Shift** flips to previous style. Does not collide with Mark Style (`Cmd+1`…`5`) or Fold Level (`Alt+1`…`8`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.318] — 2026-10-15

Search:

- **Replace bar Enter / Shift+Enter**: with focus in the Replace field, **Enter** runs Replace once (same as the Replace button); **Shift+Enter** runs Replace All. Hover tips on the field and buttons mention the chords.

## [0.3.317] — 2026-10-15

Search:

- **Find bar Shift+Enter**: with focus in the Find field, **Enter** still runs Find Next; **Shift+Enter** runs Find Previous (same as the Prev button). Hover tips on the field and Next/Prev mention the chords.

## [0.3.316] — 2026-10-15

Search:

- **Dual-view secondary pane** double-click on **Find Results** / **Find in Files** hit rows opens the source and parks/selects the match (same as primary-pane double-click and Enter). Works when the results tab is not the active primary tab.

## [0.3.315] — 2026-10-15

Search:

- **Enter-to-jump** on **Find Results** / **Find in Files**: with the caret on a hit row, Enter opens the source and parks/selects the match (same path as double-click). Header lines stay inert (read-only). Next/Prev Search Result (F4) unchanged.

## [0.3.314] — 2026-10-15

Search:

- **Find All / Search Results Window** is remappable: Preferences → Find All shortcut, or `shortcut_find_all` in `npp-rs/settings.json` (default `Cmd+Alt+F4`). Runs Search → Search Results Window (Find All current document). **Shift** flips to Find All in opened editor tabs. Does not collide with Next Search Result (`F4`) or Find in Files (`Cmd+Alt+F`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.313] — 2026-10-15

Search:

- **Click-to-jump** on **Find Results** / **Find in Files**: double-click a hit row (`line:col:match` or `path:line:text`) opens the source (from the workspace when needed) and parks/selects the match. Header lines still select a word on double-click. Next/Prev Search Result (F4) unchanged.

## [0.3.312] — 2026-10-15

Search:

- **Next / Previous Search Result** also walks the **Find in Files** tab: parks each `path:line:text` hit, opens the file from the workspace root when needed, selects the Find match on that line, and wraps with a status `· wrapped` bit. Find Results (Find All) walk is unchanged.

## [0.3.311] — 2026-10-15

Search:

- **Next / Previous Search Result** (F4 / Shift+F4, remappable `shortcut_next_found`) walks the **Find Results** tab from Find All / Shift+All: parks and selects each `line:col:match` in the source document (multi-doc sections too), wraps with a status `· wrapped` bit. Without a Find Results tab (or parseable hits), falls back to Find Next / Prev.

## [0.3.310] — 2026-10-15

Search:

- **Find All in opened documents** — Find bar **All** with **Shift** searches every open editor tab (skips Find Results / Find in Files) and refreshes the **Find Results** tab with per-document sections (Case / Word / Re; Sel ignored for multi-doc). Plain **All** stays current document only.

## [0.3.309] — 2026-10-15

Search:

- **Find All (current document)** — Find bar **All** opens (or refreshes) a read-only **Find Results** tab listing every match as `line:col:text` (Case / Word / Re / Sel; same engine as Find Next). Search → Search Results Window runs the same. Re-running while on the results tab searches the previous editor tab.

## [0.3.308] — 2026-10-15

Search:

- **Find Purge matching substrings** — Find bar **Purge** with **Shift** deletes every matching substring (Case / Word / Re / Sel; same engine as Find Next / Replace All) without touching the clipboard. Distinct from plain **Purge** (whole matching lines) and from **Cut** (clipboard + delete substrings).

## [0.3.307] — 2026-10-14

Search:

- **Find Cut matching lines** — Find bar **Cut** with **Shift** copies every line that contains a Find match to the clipboard and deletes those lines (Case / Word / Re / Sel; same engine as Find Next), newline-joined, deduped. Whole lines (body only), distinct from plain **Cut** (match substrings) and from **Purge** (delete lines without clipboard).

## [0.3.306] — 2026-10-14

Search:

- **Find Copy matching lines** — Find bar **Lines** copies every line that contains a Find match (Case / Word / Re / Sel; same engine as Find Next), newline-joined, deduped. Whole lines (body only), distinct from **Copy** (match substrings) and Copy Bookmarked Lines.

## [0.3.305] — 2026-10-14

Search:

- **Find Purge matching lines** — Find bar **Purge** deletes every line that contains a Find match (Case / Word / Re / Sel; same engine as Find Next). Whole lines, not substrings. Distinct from Cut (match text only) and Remove Bookmarked Lines.

## [0.3.304] — 2026-10-14

Search:

- **Find Cut matches** — Find bar **Cut** copies every matching substring to the clipboard and deletes those matches (Case / Word / Re / Sel; same engine as Find Next / Replace All), newline-joined. Distinct from Cut Bookmarked Lines (whole lines).

## [0.3.303] — 2026-10-14

Search:

- **Find Copy matches** — Find bar **Copy** copies every matching substring to the clipboard (Case / Word / Re / Sel; same engine as Find Next), newline-joined. Distinct from Copy Bookmarked Lines (whole lines).

## [0.3.302] — 2026-10-14

Search:

- **Find Bookmark all** — Find bar **Bookmark** bookmarks every matching line (Case / Word / Re / Sel; same engine as Find Next). Keeps existing bookmarks; status reports match lines and how many were new.

## [0.3.301] — 2026-10-14

Search:

- **Find Mark All** respects Case / Word / Re / Sel (same engine as Find Next). Find bar **Mark** styles matching lines with Style 1. Search → Mark runs the same when Find text is set. Style All Occurrences (Ctrl/Cmd+1…5) uses a selection token as a literal (Case/Word) or the Find query with full flags when nothing is selected.

## [0.3.300] — 2026-10-14

Settings / shortcuts:

- **Space to TAB (All)** is remappable: Preferences → Space to TAB (All) shortcut, or `shortcut_space_to_tab_all` in `npp-rs/settings.json` (default `Cmd+Alt+X`). Runs Edit → Space to TAB (All) (every four spaces → one tab). Does not collide with Expand Unchanged at Caret (`Alt+X`) or TAB to Space (`Cmd+Alt+V`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.299] — 2026-10-14

Settings / shortcuts:

- **Paste / Remove Bookmarked Lines** is remappable: Preferences → Paste bookmarked lines shortcut, or `shortcut_paste_marked_lines` in `npp-rs/settings.json` (default `Cmd+Alt+Q`). Runs Search → Paste Bookmarked Lines. **Shift** flips to Remove Bookmarked Lines. Does not collide with Sync Vertical Scroll (`Alt+Q`), Paste (`Cmd+V`), or Copy Bookmarked Lines (`Cmd+Alt+F2`). Remove Unmarked stays menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.298] — 2026-10-14

Settings / shortcuts:

- **Copy / Cut Bookmarked Lines** is remappable: Preferences → Copy bookmarked lines shortcut, or `shortcut_copy_marked_lines` in `npp-rs/settings.json` (default `Cmd+Alt+F2`). Runs Search → Copy Bookmarked Lines. **Shift** flips to Cut Bookmarked Lines. Stays on the F2 bookmark family without colliding with next (`F2`), toggle (`Cmd+F2`), clear (`Cmd+Shift+F2`), or inverse (`Alt+Shift+F2`). Paste / Remove bookmarked lines stay menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.297] — 2026-10-14

Settings / shortcuts:

- **TAB to Space / Space to TAB (Leading)** is remappable: Preferences → TAB to Space shortcut, or `shortcut_tab_to_space` in `npp-rs/settings.json` (default `Cmd+Alt+V`). Runs Edit → TAB to Space (each tab → four spaces). **Shift** flips to Space to TAB (Leading). Space to TAB (All) stays menu-only. Does not collide with Paste (`Cmd+V`), Show White Space (`Alt+V`), or Trim Trailing (`Cmd+Alt+T`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.296] — 2026-10-14

Settings / shortcuts:

- **First / Last Tab** is remappable: Preferences → First tab shortcut, or `shortcut_first_tab` in `npp-rs/settings.json` (default `Cmd+Alt+Home`). Runs View → First Tab. Opposite **End** jumps to Last Tab; letter remaps use **Shift** for last. Does not collide with Document Home (`Cmd+Home`) or Next tab (`Cmd+Tab`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.295] — 2026-10-14

Settings / shortcuts:

- **Next / Previous Search Result** is remappable: Preferences → Next search result shortcut, or `shortcut_next_found` in `npp-rs/settings.json` (default `F4`). Runs Search → Next Search Result. **Shift** flips to Previous Search Result. Chord parser accepts `F4`. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.294] — 2026-10-14

Settings / shortcuts:

- **Path autocomplete** is remappable: Preferences → Path autocomplete shortcut, or `shortcut_autocomplete_path` in `npp-rs/settings.json` (default `Cmd+Alt+Space`). Runs Edit → Function Completion Path. Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.293] — 2026-10-14

Settings / shortcuts:

- **Autocomplete / Function Call Tip** is remappable: Preferences → Autocomplete shortcut, or `shortcut_autocomplete` in `npp-rs/settings.json` (default `Cmd+Space`). Runs Edit → Function Completion File. **Shift** flips to Function Call Tip. Path completion stayed menu-only until v0.3.294. Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.292] — 2026-10-14

Settings / shortcuts:

- **Fold level 1…8** is remappable: Preferences → Fold level shortcut, or `shortcut_fold_level` in `npp-rs/settings.json` (default `Alt+1`). Digit-family chords accept `1`…`8` with those modifiers (fold that indent level). **Shift** flips to Unfold Level. `Alt+0` stays Fold All. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.291] — 2026-10-14

Settings / shortcuts:

- **Rect column select** is remappable: Preferences → Rect column select shortcut, or `shortcut_rect_select` in `npp-rs/settings.json` (default `Alt+Shift+Left`). Arrow-family chords accept any arrow with those modifiers (Left/Right grow columns; Up/Down grow rows). Letter remaps: base = left, Shift-flipped partner = right. On the default chord, Alt+Shift+arrows still prefer rect over word-jump extend. Shortcut Mapper and About show the effective binding.

## [0.3.290] — 2026-10-14

Settings / shortcuts:

- **Zoom Sync** is remappable: Preferences → Zoom Sync shortcut, or `shortcut_zoom_sync` in `npp-rs/settings.json` (default `Alt+Shift+Z`). Toggles dual-view shared font size. Menu, Shortcut Mapper, and About show the effective binding. Stays near word wrap (`Alt+Z`); Sync H keeps `Alt+Shift+Q`.

## [0.3.289] — 2026-10-14

Settings / shortcuts:

- **Inverse Bookmarks** is remappable: Preferences → Inverse bookmarks shortcut, or `shortcut_inverse_bookmarks` in `npp-rs/settings.json` (default `Alt+Shift+F2`). Inverts bookmarks on every line in the active tab. Menu, Shortcut Mapper, and About show the effective binding. Stays on the F2 family without colliding with next/prev/toggle/clear.

## [0.3.288] — 2026-10-14

Settings / shortcuts:

- **Clear all styles** is remappable: Preferences → Clear all styles shortcut, or `shortcut_clear_all_styles` in `npp-rs/settings.json` (default `Cmd+Shift+0`). Clears every style mark (1–5) in the active tab. Menu, Shortcut Mapper, and About show the effective binding. `Cmd+0` stays zoom restore; `Alt+0` stays fold all.

## [0.3.287] — 2026-10-14

Settings / shortcuts:

- **Mark Style 5** is remappable: Preferences → Mark Style 5 shortcut, or `shortcut_mark_style_5` in `npp-rs/settings.json` (default `Cmd+5`). Marks all Find/selection matches with Style 5. **Shift** flips to Clear Style 5. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.286] — 2026-10-14

Settings / shortcuts:

- **Mark Style 4** is remappable: Preferences → Mark Style 4 shortcut, or `shortcut_mark_style_4` in `npp-rs/settings.json` (default `Cmd+4`). Marks all Find/selection matches with Style 4. **Shift** flips to Clear Style 4. Style 5 stays menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.285] — 2026-10-14

Settings / shortcuts:

- **Mark Style 3** is remappable: Preferences → Mark Style 3 shortcut, or `shortcut_mark_style_3` in `npp-rs/settings.json` (default `Cmd+3`). Marks all Find/selection matches with Style 3. **Shift** flips to Clear Style 3. Styles 4–5 stay menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.284] — 2026-10-14

Settings / shortcuts:

- **Mark Style 2** is remappable: Preferences → Mark Style 2 shortcut, or `shortcut_mark_style_2` in `npp-rs/settings.json` (default `Cmd+2`). Marks all Find/selection matches with Style 2. **Shift** flips to Clear Style 2. Styles 3–5 stay menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.283] — 2026-10-14

Settings / shortcuts:

- **Mark Style 1** is remappable: Preferences → Mark Style 1 shortcut, or `shortcut_mark_style` in `npp-rs/settings.json` (default `Cmd+1`). Marks all Find/selection matches with Style 1. **Shift** flips to Clear Style 1. Styles 2–5 stay menu-only. Menu, Shortcut Mapper, and About show the effective bindings. Chord parser now accepts digit keys `1`–`5`.

## [0.3.282] — 2026-10-14

Settings / shortcuts:

- **Select and Find Next** is remappable: Preferences → Select and Find shortcut, or `shortcut_set_and_find` in `npp-rs/settings.json` (default `Cmd+F3`). Copies the selection into the find query then finds next. **Shift** flips to Select and Find Previous. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.281] — 2026-10-14

Settings / shortcuts:

- **Move Current Tab Forward** is remappable: Preferences → Move tab shortcut, or `shortcut_move_tab` in `npp-rs/settings.json` (default `Cmd+Shift+PageDown`). Opposite PageUp moves the tab backward; letter remaps use Shift for backward. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.280] — 2026-10-13

Settings / shortcuts:

- **Function List** is remappable: Preferences → Function List shortcut, or `shortcut_function_list` in `npp-rs/settings.json` (default `Cmd+E`). **Shift** flips to Character Panel. Both panels toggle open/closed. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.279] — 2026-10-13

Settings / shortcuts:

- **Save a Copy As** is remappable: Preferences → Save a Copy As shortcut, or `shortcut_save_copy` in `npp-rs/settings.json` (default `Cmd+Alt+Y`). **Shift** flips to Rename. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.278] — 2026-10-13

Settings / shortcuts:

- **Close All to the Left** is remappable: Preferences → Close All to Left shortcut, or `shortcut_close_to_side` in `npp-rs/settings.json` (default `Cmd+Alt+I`). **Shift** flips to Close All to the Right. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.277] — 2026-10-13

Settings / shortcuts:

- **Close All but Pinned** is remappable: Preferences → Close All but Pinned shortcut, or `shortcut_close_but_pinned` in `npp-rs/settings.json` (default `Cmd+Alt+H`). **Shift** flips to Close All Unchanged. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.276] — 2026-10-13

Settings / shortcuts:

- **Open Folder as Workspace** is remappable: Preferences → Open workspace shortcut, or `shortcut_open_workspace` in `npp-rs/settings.json` (default `Cmd+Alt+G`). **Shift** flips to Open Containing Folder as Workspace. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.275] — 2026-10-13

Settings / shortcuts:

- **Open Containing Folder** is remappable: Preferences → Open folder shortcut, or `shortcut_open_folder` in `npp-rs/settings.json` (default `Cmd+Alt+L`). **Shift** flips to open a shell in the active file's directory. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.274] — 2026-10-13

Settings / shortcuts:

- **Open Selected Filename** is remappable: Preferences → Open selected shortcut, or `shortcut_open_selected` in `npp-rs/settings.json` (default `Cmd+Alt+J`). **Shift** flips to Open Containing Folder in the file manager. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.273] — 2026-10-13

Settings / shortcuts:

- **Copy All Paths** is remappable: Preferences → Copy all paths shortcut, or `shortcut_copy_all_paths` in `npp-rs/settings.json` (default `Cmd+Alt+M`). **Shift** flips to Copy All Filenames. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.272] — 2026-10-13

Settings / shortcuts:

- **Copy Current Directory Path** is remappable: Preferences → Copy directory shortcut, or `shortcut_copy_dir` in `npp-rs/settings.json` (default `Cmd+Alt+K`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.271] — 2026-10-13

Settings / shortcuts:

- **Copy Full Path** is remappable: Preferences → Copy path shortcut, or `shortcut_copy_path` in `npp-rs/settings.json` (default `Cmd+Alt+P`). **Shift** flips to Copy File Name. Current directory stayed menu-only until v0.3.272. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.270] — 2026-10-13

Settings / shortcuts:

- **Date Time (short)** is remappable: Preferences → Date Time shortcut, or `shortcut_insert_datetime` in `npp-rs/settings.json` (default `F5`). **Shift** flips to Date Time (long). Customized format stays menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.269] — 2026-10-13

Settings / shortcuts:

- **Remove Consecutive Duplicate Lines** is remappable: Preferences → Remove Duplicate Lines shortcut, or `shortcut_remove_dup_lines` in `npp-rs/settings.json` (default `Cmd+Alt+D`). **Shift** flips to Remove Duplicate Lines (any occurrence). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.268] — 2026-10-13

Settings / shortcuts:

- **Remove Empty Lines** is remappable: Preferences → Remove Empty Lines shortcut, or `shortcut_remove_empty_lines` in `npp-rs/settings.json` (default `Cmd+Alt+E`). **Shift** flips to Remove Empty Lines (Containing Blank Characters). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.267] — 2026-10-13

Settings / shortcuts:

- **Sort Lines Lexicographic Ascending** is remappable: Preferences → Sort Lines shortcut, or `shortcut_sort_lines` in `npp-rs/settings.json` (default `Cmd+Alt+O`). **Shift** flips to Lexicographic Descending. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.266] — 2026-10-13

Settings / shortcuts:

- **Column Editor** is remappable: Preferences → Column Editor shortcut, or `shortcut_column_editor` in `npp-rs/settings.json` (default `Cmd+Alt+C`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.265] — 2026-10-13

Settings / shortcuts:

- **Toggle Read-Only** is remappable: Preferences → Toggle read-only shortcut, or `shortcut_toggle_readonly` in `npp-rs/settings.json` (default `Cmd+Alt+R`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.264] — 2026-10-13

Edit / column mode:

- **Document Home/End** (Cmd/Ctrl+Home/End and Cmd/Ctrl+Up/Down) move multi-carets / rect blocks to the document edge; **Shift** extends stream multi-carets (keep anchors) and grows rect lines to the first/last document line (`docs/column-mode.md`).

## [0.3.263] — 2026-10-13

Edit / column mode:

- **Alt+Shift+arrows** start or extend a rectangular column selection from the caret (N++-style keyboard column select), including virtual columns past EOL. Prefer Up/Down first for a multi-line block; Left/Right then grow columns. On the default Alt+Left/Right word-jump chord, Alt+Shift+Left/Right prefers rect over word-jump extend.

## [0.3.262] — 2026-10-13

Edit / column mode:

- **Shift+Home / Shift+End** extend rectangular selections (keep Alt+drag anchor corner; Home → column 0, End → that line’s body end), same idea as Shift+Left/Right on a rect.

## [0.3.261] — 2026-10-13

Edit / column mode:

- **Begin/End Select in Column Mode** keeps virtual columns past EOL on both marks (free-caret space), so the rect matches Alt+drag past EOL. Stream Begin/End Select can also end in virtual space.

## [0.3.260] — 2026-10-13

Edit / column mode / shortcuts:

- **Begin/End Select in Column Mode** builds a real rectangular selection between the marked start caret and the current caret (same corners as Alt+drag). Stream Begin/End Select is unchanged.
- **Begin/End Select** is remappable: Preferences → Begin/End Select shortcut, or `shortcut_begin_end_select` in `npp-rs/settings.json` (default `Cmd+Alt+B`). Shift flips to **Begin/End Select in Column Mode**. Menu, Shortcut Mapper, and About show both bindings.

## [0.3.259] — 2026-10-13

Settings / shortcuts:

- **Multi-select All** is remappable: Preferences → Multi-select all shortcut, or `shortcut_multi_select_all` in `npp-rs/settings.json` (default `Cmd+Alt+A`). Shift flips to **Multi-select All (Whole Word)**. Menu, Shortcut Mapper, and About show both bindings.

## [0.3.258] — 2026-10-13

Edit / column mode:

- **Multi-caret / rect paste line-split**: when the clipboard has the same number of lines as carets (typical after a multi-caret or column copy), each caret gets its matching line. Mismatched line counts still paste the full string at every caret (`docs/column-mode.md`).

## [0.3.257] — 2026-10-13

Settings / shortcuts:

- **Trim Trailing Space** is remappable: Preferences → Trim Trailing shortcut, or `shortcut_trim_trailing` in `npp-rs/settings.json` (default `Cmd+Alt+T`). Shift flips to **Trim Leading and Trailing Space**. Menu, Shortcut Mapper, and About show both bindings.

## [0.3.256] — 2026-10-13

Settings / shortcuts:

- **Close All** is remappable: Preferences → Close All shortcut, or `shortcut_close_all` in `npp-rs/settings.json` (default `Cmd+Alt+W`). Shift flips to **Close All but Current**. Menu, Shortcut Mapper, and About show both bindings.

## [0.3.255] — 2026-10-13

Edit / column mode:

- **Escape clears multi-carets**: remappable close-Find chord (default Escape) closes Find/Replace when open; when closed, clears multi-carets / column select (`docs/column-mode.md`).

## [0.3.254] — 2026-10-12

Edit / column mode:

- **Multi-caret Page Up/Down**: remappable Page Up/Down (default PageUp/PageDown) moves every multi-caret / rect block by a page (±30 lines); Shift extends stream multi-carets (and rect Shift+extend). Same path as plain Up/Down, larger delta (`docs/column-mode.md`).

## [0.3.253] — 2026-10-12

Edit / column mode:

- **Multi-caret line Home/End**: remappable line Home/End (default Home/End) moves every multi-caret to that line’s start or body end; Shift extends stream multi-carets. Rect selections become stream multi after a plain Home/End (`docs/column-mode.md`).

## [0.3.252] — 2026-10-12

Edit / column mode:

- **Multi-caret word-jump**: remappable word-jump (default Alt+Left/Right) moves every multi-caret together; Shift extends stream multi-carets. Rect selections become stream multi after a plain word-jump (`docs/column-mode.md`).

## [0.3.251] — 2026-10-12

Edit / column mode:

- **Rect Shift+extend**: Shift+Left/Right/Up/Down grow or shrink an Alt+drag rectangular selection (anchor corner kept, caret corner moves, including past EOL). Collapsing to one line becomes a stream selection (`docs/column-mode.md`).

## [0.3.250] — 2026-10-12

Edit / column mode:

- **Shift+extend multi-carets**: Shift+Left/Right/Up/Down extend every stream multi-caret selection (anchor kept, caret moves). Typing replaces each extended range. Rect Alt+drag still clears on Shift (`docs/column-mode.md`).

## [0.3.249] — 2026-10-12

Edit / column mode:

- **Ctrl/Cmd+click add carets**: click with Ctrl (Cmd on macOS) adds a zero-width caret; click an existing caret again to remove it. Seeds the previous caret/selection into the multi-caret set.

## [0.3.248] — 2026-10-12

Edit / column mode:

- **Multi-caret arrow move**: plain Left/Right/Up/Down move every multi-caret / rect caret together (ranges collapse to an edge first; rect keeps virtual columns). Shift / word-jump / document jumps still clear extras (`docs/column-mode.md`).

## [0.3.247] — 2026-10-12

Edit / column mode:

- **Stream select into virtual space**: Shift+click / drag (and plain drag) past EOL extends the selection wash and caret; Shift+Right / Shift+Left grow or shrink virtual space at EOL. Buffer selection still clamps to real characters (`docs/column-mode.md`).

## [0.3.246] — 2026-10-12

Edit / column mode:

- **Plain free caret past EOL**: click or Right past the line end places a virtual caret; paint follows; typing / paste / Tab pad with spaces; Backspace / Left shrink virtual space; Up/Down keep the preferred column (`docs/column-mode.md`).

## [0.3.245] — 2026-10-12

Edit / column mode:

- **Alt+drag past EOL**: mouse hit-test can place rectangular carets / selection past every line’s end; carets and the wash draw in that virtual space; typing still pads with spaces (`docs/column-mode.md`).

## [0.3.244] — 2026-10-12

Edit / column mode:

- **Virtual space** for Alt+drag rectangular selections: typing on short lines pads with spaces to the target column; copy includes space padding for columns past EOL. Carets still clamp visually until padded (`docs/column-mode.md`).

## [0.3.243] — 2026-10-12

Docs (issue #20):

- Refresh **`docs/next-gaps.md`**: drop the stale post-v0.3.12 / open-#14 table; document what shipped through v0.3.242 and rank real remaining gaps (shortcuts.xml, column virtual space, Find/FIF UI depth, LSP, macros, plugins ABI, UDL, hex, 3-way, …). Point overnight idle work at `docs/autoresearch/standing_backlog.md`.
- Bump `docs/whats-missing.md` / docs index checkpoint dates to match.

## [0.3.242] — 2026-10-12

Plugins / Ollama (issue #19):

- **Ask Ollama** and **Ollama Status** run on a background thread so long generations no longer freeze the window. Status bar shows progress; reply / status still open in a tab when ready. One request at a time.
- Editor **right-click → Ask Ollama** (same as Plugins → Ask Ollama).

## [0.3.241] — 2026-10-12

View / Preferences:

- **Post-It / Distraction Free** hides the menu and status bar (tabs and editor stay). No longer an alias for OS fullscreen (F11 stays fullscreen). State persists in `npp-rs/settings.json` (`post_it`). Menu ✓, remappable F12 (`shortcut_postit`), and Preferences checkbox restore on launch. Exit with the same chord when chrome is hidden.

## [0.3.240] — 2026-10-12

View / Preferences / Dual view:

- **Zoom Sync** persists in `npp-rs/settings.json` (`zoom_sync`). Menu toggle, Preferences checkbox, and relaunch restore shared font size across panes.
- **Sync Vertical / Horizontal / Zoom Sync** menu and remappable toggles now flip the live dual-view flags (no process-wide atomics). After session restore of `@syncv` / `@synch`, or Clear Compare restoring pre-start sync, the next click turns sync off correctly.

## [0.3.239] — 2026-10-12

File / Session / Compare:

- Opt-in session restore now keeps a **marked Compare partner** (`@partner`, alias `@cmppartner`) in `npp-rs/session.txt` when a tab was ⌘/Ctrl-clicked (or marked via the tab context menu) for Compare. Relaunch re-marks that tab so Compare with Other View still prefers it. Older session files still load (no partner mark by default).

## [0.3.238] — 2026-10-12

View / Preferences:

- **Project panel** open/closed persists in `npp-rs/settings.json` (`show_project_panel`). Menu / remappable toggle, window close, and Preferences checkbox restore the panel on launch.

## [0.3.237] — 2026-10-12

File / Session / Compare:

- Opt-in session restore now keeps **Compare on** (`@compare`, alias `@cmp`) in `npp-rs/session.txt` with dual-view `@other`. Relaunch re-opens the split and restarts Compare (parks the first hunk). Older session files still load (Compare off by default).

## [0.3.236] — 2026-10-12

View / Preferences:

- **Document Map** open/closed persists in `npp-rs/settings.json` (`show_document_map`). Menu / remappable toggle, window close, and Preferences checkbox restore the panel on launch.

## [0.3.235] — 2026-10-12

View / Preferences:

- **Always on Top** persists in `npp-rs/settings.json` (`always_on_top`). Menu / Alt+T toggle and Preferences checkbox restore the window level on launch (same pattern as `text_rtl`).

## [0.3.234] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **Synchronize Vertical / Horizontal Scrolling** (`@syncv` / `@synch`, aliases `@syncvert` / `@synchoriz`) in `npp-rs/session.txt` with dual-view `@other`. Relaunch re-opens the split with the same sync flags. Older session files still load (sync off by default).

## [0.3.233] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **dual-view other-pane focus** (`@focus`, alias `@otherfocus`) in `npp-rs/session.txt` with `@other`. Relaunch re-opens the split and puts keyboard focus on the other pane when that is how you left it. Older session files still load (primary focus remains the default).

## [0.3.232] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **dual-view other pane** (`@other`, alias `@otherview`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@style1`…`@style5`, `@caret`, `@sel`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, `@tail`, and `@active`. Move / Clone to Other View layout survives relaunch (including async opens). Older session files still load.

## [0.3.231] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **Search → Style marks** (`@style1`…`@style5` line lists) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@sel`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, `@tail`, and `@active`. Colored style marks survive relaunch (including async opens). Older session files still load.

## [0.3.230] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **selection** (`@sel A_LINE A_COL C_LINE C_COL`, anchor then caret) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, `@tail`, and `@active`. Highlighted ranges survive relaunch (including async opens; `@tail` still clears selection). Older session files still load.

## [0.3.229] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **log-tail follow** (`@tail`, alias `@logtail`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, and `@active`. View → Monitoring (tail) survives relaunch (including async opens; skips the Ask prompt when `@tail` is present). Older session files still load.

## [0.3.228] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **tab colour** (`@colour N`, alias `@color N`, 1..=5) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, and `@active`. View → Tab colour survives relaunch. Older session files still load.

## [0.3.227] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **per-tab** first visible line (`@scroll LINE`) for every open path in `npp-rs/session.txt` (not only `@active`). Tab clicks and next/prev tab restore each tab’s viewport instead of jumping to line 0. `@scroll` also survives sync open without being wiped by the post-open view reset.

## [0.3.226] — 2026-10-12

File / Session:

- Opt-in session restore now keeps the **first visible line** (`@scroll LINE`) for the active tab in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@lang`, `@encoding`, `@readonly`, `@pin`, and `@active`. Viewport scroll survives relaunch (wins over caret follow when present; including async opens). Older session files still load (no `@scroll` → scroll follows restored caret as before).

## [0.3.225] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **read-only** (`@readonly`, alias `@ro`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@lang`, `@encoding`, `@pin`, and `@active`. Edit → Toggle Read-Only survives relaunch (including async opens). Older session files still load.

## [0.3.224] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **save encoding** (`@encoding TOKEN`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@lang`, `@pin`, and `@active`. Overrides open detect after load (including async opens) so Format → Convert / ANSI vs UTF-8 choices survive relaunch. Tokens: `utf8`, `utf8-bom`, `utf16-le`, `utf16-be`, `windows-1252` (aliases accepted). Older session files still load.

## [0.3.223] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **Language menu** highlight id (`@lang ID`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@pin`, and `@active`. Overrides path-based detect after open (including async loads). Older session files still load.

## [0.3.222] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **pinned tabs** (`@pin`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, and `@active`. Older session files still load.

## [0.3.221] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **bookmarks** (`@bookmarks l1,l2,…`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@caret`, and `@active`. Lines past EOF are skipped. Older session files still load.

## [0.3.220] — 2026-10-11

View / Settings:

- **Show Fold Margin** is remappable (`shortcut_show_fold_margin`, default Alt+G). View → Show Symbol → Show Fold Margin, Preferences, Shortcut Mapper, and About show the effective binding. Persists via `show_fold_margin` as before.

## [0.3.219] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **caret line/column** (`@caret LINE COL`) and the **active tab** (`@active`) in `npp-rs/session.txt`, in addition to paths and `@folds`. Scroll follows the restored caret on launch. Older path-only / folds-only session files still load.

## [0.3.218] — 2026-10-11

View / Settings:

- **Show Line Numbers** is remappable (`shortcut_show_line_numbers`, default Alt+L). View → Show Symbol → Show Line Numbers, Preferences, Shortcut Mapper, and About show the effective binding. Persists via `show_line_numbers` as before.

## [0.3.217] — 2026-10-11

Edit / Settings:

- **RTL / LTR** is remappable (`shortcut_rtl`, default Alt+R). Shift + that chord sets LTR. Menu, Preferences, Shortcut Mapper, and About show the effective binding. Persists via `text_rtl` as before.

## [0.3.216] — 2026-10-11

View / Settings:

- **Move / Clone to Other View** is remappable (`shortcut_move_other_view`, default Alt+O). Shift + that chord clones the document to the other view. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.215] — 2026-10-11

View:

- **Dual-view other pane** paints Tree-sitter syntax colours (same viewport window as the primary pane). Edits on that tab refresh its own highlight cache; tab close / reorder remaps the cache.

## [0.3.214] — 2026-10-11

View / Settings:

- **Post-It** is remappable (`shortcut_postit`, default F12). Menu, Preferences, Shortcut Mapper, and About show the effective binding. Same distraction-free toggle as the View → Post-It command (viewport fullscreen).

## [0.3.213] — 2026-10-11

View / Settings:

- **Show Non-Printing Characters** is remappable (`shortcut_show_npc`, default Alt+N). Menu, Preferences, Shortcut Mapper, and About show the effective binding. Show All Characters (Alt+A) still toggles whitespace + EOL + NPC together.

## [0.3.212] — 2026-10-11

View / Settings:

- **Synchronize Vertical / Horizontal Scrolling** is remappable (`shortcut_sync_scroll`, default Alt+Q). Shift + that chord toggles **Synchronize Horizontal Scrolling**. Menu, Preferences, Shortcut Mapper, and About show the effective binding. Either toggle opens dual view.

## [0.3.211] — 2026-10-11

View / Settings:

- **Show White Space and TAB** is remappable (`shortcut_show_whitespace`, default Alt+V). Shift + that chord toggles **Show EOL**. Menu, Preferences, Shortcut Mapper, and About show the effective binding. Show All Characters (Alt+A) still toggles whitespace + EOL + NPC together.

## [0.3.210] — 2026-10-11

File / Settings:

- **Pin tab** is remappable (`shortcut_pin_tab`, default Alt+Shift+P). Toggles the active tab pin (`IDM_PINTAB`). Tab chrome, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.209] — 2026-10-11

View / Settings:

- **Project Panel** toggles open/closed (was open-only) and is remappable (`shortcut_project_panel`, default Alt+P). Menu, Preferences, Shortcut Mapper, and About show the effective binding. File → Open Folder as Workspace still force-opens the panel.

## [0.3.208] — 2026-10-11

View / Settings:

- **Indent Guide** is remappable (`shortcut_indent_guide`, default Alt+I). Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.207] — 2026-10-11

View / Settings:

- **Show All Characters** is remappable (`shortcut_show_all_characters`, default Alt+A). Menu, Preferences, Shortcut Mapper, and About show the effective binding. Toggles whitespace + EOL + NPC together (same as the View menu command).

## [0.3.206] — 2026-10-11

Search / Edit:

- **Drop selection into Replace**: with Replace open, drag selected text onto the Replace field to fill the replacement text (first line, max 200 chars). Source text is not moved. Orange highlight on the Replace field while hovering.

## [0.3.205] — 2026-10-11

Search / Edit:

- **Drop selection into Find**: with the Find bar open, drag selected text onto the Find field to fill the query (first line, max 200 chars). Source text is not moved. Orange highlight on the Find field while hovering.

## [0.3.204] — 2026-10-11

Edit / View:

- **Selection drag across dual view**: drag selected text from one pane into the other to move it (Ctrl/Cmd = copy). Orange drop caret follows the destination pane. Same-buffer drag is unchanged.

## [0.3.203] — 2026-10-11

View / Settings:

- **Always on Top** toggles on/off (was set-only) and is remappable (`shortcut_always_on_top`, default Alt+T). Menu, Preferences, Shortcut Mapper, and About show the effective binding. Keyboard fullscreen toggle also applies the viewport command (same path as menu).

## [0.3.202] — 2026-10-11

View / Settings:

- **Fullscreen** is remappable (`shortcut_fullscreen`, default F11). Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.201] — 2026-10-11

Edit / Settings:

- **Multi-select Next / Skip** is remappable (`shortcut_multi_select_next`, default Cmd+Alt+N). Shift + that chord skips the current match and jumps to the next. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.200] — 2026-10-11

Edit / Settings:

- **Enter / newline** is remappable (`shortcut_enter`, default Enter). When remapped, bare Enter no longer inserts a newline; the chord inserts preferred EOL. Blank line (Cmd+Enter) stays separate. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.199] — 2026-10-11

Edit / Settings:

- **RTL / LTR text direction** persists in `npp-rs/settings.json` (`text_rtl`). Edit → RTL / LTR and Preferences → Right-to-left text restore on launch (line anchors + status cue). Full bidi / UI chrome mirror still open.

## [0.3.198] — 2026-10-10

View / Compare:

- **Preferences Compare toggles** (ignore whitespace / case / blank, hide unchanged) match the View menu while Compare is on: immediate re-diff or hide apply, first-hunk park + select, and pair status with `· at L|R …` (no more delayed stale re-diff / generic “Preferences saved” only).

## [0.3.197] — 2026-10-10

View / Compare:

- **Clear Compare** restores the dual-pane sync H/V scroll flags from before Start Compare forced them on (so leaving Compare does not leave scroll stuck synced).

## [0.3.196] — 2026-10-10

Edit / Settings:

- **Document Home / End aliases** are remappable (`shortcut_document_home`, default Cmd+Home). Opposite End jumps to document end; letter remaps use Shift for end. When remapped, bare Cmd+Home/End no longer jump. Document start (Cmd+Up) stays separate. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.195] — 2026-10-10

Edit:

- **Shift+Up / Down** and **Shift+PageUp / PageDown** extend the selection while moving (same as Shift+Left/Right and Shift+Home/End). Letter remaps that use Shift for the opposite direction keep that meaning and do not also extend.

## [0.3.194] — 2026-10-10

Edit / Settings:

- **Line up / down** is remappable (`shortcut_line_up`, default Up). Opposite Down jumps one line down; letter remaps use Shift for down. When remapped, bare Up/Down no longer move by line. Move line and document start/end stay separate. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.193] — 2026-10-10

Edit / Settings:

- **Character left / right** is remappable (`shortcut_char_left`, default Left). Opposite Right jumps one character right; letter remaps use Shift for right. When remapped, bare Left/Right no longer move by character. Word jump stays separate. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.192] — 2026-10-10

Edit / Settings:

- **Page up / down** is remappable (`shortcut_page_up`, default PageUp). Opposite PageDown jumps page down; letter remaps use Shift for page down. When remapped, bare PageUp/PageDown no longer page. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.191] — 2026-10-10

Edit / Settings:

- **Line start / end** is remappable (`shortcut_line_home`, default Home). Opposite End jumps to line end; letter remaps use Shift for end. When remapped, bare Home/End no longer move on the line. `Cmd+Home` / `Cmd+End` stay hard-wired document start/end aliases. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.190] — 2026-10-10

Edit / Settings:

- **Backspace** (delete backward) is remappable (`shortcut_delete_backward`, default Backspace). When remapped, bare Backspace no longer deletes backward; the remapped chord deletes backward (same as editor Backspace). Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.189] — 2026-10-10

Edit / Settings:

- **Delete** (forward) is remappable (`shortcut_delete_forward`, default Delete). When remapped, bare Delete no longer deletes forward; the remapped chord runs Edit → Delete. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.188] — 2026-10-10

Edit / Settings:

- **Paste** is remappable (`shortcut_paste`, default Cmd+V). When remapped, platform Cmd+V no longer pastes; the remapped chord pastes last copied text (same as Edit → Paste). Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.187] — 2026-10-10

Edit / Settings:

- **Cut** is remappable (`shortcut_cut`, default Cmd+X). When remapped, platform Cmd+X no longer cuts. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.186] — 2026-10-10

Edit / Settings:

- **Copy** is remappable (`shortcut_copy`, default Cmd+C). When remapped, platform Cmd+C no longer copies. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.185] — 2026-10-10

Search / Settings:

- **Clear all bookmarks** is remappable (`shortcut_clear_bookmarks`, default Cmd+Shift+F2). Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.184] — 2026-10-10

Edit / Settings:

- **Blank line below / above** is remappable (`shortcut_blank_line`, default Cmd+Enter). Shift + that chord inserts a blank line above. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.183] — 2026-10-10

Edit / Settings:

- **Invert Case / Proper Case** is remappable (`shortcut_invert_case`, default Cmd+Alt+U). Shift + that chord runs Proper Case. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.182] — 2026-10-10

Edit / Settings:

- **Join Lines / Split Lines** is remappable (`shortcut_join_lines`, default Cmd+J). Shift + that chord runs Split Lines. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.181] — 2026-10-10

Edit / Settings:

- **lowercase / UPPERCASE** is remappable (`shortcut_lowercase`, default Cmd+U). Shift + that chord runs UPPERCASE. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.180] — 2026-10-10

Search / Settings:

- **Next / Previous Change History** is remappable (`shortcut_next_changed`, default F8). Shift + that chord jumps to the previous change-history mark. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.179] — 2026-10-10

View / Compare:

- **Focus on Another View** now toggles keyboard focus between dual-view panes (matches the menu label) instead of swapping tabs. Remappable (`shortcut_focus_other_view`, default F6). Shift + that chord swaps tabs (header Switch). Other-view header shows ` · focused` when that pane has focus, plus a Focus button. Status names the focused side (`primary` / `other`, or `L` / `R` while Compare is on).

## [0.3.178] — 2026-10-10

View / Settings:

- **Go to document start / end** is remappable (`shortcut_goto_document_start`, default Cmd+Up). Opposite Down jumps to end; letter remaps use Shift for end. Menu, Preferences, Shortcut Mapper, and About show the effective binding. `Cmd+Home` / `Cmd+End` stay hard-wired aliases.

## [0.3.177] — 2026-10-10

View:

- **Document Map** viewport band is sized to the editor page (visible rows / file lines) with a soft green wash, not a fixed ~8% mark. Title appends `· view N` when the page is shorter than the file; the band fills the map when the whole file fits.

## [0.3.176] — 2026-10-10

Search / Settings:

- **Find in Files** is remappable (`shortcut_find_in_files`, default Cmd+Alt+F). Menu, Find bar button, Preferences, Shortcut Mapper, and the chord run the workspace scan (empty Find text still opens the Find bar).

## [0.3.175] — 2026-10-10

View / Settings:

- **Document Map** toggle is remappable (`shortcut_document_map`, default Cmd+Shift+D). Menu, Preferences, Shortcut Mapper, and the chord all open/close the panel (no longer open-only).

## [0.3.174] — 2026-10-10

View:

- **Document Map** paints a soft blue wash over the current selection (title `· sel`), a white caret tick at the focused caret, and orange center ticks for multi-carets when column / multi-select is active (title `· N carets`). Click near a caret tick parks on that line (status `Document Map → caret line N` / `… · N carets`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park.

## [0.3.173] — 2026-10-10

View / Fold:

- **Document Map** paints slate fold-header ticks (brighter when that region is folded). Title appends fold count (`· N folds` / `· N folds (M folded)`). Click near a tick parks the caret and toggles that fold (status `Document Map → folded|unfolded K lines at N`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park.

## [0.3.172] — 2026-10-10

Search / View:

- **Document Map** paints magenta ticks on Find match lines when the Find query has hits. Title appends match count. Click near a tick selects the first match on that line (status `Document Map → find line N · K matches`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park.

## [0.3.171] — 2026-10-09

View:

- **Document Map** paints change-history ticks on the right (amber unsaved, green saved, same hues as the gutter bars). Title appends change count. Click near a tick parks the caret on that line (status `Document Map → change history line N (unsaved|saved)`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park. Bookmark ticks stay on the left.

## [0.3.170] — 2026-10-09

View:

- **Document Map** paints cyan ticks at bookmark lines (same hue as the gutter mark). Title appends bookmark count. Click near a tick parks the caret on that bookmark (status `Document Map → bookmark line N`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park.

## [0.3.169] — 2026-10-09

Compare / View:

- **Document Map** while Compare + Hide Unchanged Lines is on paints muted bands over collapsed Equal runs (`···N` gaps). Title shows gap count. Click a band to park on that gap (same status shape as Alt+F7 hidden-equal nav); other Compare clicks still park hunk / Equal partner.

## [0.3.168] — 2026-10-09

Compare / View:

- **Document Map** while Compare is on paints amber tick marks at each change-hunk start so clusters read at a glance. Drag (and non-Compare click) also syncs the partner pane when sync H/V scroll is on (Compare starts with sync V).

## [0.3.167] — 2026-10-09

Compare / View:

- **Document Map** click while Compare is on parks the caret on that line and syncs the other pane (hunk select or Equal partner park, same status as a gutter click). Drag still only scrolls.

## [0.3.166] — 2026-10-09

Compare / View:

- **Document Map** shows 2-way Compare colours while Compare is on: delete lines red, insert lines green, equal lines stay density gray. Map follows the focused dual-view pane (scroll mark + jump). Title appends `· compare` when tags apply.

## [0.3.165] — 2026-10-09

Plugins / local AI:

- **Ask Ollama** and **Ollama Status** under Plugins: talk to a user-run Ollama on loopback only (`127.0.0.1` / `localhost` / `::1`). Selection (or whole buffer) goes to `/api/generate`; reply opens in a tab. Status lists `/api/tags` models.
- Preferences / `settings.json`: `ollama_host` (default `http://127.0.0.1:11434`), `ollama_model` (default `llama3.2`). Remote hosts rejected. See `docs/ollama-helper.md` (issue #18).

## [0.3.164] — 2026-10-09

UI / menus:

- Tall menus (especially **View**) stay on screen: popup bodies use a vertical scroll area capped to the window height so egui does not flip them upward over the menu bar.
- Remappable shortcuts show on the right of matching menu items (weak shortcut text).

## [0.3.163] — 2026-10-09

Editor:

- Vertical **scrollbar** in the edit window is clickable and draggable: click the track to jump, drag the thumb to scroll (primary and dual-view secondary panes). Wider hit strip so the thin thumb is easier to grab.

## [0.3.162] — 2026-10-09

Settings / shortcuts:

- **Bookmark / Clear Compare Difference Bookmarks** is remappable: Preferences → Bookmark compare differences shortcut, or `shortcut_bookmark_compare_diffs` in `npp-rs/settings.json` (default `Alt+M`). Shift flips to Clear (`Alt+Shift+M`). Shortcut Mapper and About show the effective binding.

## [0.3.161] — 2026-10-09

Help / About:

- **About** is clean again: version, links, short why/made-by copy. Keyboard shortcuts moved to a separate **Keyboard shortcuts** window (About → Keyboard shortcuts…), with a vertical scroll area when the list is tall. Settings → Shortcut Mapper still opens the text dump.

## [0.3.160] — 2026-10-09

Settings / shortcuts:

- **Expand / Collapse Unchanged at Caret** (Compare) is remappable: Preferences → Expand unchanged at caret shortcut, or `shortcut_expand_unchanged_at_caret` in `npp-rs/settings.json` (default `Alt+X`). Shift flips to Collapse at Caret (`Alt+Shift+X`). Shortcut Mapper and About show the effective binding.

## [0.3.159] — 2026-10-09

Settings / shortcuts:

- **Expand / Collapse All Unchanged Lines** (Compare) is remappable: Preferences → Expand all unchanged shortcut, or `shortcut_expand_all_unchanged` in `npp-rs/settings.json` (default `Alt+E`). Shift flips to Collapse All (`Alt+Shift+E`). Shortcut Mapper and About show the effective binding.

## [0.3.158] — 2026-10-09

Settings / shortcuts:

- **Ignore blank lines** (Compare) is remappable: Preferences → Ignore blank lines shortcut, or `shortcut_compare_ignore_blank` in `npp-rs/settings.json` (default `Alt+B`). Shortcut Mapper and About show the effective binding.

## [0.3.157] — 2026-10-09

Settings / shortcuts:

- **Ignore whitespace / Ignore case** (Compare) is remappable: Preferences → Ignore whitespace shortcut, or `shortcut_compare_ignore_ws` in `npp-rs/settings.json` (default `Alt+W`). Shift flips to Ignore Case (`Alt+Shift+W`). Ignore Blank Lines stayed menu-only until v0.3.158. Shortcut Mapper and About show the effective binding.

## [0.3.156] — 2026-10-09

Settings / shortcuts:

- **Tab indent** is remappable: Preferences → Tab indent shortcut, or `shortcut_tab_indent` in `npp-rs/settings.json` (default `Tab`). Inserts spaces at the caret (and multi-carets); `Cmd+]` (`shortcut_indent`) stays as line-indent. Shortcut Mapper and About show the effective binding.

## [0.3.155] — 2026-10-09

Settings / shortcuts:

- **Copy / Open Compare Hunk** is remappable: Preferences → Copy Compare Hunk shortcut, or `shortcut_copy_compare_hunk` in `npp-rs/settings.json` (default `Alt+K`). Shift flips to Open Compare Hunk (`Alt+Shift+K`). Shortcut Mapper and About show the effective binding.

## [0.3.154] — 2026-10-09

Settings / shortcuts:

- **Copy / Open Compare Summary** is remappable: Preferences → Copy Compare Summary shortcut, or `shortcut_copy_compare_summary` in `npp-rs/settings.json` (default `Alt+Y`). Shift flips to Open Compare Summary (`Alt+Shift+Y`). Shortcut Mapper and About show the effective binding.

## [0.3.153] — 2026-10-09

Settings / shortcuts:

- **Copy / Open Compare Diff** is remappable: Preferences → Copy Compare Diff shortcut, or `shortcut_copy_compare_diff` in `npp-rs/settings.json` (default `Alt+C`). Shift flips to Open Compare Diff (`Alt+Shift+C`). Shortcut Mapper and About show the effective binding.

## [0.3.152] — 2026-10-09

Settings / shortcuts:

- **Toggle comment** is remappable: Preferences → Toggle comment shortcut, or `shortcut_toggle_comment` in `npp-rs/settings.json` (default `Cmd+/`). Shift flips to Block Comment (`Cmd+Shift+/`). Shortcut Mapper and About show the effective binding. Uses `/` so macOS Cmd+Q Quit stays free (N++ Ctrl+Q remaps via Preferences).

## [0.3.151] — 2026-10-09

Settings / shortcuts:

- **Move line** is remappable: Preferences → Move line shortcut, or `shortcut_move_line` in `npp-rs/settings.json` (default `Cmd+Shift+Up`). Opposite Down moves the line down; letter remaps use Shift for down. Shortcut Mapper and About show the effective binding. `Cmd+Up` / `Cmd+Down` still jump to document start / end.

## [0.3.150] — 2026-10-09

Settings / shortcuts:

- **Matching brace** is remappable: Preferences → Matching brace shortcut, or `shortcut_matching_brace` in `npp-rs/settings.json` (default `Cmd+B`). Shift flips to Select matching braces (`Cmd+Shift+B`). Shortcut Mapper and About show the effective binding.

## [0.3.149] — 2026-10-09

Settings / shortcuts:

- **Fold / Unfold current** is remappable: Preferences → Fold current shortcut, or `shortcut_fold_current` in `npp-rs/settings.json` (default `Alt+F`). Shift flips to Unfold Current (`Alt+Shift+F`). Shortcut Mapper and About show the effective binding.

## [0.3.148] — 2026-10-09

Settings / shortcuts:

- **Fold / Unfold all** is remappable: Preferences → Fold all shortcut, or `shortcut_fold_all` in `npp-rs/settings.json` (default `Alt+0`). Shift flips to Unfold All (`Alt+Shift+0`). Shortcut Mapper and About show the effective binding.

## [0.3.147] — 2026-10-09

Settings / shortcuts:

- **Shift+Tab outdent** is remappable: Preferences → Shift+Tab outdent shortcut, or `shortcut_shift_tab_outdent` in `npp-rs/settings.json` (default `Shift+Tab`). Outdents the current line(s); `Cmd+[` (`shortcut_outdent`) stays as the other outdent binding. Shortcut Mapper and About show the effective binding.

## [0.3.146] — 2026-10-09

Settings / shortcuts:

- **Print** is remappable: Preferences → Print shortcut, or `shortcut_print` in `npp-rs/settings.json` (default `Cmd+P`). Shortcut Mapper and About show the effective binding.

## [0.3.145] — 2026-10-09

Settings / shortcuts:

- **Save All** is remappable: Preferences → Save All shortcut, or `shortcut_save_all` in `npp-rs/settings.json` (default `Cmd+Alt+S`). Shortcut Mapper and About show the effective binding.

## [0.3.144] — 2026-10-09

Settings / shortcuts:

- **Next / previous tab** is remappable: Preferences → Next tab shortcut, or `shortcut_next_tab` in `npp-rs/settings.json` (default `Cmd+Tab`). Shift flips to previous tab. Shortcut Mapper and About show the effective binding.

## [0.3.143] — 2026-10-09

Settings / shortcuts:

- **Compare to Saved** is remappable: Preferences → Compare to Saved shortcut, or `shortcut_compare_to_saved` in `npp-rs/settings.json` (default `Alt+Shift+S`). Shortcut Mapper and About show the effective binding.

## [0.3.142] — 2026-10-09

Settings / shortcuts:

- **Word jump** is remappable: Preferences → Word jump shortcut, or `shortcut_word_jump` in `npp-rs/settings.json` (default `Alt+Left`). Opposite Left/Right jumps forward; Shift extends the selection. Shortcut Mapper and About show the effective binding.

## [0.3.141] — 2026-10-08

Settings / shortcuts:

- **Swap Compare Sides** is remappable: Preferences → Swap Compare sides shortcut, or `shortcut_swap_compare` in `npp-rs/settings.json` (default `Alt+S`). Shortcut Mapper and About show the effective binding.

## [0.3.140] — 2026-10-08

Settings / shortcuts:

- **Close Find/Replace** is remappable: Preferences → Close Find/Replace shortcut, or `shortcut_close_find` in `npp-rs/settings.json` (default `Escape`). Shortcut Mapper and About show the effective binding.

## [0.3.139] — 2026-10-08

macOS install / Gatekeeper:

- Release pack keeps the execute bit on Unix binaries; macOS artifact is ad-hoc codesigned in CI
- One-liner install: `scripts/install-macos.sh` (download, clear quarantine, install to Applications)
- Docs: `docs/macos-install.md` — Gatekeeper “damaged” is quarantine, not a corrupt file

## [0.3.138] — 2026-10-08

Settings / shortcuts:

- **Compare start / clear** is remappable: Preferences → Compare shortcut, or `shortcut_compare` in `npp-rs/settings.json` (default `Alt+D`). Shift + that chord is Clear Compare. Shortcut Mapper and About show the effective binding.

## [0.3.137] — 2026-10-08

Settings / shortcuts:

- **Hide unchanged lines** is remappable: Preferences → Hide unchanged lines shortcut, or `shortcut_hide_equal` in `npp-rs/settings.json` (default `Alt+H`). Shortcut Mapper and About show the effective binding.

## [0.3.136] — 2026-10-08

Settings / shortcuts:

- **First hidden equal** is remappable: Preferences → First hidden equal shortcut, or `shortcut_first_hidden_equal` in `npp-rs/settings.json` (default `Cmd+Alt+F7`). Shift + that chord is Last hidden equal. Shortcut Mapper and About show the effective binding.

## [0.3.135] — 2026-10-08

Settings / shortcuts:

- **First difference** is remappable: Preferences → First difference shortcut, or `shortcut_first_diff` in `npp-rs/settings.json` (default `Cmd+F7`). Shift + that chord is Last difference. Shortcut Mapper and About show the effective binding.

## [0.3.134] — 2026-10-08

Settings / shortcuts:

- **Reload from disk** is remappable: Preferences → Reload shortcut, or `shortcut_reload` in `npp-rs/settings.json` (default `Cmd+R`). Shortcut Mapper and About show the effective binding.

## [0.3.133] — 2026-10-08

Settings / shortcuts:

- **Hide-equal context** is remappable: Preferences → Hide-equal context shortcut, or `shortcut_hide_equal_context` in `npp-rs/settings.json` (default `Alt+]`). Opposite `[` / `]` decreases. Shortcut Mapper and About show the effective bindings.

## [0.3.132] — 2026-10-08

Settings / shortcuts:

- **Apply compare hunk** is remappable: Preferences → Apply compare hunk shortcut, or `shortcut_apply_compare_hunk` in `npp-rs/settings.json` (default `Cmd+Alt+Left`). Opposite Left/Right applies the hunk to the other pane; Shift applies all hunks in that direction. Chord parser accepts `Left` / `Right` / `ArrowLeft` / `ArrowRight`. Shortcut Mapper and About show the effective bindings.

## [0.3.131] — 2026-10-08

Settings / shortcuts:

- **Next hidden equal** is remappable: Preferences → Next hidden equal shortcut, or `shortcut_next_hidden_equal` in `npp-rs/settings.json` (default `Alt+F7`). Shift + that chord is Previous hidden equal. Cmd/Ctrl+Alt+F7 first/last stay hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.130] — 2026-10-08

Settings / shortcuts:

- **Replace (alternate)** is remappable: Preferences → Replace (alternate) shortcut, or `shortcut_replace_alt` in `npp-rs/settings.json` (default `Cmd+Shift+F`). Shortcut Mapper and About show the effective binding.

## [0.3.129] — 2026-10-08

Settings / shortcuts:

- **Find next (global)** is remappable: Preferences → Find next (global) shortcut, or `shortcut_find_next_global` in `npp-rs/settings.json` (default `Cmd+G`). Shift + that chord is Find previous. Shortcut Mapper and About show the effective binding.

## [0.3.128] — 2026-10-08

Settings / shortcuts:

- **Toggle log tail** is remappable: Preferences → Toggle log tail shortcut, or `shortcut_toggle_log_tail` in `npp-rs/settings.json` (default `Cmd+Shift+T`). Shortcut Mapper and About show the effective binding.

## [0.3.127] — 2026-10-08

Settings / shortcuts:

- **Zoom restore** is remappable: Preferences → Zoom restore shortcut, or `shortcut_zoom_restore` in `npp-rs/settings.json` (default `Cmd+0`). Mouse wheel zoom stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.126] — 2026-10-08

Settings / shortcuts:

- **Zoom out** is remappable: Preferences → Zoom out shortcut, or `shortcut_zoom_out` in `npp-rs/settings.json` (default `Cmd+-`). Restore (`Cmd+0`) stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.125] — 2026-10-08

Settings / shortcuts:

- **Zoom in** is remappable: Preferences → Zoom in shortcut, or `shortcut_zoom_in` in `npp-rs/settings.json` (default `Cmd+=`). Zoom out (`Cmd+-`) and restore (`Cmd+0`) stay hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.124] — 2026-10-08

Compare:

- Files longer than **3000 lines** still Compare: LCS uses the first 3000 lines per side (tail stays uncoloured). Live status appends `· first 3000 lines` instead of refusing the pair.

## [0.3.123] — 2026-10-08

Settings / shortcuts:

- **Redo** is remappable: Preferences → Redo shortcut, or `shortcut_redo` in `npp-rs/settings.json` (default `Cmd+Y`). Shift + Undo chord remains Redo. Shortcut Mapper and About show the effective binding.

## [0.3.122] — 2026-10-08

Settings / shortcuts:

- **Undo** is remappable: Preferences → Undo shortcut, or `shortcut_undo` in `npp-rs/settings.json` (default `Cmd+Z`). Shift + that chord is Redo. Cmd/Ctrl+Y stays hard-wired as an alternate. Shortcut Mapper and About show the effective binding.

## [0.3.121] — 2026-10-08

Settings / shortcuts:

- **Select all** is remappable: Preferences → Select all shortcut, or `shortcut_select_all` in `npp-rs/settings.json` (default `Cmd+A`). Shortcut Mapper and About show the effective binding.

## [0.3.120] — 2026-10-08

Settings / shortcuts:

- **Open** is remappable: Preferences → Open shortcut, or `shortcut_open` in `npp-rs/settings.json` (default `Cmd+O`). Shortcut Mapper and About show the effective binding.

## [0.3.119] — 2026-10-08

Settings / shortcuts:

- **New** is remappable: Preferences → New shortcut, or `shortcut_new` in `npp-rs/settings.json` (default `Cmd+N`). Shortcut Mapper and About show the effective binding.

## [0.3.118] — 2026-10-08

Settings / shortcuts:

- **Save As** is remappable: Preferences → Save As shortcut, or `shortcut_save_as` in `npp-rs/settings.json` (default `Cmd+Shift+S`). Shortcut Mapper and About show the effective binding.

## [0.3.117] — 2026-10-08

Settings / shortcuts:

- **Replace** is remappable: Preferences → Replace shortcut, or `shortcut_replace` in `npp-rs/settings.json` (default `Cmd+H`). Cmd+Shift+F stays hard-wired as an alternate. Shortcut Mapper and About show the effective binding.

## [0.3.116] — 2026-10-08

Settings / shortcuts:

- **Find** is remappable: Preferences → Find shortcut, or `shortcut_find` in `npp-rs/settings.json` (default `Cmd+F`). Replace stays hard-wired `Cmd+H` / `Cmd+Shift+F`. Shortcut Mapper and About show the effective binding.

## [0.3.115] — 2026-10-08

Settings / shortcuts:

- **Save** is remappable: Preferences → Save shortcut, or `shortcut_save` in `npp-rs/settings.json` (default `Cmd+S`). Save As stays hard-wired `Cmd+Shift+S`. Shortcut Mapper and About show the effective binding.

## [0.3.114] — 2026-10-07

Settings / shortcuts:

- **Close tab** is remappable: Preferences → Close tab shortcut, or `shortcut_close_tab` in `npp-rs/settings.json` (default `Cmd+W`). Shortcut Mapper and About show the effective binding.

## [0.3.113] — 2026-10-07

Settings / shortcuts:

- **Format document** is remappable: Preferences → Format document shortcut, or `shortcut_format_document` in `npp-rs/settings.json` (default `Cmd+Shift+I`). Shortcut Mapper and About show the effective binding.

## [0.3.112] — 2026-10-07

Settings / shortcuts:

- **Toggle bookmark** is remappable: Preferences → Toggle bookmark shortcut, or `shortcut_toggle_bookmark` in `npp-rs/settings.json` (default `Cmd+F2`). Shortcut Mapper and About show the effective binding.

## [0.3.111] — 2026-10-07

Settings / shortcuts:

- **Outdent** is remappable: Preferences → Outdent shortcut, or `shortcut_outdent` in `npp-rs/settings.json` (default `Cmd+[`). Shortcut Mapper and About show the effective binding.

## [0.3.110] — 2026-10-07

Settings / shortcuts:

- **Indent** is remappable: Preferences → Indent shortcut, or `shortcut_indent` in `npp-rs/settings.json` (default `Cmd+]`). Outdent stays hard-wired `Cmd+[`. Shortcut Mapper and About show the effective binding.

## [0.3.109] — 2026-10-07

Settings / shortcuts:

- **Delete line** is remappable: Preferences → Delete line shortcut, or `shortcut_delete_line` in `npp-rs/settings.json` (default `Cmd+Shift+L`). Shortcut Mapper and About show the effective binding.

## [0.3.108] — 2026-10-07

Settings / shortcuts:

- **Duplicate line** is remappable: Preferences → Duplicate line shortcut, or `shortcut_duplicate_line` in `npp-rs/settings.json` (default `Cmd+D`). Shortcut Mapper and About show the effective binding.

## [0.3.107] — 2026-10-07

Settings / shortcuts:

- **Go to line** is remappable: Preferences → Go to line shortcut, or `shortcut_goto_line` in `npp-rs/settings.json` (default `Cmd+L`). Cmd/Ctrl+Shift+L delete line stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.106] — 2026-10-07

Settings / shortcuts:

- **Next difference** is remappable: Preferences → Next difference shortcut, or `shortcut_next_diff` in `npp-rs/settings.json` (default `F7`). Shift + that chord is Previous difference. Cmd/Ctrl+F7 first/last and Alt+F7 hidden-equal stay hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.105] — 2026-10-07

Settings / shortcuts:

- **Next bookmark** is remappable: Preferences → Next bookmark shortcut, or `shortcut_next_bookmark` in `npp-rs/settings.json` (default `F2`). Shift + that chord is Previous bookmark. Cmd/Ctrl+F2 toggle stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.104] — 2026-10-07

Settings / shortcuts:

- **Find next** is remappable: Preferences → Find next shortcut, or `shortcut_find_next` in `npp-rs/settings.json` (default `F3`). Shift + that chord is Find previous. Cmd/Ctrl+G stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.103] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status appends the same ignore / hide-equal bits as the live pair line, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal · ignore ws+case · hide equal ±3 · 5 hidden` (identical pairs keep `(identical)` plus those bits when active).

## [0.3.102] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status appends Equal-line match percent, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal` (identical pairs stay `(identical)`).

## [0.3.101] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status includes pair −/+/kind tallies, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert)` (or `(identical)`).

## [0.3.100] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard / edit re-diff) auto-reveals a Hide-Unchanged collapsed Equal line on either pane so the parked caret stays visible. Edit re-diff on Equal appends `· equal L|R` to pair status (hunk landings still use `· at L|R`).

## [0.3.99] — 2026-10-07

Compare:

- After the ~200 ms edit re-diff, the other pane parks on the focused caret’s change hunk (or Equal partner); pair status appends `· at L|R (i/n kind −/+)`. Focused caret and selection stay put so typing is not yanked.

## [0.3.98] — 2026-10-07

Compare:

- Changing **Hide-equal context** (Alt+]/ / Alt+[ or Preferences slider) while Compare + Hide Unchanged Lines is on parks **and selects** both panes on the first change hunk; status is `Hide-equal context ±N — Compare … · at L|R (1/n kind −/+)`.

## [0.3.97] — 2026-10-07

Compare:

- **Hide Unchanged Lines** while Compare is on parks **and selects** both panes on the first change hunk; status appends `· at L|R (1/n kind −/+)` like ignore re-diff / Compare start / swap sides.

## [0.3.96] — 2026-10-07

Compare:

- **Next / Previous / First / Last Hidden Equal** hard-wired shortcuts: Alt+F7 / Alt+Shift+F7 walk ···N gaps; ⌘/Ctrl+Alt+F7 / ⌘/Ctrl+Alt+Shift+F7 jump to first/last. Plain F7 family still walks change hunks. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.95] — 2026-10-07

Compare:

- **Apply All Hunks From / To Other View** hard-wired shortcuts: ⌘/Ctrl+Alt+Shift+← pulls every remaining hunk from the other pane; ⌘/Ctrl+Alt+Shift+→ pushes every remaining hunk to the other pane. Single-hunk ⌘/Ctrl+Alt+←/→ unchanged. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.94] — 2026-10-07

Compare:

- **Apply Hunk From / To Other View** hard-wired shortcuts: ⌘/Ctrl+Alt+← pulls the other pane into the focused hunk; ⌘/Ctrl+Alt+→ pushes the focused hunk to the other pane. Alt alone still word-jumps. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.93] — 2026-10-06

Compare:

- **Next / Previous / First / Last Hidden Equal** status includes −/+ and hunk-kind tallies (`Compare Next hidden equal → ···5 (1/3) (−1 +2, 2 hunks: 1 delete, 1 insert) · wrapped`).

## [0.3.92] — 2026-10-06

Compare:

- **Expand Unchanged at Caret** / click ···N / **Collapse Unchanged at Caret** status includes −/+ and hunk-kind tallies (`Compare expanded ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 still hidden)` / `Compare collapsed ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 hidden)`).

## [0.3.91] — 2026-10-06

Compare:

- **Expand All Unchanged Lines** / **Collapse All Unchanged Lines** status includes −/+ and hunk-kind tallies (`Compare expanded all unchanged lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 shown)` / `Compare collapsed expanded equal lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 re-hidden, 8 hidden)`).

## [0.3.90] — 2026-10-06

Compare:

- **Bookmark Compare Differences** / **Clear Compare Difference Bookmarks** status includes −/+ and hunk-kind tallies (`Compare bookmarked differences (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, +3 new)` / `Compare cleared difference bookmarks (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, −3 removed)`).

## [0.3.89] — 2026-10-06

Compare:

- **Clear Compare** status keeps the last pair overview with −/+ and hunk-kind tallies (`Compare cleared (−1 +2, 2 hunks: 1 delete, 1 insert) “a” | “b”`, or `(identical)`; saved-snapshot clear keeps the same counts after the closed-snapshot prefix).

## [0.3.88] — 2026-10-06

Compare:

- **Swap Compare Sides** re-diffs and parks **and selects** both panes on the first change hunk; status appends `· at L|R (1/n kind −/+)` like Compare start / ignore re-diff.

## [0.3.87] — 2026-10-06

Compare:

- While Compare is on, toggling Ignore Whitespace / Case / Blank Lines re-diffs and parks **and selects** both panes on the first remaining change hunk; status appends `· at L|R (1/n kind −/+)` like Compare start.

## [0.3.86] — 2026-10-06

Compare:

- Starting Compare (and Compare to Saved) status appends the parked first hunk (`· at L12 | R15 (1/5 replace −1 +1)`), matching nav/click ordinal bits.

## [0.3.85] — 2026-10-06

Compare:

- **Copy Compare Diff** / **Open Compare Diff** status includes hunk-kind tallies (`Copied unified diff (−1 +2, 2 hunks: 1 delete, 1 insert) …`), matching Copy/Open Compare Summary.

## [0.3.84] — 2026-10-06

Compare:

- **Copy Compare Summary** / **Open Compare Summary** status includes hunk-kind tallies (`Copied compare summary (−1 +2, 2 hunks: 1 delete, 1 insert) …`), matching the live Compare status / summary header.

## [0.3.83] — 2026-10-06

Compare:

- **Copy Compare Hunk** / **Open Compare Hunk** status includes the hunk kind and −/+ counts (`Copied hunk (2/5 replace −1 +1) unified diff …`), matching nav/apply ordinal bits.

## [0.3.82] — 2026-10-06

Compare:

- **Apply All Hunks From/To Other View** status includes kind tallies and total −/+ for the hunks actually applied (`Applied 3 hunks (1 delete, 1 insert, 1 replace −2 +2) from other view · identical`).

## [0.3.81] — 2026-10-06

Compare:

- **Apply Hunk From/To Other View** status includes the applied hunk kind and −/+ counts, and the next remaining hunk uses the same ordinal bit (`Applied hunk (1/5 replace −2 +1) from other view → L12 | R15 (2/4 delete −1 +0)`).

## [0.3.80] — 2026-10-06

Compare:

- Next/Previous/First/Last Difference and click/keyboard hunk sync status include the current hunk −/+ line counts (`(2/5 replace −1 +1)`), so hunk size is visible without opening Compare Summary.

## [0.3.79] — 2026-10-06

Compare:

- Next/Previous/First/Last Difference and click/keyboard hunk sync status include the current hunk kind (`(2/5 replace)`), so delete/insert/replace is visible without opening Compare Summary.

## [0.3.78] — 2026-10-06

Compare:

- Live Compare status line tallies hunk kinds after the hunk count (`· K hunks: N delete, M insert, P replace`) so delete/insert/replace mix is visible without opening Compare Summary.

## [0.3.77] — 2026-10-06

Compare:

- Live Compare status line appends Equal-line match percent (`· N% equal`) for non-identical pairs so similarity is visible without opening Compare Summary.

## [0.3.76] — 2026-10-06

Compare:

- **Open / Copy Compare Summary** header includes an Equal-line match percent (`· N% equal`) so similarity is visible without scanning every hunk.

## [0.3.75] — 2026-10-06

Compare:

- **Open / Copy Compare Summary** shows up to 3 preview lines per side under each hunk (`-` / `+`, max 72 chars), then `… (+N more)` when the side is longer, so multi-line hunks stay scannable.

## [0.3.74] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** header appends active ignore flags (`· ignore ws+case+blank`) so a copied or opened summary records which Ignore toggles shaped the hunk list.

## [0.3.73] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** header tallies hunk kinds (`N delete, M insert, K replace`) next to the hunk count.

## [0.3.72] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** includes a truncated first-line preview under each hunk (`-` / `+`, max 72 chars) so the index is scannable without opening every hunk.

## [0.3.71] — 2026-10-05

Compare:

- **View → Copy Compare Summary** copies the hunk index text to the clipboard (same body as Open Compare Summary; Compare stays on).

## [0.3.70] — 2026-10-05

Compare:

- **View → Open Compare Summary** opens a `compare-summary.txt` tab listing every change hunk with L|R line ranges and −/+ counts (clears Compare so the tab stays visible).

## [0.3.69] — 2026-10-05

Compare / Preferences:

- **Hide-equal context** (0–10, default 3) sets how many Equal lines stay visible on each side of a change when Hide Unchanged Lines is on. Status shows `· hide equal ±N`. Key: `compare_hide_equal_context`.

## [0.3.68] — 2026-10-05

Compare:

- **View → First Hidden Equal** / **Last Hidden Equal** jump to the first/last ···N collapsed Equal gap while Hide Unchanged Lines is on (parks both panes; status shows ordinal).

## [0.3.67] — 2026-10-05

Compare:

- **View → Next Hidden Equal** / **Previous Hidden Equal** jump between ···N collapsed Equal gaps while Hide Unchanged Lines is on (wraps with `· wrapped` status; parks both panes on the aligned Equal partner).

## [0.3.66] — 2026-10-05

Folding / session:

- Opt-in session restore now keeps **folded regions** (writes `@folds` header lines under each path in `npp-rs/session.txt`; applies on restore when headers still match).

## [0.3.65] — 2026-10-05

Compare:

- **View → Collapse Unchanged at Caret** re-collapses the expanded Equal run nearest the caret on both panes (···N cue returns for that run; hide-equal stays on).

## [0.3.64] — 2026-10-05

Compare:

- **View → Expand Unchanged at Caret** expands the collapsed Equal run nearest the caret on both panes (same as clicking that `···N` cue; hide-equal stays on).

## [0.3.63] — 2026-10-05

Compare:

- Click or move the caret onto an Equal (unchanged) line parks the other pane on the LCS-aligned partner line (status: `Compare equal → Lx | Ry`). Change-line hunk select is unchanged.

## [0.3.62] — 2026-10-05

Compare:

- **View → Clear Compare Difference Bookmarks** removes bookmarks at each change-hunk start on both panes (other bookmarks stay).

## [0.3.61] — 2026-10-05

Compare:

- **View → Bookmark Compare Differences** bookmarks the start of every change hunk on both panes so F2 / Shift+F2 can walk differences after you leave a hunk.

## [0.3.60] — 2026-10-05

Compare:

- **View → Collapse All Unchanged Lines** re-collapses every click/menu-expanded Equal run on both panes while Hide Unchanged Lines stays on (`···N` cues return without flipping the preference).

## [0.3.59] — 2026-10-05

Compare:

- **View → Expand All Unchanged Lines** reveals every collapsed Equal run on both panes while Hide Unchanged Lines stays on (clears remaining `···N` cues without flipping the preference).

## [0.3.58] — 2026-10-05

Compare:

- **Hide Unchanged Lines**: clicking a `···N` cue expands the collapsed Equal run on **both panes** (LCS-aligned partner lines). Status notes `both panes` when the other side opened too.

## [0.3.57] — 2026-10-05

Compare:

- **Hide Unchanged Lines**: click a gutter `···N` cue to expand that collapsed Equal run on the clicked pane. Status reports how many lines opened and how many remain hidden. Clear Compare / toggle hide-equal off / swap sides resets expansions.

## [0.3.56] — 2026-10-05

Compare:

- **Hide Unchanged Lines** paints gutter `···N` gap cues (and a hairline) between collapsed Equal runs, and status shows how many lines are hidden (`· N hidden` or `· Lx|Ry hidden`).

## [0.3.55] — 2026-10-05

Compare:

- **Hide Unchanged Lines** keeps **±3 Equal lines of context** around each change hunk so collapsed Compare panes stay readable. Status shows `· hide equal ±3`.

## [0.3.54] — 2026-10-05

Compare:

- **View → Hide Unchanged Lines** (Preferences persist) collapses Equal-tagged lines in both Compare panes so only change lines stay visible. Identical pairs stay fully shown. Status appends `· hide equal`.

## [0.3.53] — 2026-10-05

Compare:

- Replace-hunk intra-line wash is **word-aware**: LCS on word/separator tokens, with char refine on 1:1 token replaces so typos still highlight inside a word. Whole-word edits (e.g. `cat` → `dog`) wash the token, not scattered characters.

## [0.3.52] — 2026-10-05

Compare:

- **View → Clear Compare** (and other clear paths) closes the read-only `name (saved)` snapshot tab when it was part of the compare pair, so Compare to Saved does not leave an orphan tab.

## [0.3.51] — 2026-10-05

Compare:

- **View → Compare to Saved** diffs the active named tab against a read-only snapshot of its on-disk contents (reuses the `name (saved)` tab when present). Status uses the usual Compare −/+ / identical wording.

## [0.3.50] — 2026-10-05

Compare:

- **View → Apply All Hunks To Other View** pushes every remaining change hunk onto the other pane in one undo (last hunk first). Status shows how many hunks were applied, plus `· identical` when the pair matches.

## [0.3.49] — 2026-10-05

Compare:

- **View → Apply Hunk To Other View** pushes the change hunk at the focused caret onto the other pane (one undo). Parks and selects the next remaining hunk on both panes, same as Apply From Other. Status shows the next `L|R` ordinal, or `· identical` when the pair matches.

## [0.3.48] — 2026-10-05

Compare:

- **View → Open Compare Hunk** opens a `compare-hunk.diff` tab with the unified diff for the change hunk at the caret (same payload as Copy Compare Hunk). Compare mode turns off so the tab stays visible. Status shows hunk ordinal and −/+ counts.

## [0.3.47] — 2026-10-04

Compare:

- **View → Ignore Blank Lines** (Preferences `compare_ignore_blank`) skips blank / whitespace-only lines in the LCS so extra empty lines do not create hunks. Combines with ignore-whitespace / ignore-case; status shows `· ignore blank` (or `ws+case+blank`).

## [0.3.46] — 2026-10-04

Compare:

- **Apply Hunk From Other View** parks and selects the next remaining hunk on both panes after a successful apply. Status shows the next `L|R` ordinal, or `· identical` when the pair matches. **Apply All** also appends `· identical` when nothing is left.

## [0.3.45] — 2026-10-04

Search:

- Regex **Replace** / **Replace All** expand `$n`, `${n}`, `$0` / `$&` (whole match), `$$`, and `\1` (plus `\n` `\t` `\r`). Literal replace is unchanged when **Re** is off.

## [0.3.44] — 2026-10-04

Compare:

- **View → Open Compare Diff** opens a `compare.diff` tab with the same unified diff as Copy Compare Diff. Compare mode turns off so the tab stays visible. Status shows change counts or `(identical)`.

## [0.3.43] — 2026-10-04

Search:

- **Find in Files** honors the Find bar **Re** toggle (same linear-time regex as in-file Find). Invalid patterns report `Find in Files: invalid regex` and do not open a results tab. Literal search is unchanged when **Re** is off.

## [0.3.42] — 2026-10-04

Compare:

- **View → Apply All Hunks From Other View** copies every remaining change hunk onto the focused pane in one undo. Insert-only and delete-only hunks work. Status shows how many hunks were applied.

## [0.3.41] — 2026-10-04

Compare:

- Replace hunks highlight the **changed characters** on both panes (stronger wash on the intra-line LCS). Pure insert/delete lines stay whole-line colour. Lines over 256 characters keep the line wash only.

## [0.3.40] — 2026-10-04

Compare:

- **View → Apply Hunk From Other View** replaces the focused pane's change hunk with the other pane (one undo). Insert-only and delete-only hunks work. Status shows the hunk ordinal.

## [0.3.39] — 2026-10-04

Compare:

- **View → Copy Compare Hunk** copies the change hunk at the focused caret (or the next hunk) as a unified diff, with 3 equal context lines. Status shows hunk ordinal and −/+ counts.

## [0.3.38] — 2026-10-04

Compare:

- **View → Copy Compare Diff** copies a unified diff of the compared pair to the clipboard (3 lines of context). Status shows change counts or `(identical)`.

## [0.3.37] — 2026-10-04

Preferences:

- Unknown keys in `npp-rs/settings.json` survive the next save (hand-edited or future fields are not dropped).

## [0.3.36] — 2026-10-04

Compare:

- Next/Prev/First/Last Difference, click/keyboard hunk sync, and Start Compare select the change hunk on **both** panes (Copy/Delete still use the focused pane).

## [0.3.35] — 2026-10-04

Preferences / View:

- Show white space and TAB, Show EOL, Show NPC, and Indent guide persist in `npp-rs/settings.json` (View menu + Preferences) and restore on launch.

## [0.3.34] — 2026-10-04

Preferences:

- Recent file list load now honors Preferences **Recent file count** (`recent_max`, 5–40) instead of hard-capping at 15 on restart.

## [0.3.33] — 2026-10-04

Compare:

- Next/Prev/First/Last Difference selects the whole change hunk on the focused pane (Copy/Delete apply to the change).

## [0.3.32] — 2026-10-04

Compare:

- Next/Prev Difference status appends `· wrapped` when the jump circles past the end or beginning of the file.

## [0.3.31] — 2026-10-04

Compare:

- Keyboard caret motion onto a red/green change line parks the other pane on the same hunk (same as click).

## [0.3.30] — 2026-10-04

Compare:

- Click a red/green change line to park the other pane on the same hunk ordinal.
- Next/Prev/First/Last Difference status shows both sides (`L12 | R15 (2/5)`).

## [0.3.29] — 2026-10-04

Compare:

- **View → Ignore Whitespace Differences** / **Ignore Case Differences** toggle (✓ in menu), persist to settings, and re-diff immediately while Compare is on.
- Starting Compare parks **both** panes on the first hunk ordinal (not only the left caret).
- Status line shows active ignore flags (`· ignore ws`, `· ignore case`, or `· ignore ws+case`).

## [0.3.28] — 2026-10-04

Compare:

- **View → Swap Compare Sides** flips the left/right pair (scroll included), keeps focus on the new left, and re-diffs so delete/insert colours stay oriented.

## [0.3.27] — 2026-10-04

Compare:

- Closing a tab that is not part of the compare pair remaps both sides so Compare stays on; closing either compared tab still clears Compare.

## [0.3.26] — 2026-10-04

Compare:

- **View → First Difference** / **Last Difference** (⌘/Ctrl+F7 / ⌘/Ctrl+Shift+F7) jump to the first or last change hunk; other pane stays on the same ordinal.

## [0.3.25] — 2026-10-04

Compare:

- Next/Previous Difference also parks the other pane on the same hunk ordinal.
- Status says `(identical)` when both sides match (including ignore options); live re-diff keeps hunk count.

## [0.3.24] — 2026-10-04

Compare:

- Preferences: **Ignore case differences** (`compare_ignore_case` in settings.json). Combines with ignore-whitespace; live re-diff while Compare is on.

## [0.3.23] — 2026-10-04

Compare:

- Starting Compare jumps the left caret to the first change hunk and shows hunk count in the status line.
- Next/Previous Difference status shows hunk ordinal, e.g. `(2/5)`.

## [0.3.22] — 2026-10-04

Project panel:

- **Reveal** opens the workspace folder (or a right-clicked entry) in the OS file manager.
- **Refresh** reloads a cached folder listing instead of re-reading every frame.
- Context menu: Open / Enter folder / Reveal in file manager.

## [0.3.21] — 2026-10-04

Folding:

- Fold margin click uses the same region pick as View → Fold/Unfold Current: toggle works on header markers and on any line inside a foldable block (primary and dual view).

## [0.3.20] — 2026-10-03

Compare:

- **Next / Previous Difference** (View menu, **F7** / **Shift+F7**) jumps the focused pane to the start of the next/previous change hunk while Compare is on (wraps). Mid-hunk Next skips to the following hunk.

## [0.3.19] — 2026-10-03

Settings / shortcuts:

- **Word wrap** is remappable: Preferences → Word wrap shortcut, or `shortcut_word_wrap` in `npp-rs/settings.json` (default `Alt+Z`). Shortcut Mapper and About show the effective binding. Other keys stay hard-wired; full `shortcuts.xml` remap is still later.

## [0.3.18] — 2026-10-03

Undo:

- Multi-caret / column typing coalesce: successive inserts at the same carets merge into one undo unit (same 1s window as plain typing). Replace-selection and deletes still start a new unit.

## [0.3.17] — 2026-10-03

Encoding:

- Open detects **UTF-16 LE/BE without a BOM** when at least two-thirds of 16-bit units have a zero high byte (typical ASCII / Latin). Status notes the missing BOM; the next save writes one.
- Valid UTF-8 without that NUL pattern is unchanged. Invalid UTF-8 still falls back to Windows-1252.

## [0.3.16] — 2026-10-03

Find / replace:

- Opening Find or Replace with a **multi-line selection** turns **Sel** on and searches that range (query stays as-is). A short single-line selection still fills the query.
- Match count is 0 while Sel is on but no range is captured (no silent whole-file search).
- Wrap-off Next/Prev says `passed end of selection` when Sel is on.

## [0.3.15] — 2026-10-03

Find / replace:

- **Re** on the Find bar (and Preferences **Regular expression**) treats the query as a regex. Engine is linear-time, so nested quantifiers do not hang the UI. Invalid patterns report `Find: invalid regex`. Replace uses a literal replacement string.
- Setting `find_regex` in `npp-rs/settings.json`

## [0.3.14] — 2026-10-03

Find / replace:

- **Wrap** on the Find bar (and Preferences) turns wrap-around on or off. Off: Next/Prev stop at the bound (`Find: passed end of file`). Default on, same as before.
- Setting `find_wrap` in `npp-rs/settings.json`

## [0.3.13] — 2026-10-03

Find / replace:

- **In selection** (`Sel` on the Find bar, Preferences Find) limits Next / Prev / Replace All to a captured range
- Setting `find_in_selection` in `npp-rs/settings.json`

## [0.3.12] — 2026-08-31

Issue #14 (P1 lexer-aware folding + fold margin):

- Gutter fold markers (`−` / `+`); click toggles a region
- Brace folds for Rust/C-like languages; indent folds for Python and others
- View → Fold / Unfold / levels use the same regions
- Preferences: **Show fold margin** (`show_fold_margin` in `npp-rs/settings.json`)
- Docs: `docs/folding.md`

## [0.3.11] — 2026-08-31

Issue #13 (P1 autosave / backup-on-save):

- Preferences → Files: **Backup on save** copies the on-disk file into `npp-rs/backup/` (path layout) before overwrite
- Preferences → Files: **Autosave interval** (0 = off, else 15–900s) saves dirty tabs that already have a path
- Settings keys: `backup_on_save`, `autosave_interval_secs` in `npp-rs/settings.json`
- Docs: `docs/autosave-backup.md`

## [0.3.10] — 2026-08-31

Issue #12 (P1 deeper Find in Files):

- Search uses the Project / workspace root and walks folders recursively
- Skips hidden dirs, symlinks, binary / huge files; caps matches and depth
- Include / exclude globs on the Find bar (persisted); default exclude skips `target`, `node_modules`, and similar
- Menu **Search → Find in Files** and the Find bar **Find in Files** button

## [0.3.9] — 2026-08-31

Issue #11 (P0 column / rect select + multi-caret typing):

- Alt+drag (Option+drag on macOS) builds a rectangular / column selection
- Typing, Backspace, Delete, Paste, Enter, and Tab apply to all multi-carets (one undo)
- Copy/Cut of multi ranges joins line slices with newlines
- Docs: `docs/column-mode.md` (no virtual space; arrows clear multi-carets)

## [0.3.8] — 2026-08-31

Issue #10 (P0 file drop + selection drag):

- Drop files onto the window to open them (folders skipped)
- Drag selected text to move; Ctrl/Cmd+drag to copy (one undo)
- Orange drop caret while dragging; dual-view panes supported

## [0.3.7] — 2026-08-31

Issue #9 (P0 hotkeys / Find next):

- F3 / Shift+F3 find next/prev without the Find bar open
- Cmd/Ctrl+G / Shift+G find next/prev are global
- Cmd/Ctrl+L go to line; F2 / Shift+F2 / Cmd+F2 bookmarks
- Cmd/Ctrl+= − 0 and Cmd/Ctrl+mouse wheel zoom; Alt+Z word wrap
- Cmd/Ctrl+H replace; Shortcut Mapper and About lists updated

## [0.3.6] — 2026-08-31

Issue #8 (P1 encoding):

- Open UTF-16 LE/BE with BOM; decode to Unicode in the buffer
- Save as UTF-16 LE/BE when Format (or Convert to) sets that encoding
- Status / docs: `docs/encoding.md`

## [0.3.5] — 2026-08-31

Issue #8 (P1 change-history depth):

- Gutter: full-height SC_MARK_BAR-style blocks (amber unsaved / green saved) with joined runs
- Soft line wash + dual-view pane marks
- Undo/redo remaps line marks via buffer line-structure hooks (no caret re-stamp)
- Status `CHG u/s`, Clear/jump messages name unsaved vs saved

## [0.3.4] — 2026-08-30

Issue #8 (P1 themes depth):

- Theme JSON: selection, caret, whitespace, indent guide, and syntax `tokens` map
- Notepad++ XML subset: GlobalStyles chrome + preferred lexer WordsStyle → highlight tokens
- Samples: deeper `themes/slate.json`, `themes/mini-dark.xml`
- Primary and secondary panes use theme chrome colours


## [0.3.3] — 2026-08-30

- Compare: default partner is the tab to the right (or left if last); ⌘/Ctrl-click or tab menu to pick any second tab
- Help / About / cmdline links use `main` (not `dev`)
- README feature-tour GIF (`docs/screens/`) + rebuild scripts

## [0.3.2] — 2026-08-29

Issue #8 (partial P1):

- Project panel: name filter, Refresh, persist workspace root in settings.json
- ANSI (Windows-1252) save: confirm dialog when characters would become `?`

## [0.3.1] — 2026-08-29

Issue #7 P0 gap batch:

- Preferences: gutter extra, caret blink, default EOL, recent count, restore session, find options, compare ignore-whitespace
- Find/Replace: Match case / Whole word, live match count, persisted query; Replace All is one undo + marks compare stale
- Session: config-dir `npp-rs/session.txt`; opt-in restore on launch; save on quit
- Compare: sync H+V scroll on start; optional ignore whitespace

## [0.3.0] — 2026-08-29

Stability and architecture batch for issues #3 / #4 / #6:

- Atomic saves; DocumentId async loads; tail worker; bookmark remap
- Read-only edit gate; reload + saved revision; transactional undo
- Honest UTF-8/Windows-1252 open; CI fmt/clippy; highlight viewport
- Theme apply MVP; project file panel; RTL layout cue


## [0.2.14] — 2026-08-29

### Stability (issue #4)

- File → Reload from Disk replaces buffer contents (dirty confirm)
- Dirty follows `saved_generation` vs `edit_generation` (undo-to-saved clears dirty)


## [0.2.13] — 2026-08-29

### Stability (issue #4)

- File → Reload from Disk replaces buffer contents (dirty confirm)
- Dirty follows saved vs edit generation (undo-to-saved clears dirty)

### Themes / project / RTL

- Theme MVP: built-in Dark/Light + `themes/*.json`; Preferences and Import Style Theme(s) apply egui visuals and editor bg/fg/gutter
- Project panel lists the workspace folder and opens files on click; Open Folder as Workspace sets the root
- RTL/LTR mirrors editor line anchors and shows an RTL status cue

## [0.2.12] — 2026-08-29

### Read-only

- Menu mutate paths refuse edits when the document is read-only or still loading (`Document::try_buffer_mut`, command `ensure_editable`)
- Toggle / clear read-only commands still work


## [0.2.11] — 2026-08-29

### Build / CI

- Fix TailMsg test initializers, `EditorState::workspace_root`, and edit `ensure_editable` gate
- Restore clippy allows on buffer `from_str` / `to_string`
- Mark issue #4 Agent E checklist done

## [0.2.10] — 2026-08-29

### Encoding honesty

- Open/tail never insert U+FFFD via `from_utf8_lossy`; invalid UTF-8 uses Windows-1252 with clear status
- Tests cover invalid byte sequences (`docs/encoding.md`)

## [0.2.9] — 2026-08-29

### Undo

- One user command is one undo unit (`with_transaction` for multi-edit helpers)
- Typing coalesce: same kind (insert), adjacent caret, within 1s
- Notes: `docs/undo-transactions.md`

## [0.2.8] — 2026-08-29

### Highlight

- Byte→char conversion is one forward pass (no per-span full rescans)
- Refresh uses a viewport-oriented window (still capped at 512 KiB)
- Notes: `docs/highlight-viewport.md`

## [0.2.7] — 2026-08-29

### Bookmarks

- Bookmarks (and other line marks) shift when inserts or deletes change line structure
- Buffer records `LineStructureEdit`; the editor prefers that over the snap heuristic

## [0.2.6] — 2026-08-29

### Stability (issue #4)

- Log tail: disk reads and rotate reload run on a background worker; the UI only applies `TailMsg` (dirty/suspend policy unchanged)

### Tabs / open

- Async large-file load binds to a stable `DocumentId` (not tab index)
- Pending load apply drops when the id is gone or no longer the loading placeholder

## [0.2.5] — 2026-08-29

### Save

- Atomic save: write a sibling temp file, `sync_all`, then rename over the target (std rename replaces on Windows)
- Save no longer creates missing parent directories

## [0.2.4] — 2026-08-29

### CLI

- `-V` / `--version`, `-n` / `--line`, `-ro` / `--read-only`, `--` end of options
- Help → Command Line Arguments documents the same flags

### Preferences follow-up

- View → Word wrap writes `settings.json`
- Edit → Indent Tab uses Preferences tab width

## [0.2.3] — 2026-08-29

### Dual view

- Menu Edit (and Format) commands use the focused pane tab, not only the tab-bar active document

### Encoding

- Format → ANSI: save writes Windows-1252 (lossy); UTF-8 / UTF-8-BOM set per-tab save encoding
- Open files keep the detected encoding for later save (`docs/encoding.md`)

### Compare

- Re-diff line tags after edits (~200 ms debounce) while Compare is on

### Change history

- Amber gutter ticks for unsaved edits; green after save (promote, not clear)
- Line-index remap on insert/delete (`LineEditSnap` / `prepare_edit`)

## [0.2.2] — 2026-08-29

### Release checkpoint

- About tagline: “a Notepad++ inspired editor, rebuilt for fun”
- Overnight gap loop started on issue #6 (`docs/overnight-gaps.md`)

## [0.2.1] — 2026-08-29

### Compare fix

- Move to Other View no longer undoes itself (removed extra Switch)
- Compare pins left/right panes to the compared pair so colours stay visible
- Switch in compare mode swaps both sides and their tags
- Clearer compare how-to in `docs/compare.md`

## [0.2.0] — 2026-08-29

Large feature batch after 0.1.2 (menu parity, dual view, compare, UX).

### Dual view edit

- Other view pane is writable (type, delete, clipboard, caret, selection)
- Click a pane to focus keyboard input; undo shortcuts follow the focused pane
- Sync scroll / zoom sync / compare washes unchanged

### Built-in compare

- View → Compare with Other View: 2-way side-by-side colours (red delete / green insert) + sync scroll
- In-process line LCS (`diff.rs`); Clear Compare to exit; max 3000 lines/side

### Menu parity (issue #1)

- File: `IDM_PINTAB` toggles active-tab pin; Close All but Pinned reports keep/closed counts
- Fold / hide lines: hidden lines leave the viewport
- Style-mark washes and bookmark ticks in the gutter
- Search: style mark, jump, clear, and copy-styled helpers
- Edit Cut / Copy / Paste use the session clipboard
- Placeholder / status-only menus cleared; Coming Soon stubs gone (`docs/menu-todo.md`)

### Editor UX

- Drag tabs on the tab bar to reorder (live slide; dual/compare indices remap)
- Prompt before closing unsaved tabs (Save / Don't Save / Cancel), including Close All variants, Exit, and window close
- Keep the last editor line clear of the status bar (status panel before editor + bottom padding)
- Help (?) → Changelog opens `docs/changelog.md` on GitHub
- Status bar shows `v{version}` and git short hash
- Preferences: log-tail, font size, line numbers
- CLI: open path args + `-h` / `--help`

### Earlier on `dev` (after 0.1.2)

- Command split by domain for parallel agents (`docs/agent-parallel.md`)
- Log open dialog, Help → Debug Info / Open Logs, dirty-safe tail stability

## [0.1.2] — 2026-08-28

### Menu parity (issue #1)

- Encoding menu: all `IDM_FORMAT_*` acknowledged (UTF-8 in memory)
- Edit: sort lines, remove dups, blank below, sentence case, split, Mac EOL

Ready ~337 / stub ~237 — see `docs/menu-todo.md`.

## [0.1.1] — 2026-08-28

### Progress on menu parity ([#1](https://github.com/raro42/npp-rust/issues/1))

- Inventory: `docs/menu-todo.md`
- File: Save All / Copy As / Rename / close variants / open folder & viewer / shell
- Edit: case, join/move lines, datetime, path copy helpers
- Search: set-and-find, Go to Line
- View: zoom, always-on-top, tab switch

## [0.1.0] — 2026-08-28

First public release of **npp-rs**.

### Features

- Multi-tab editor (UTF-8), open / save / recent files
- Find and replace
- Undo / redo, indent, line ops, word select
- Tree-sitter highlight (Rust, C/C++, Python, SQL, Markdown, JSON, …)
- Full Notepad++-style menu tree; ready items tinted teal; stubs show Coming Soon
- In-process plugins and format helpers
- Agent loop + public-repo privacy gates (`agents/`, `scripts/gh-safe.sh`)

### Builds

GitHub Actions release binaries for Linux, Windows, and macOS.
