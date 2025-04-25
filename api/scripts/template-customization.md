# OpenAPI Generator Mustache Template Customization

This document explains how to customize OpenAPI Generator Mustache templates to handle specific endpoints differently, as implemented in this project.

## Special Endpoint Handling

The OpenAPI Generator uses Mustache templates to generate code. These templates can be customized to handle specific endpoints differently. This project implements a pattern for special endpoint handling using vendor extensions.

### Adding Vendor Extensions to OpenAPI Spec

To identify specific endpoints for special handling, we add vendor extensions to the OpenAPI specification. The `sync-vendored-extensions.ts` script demonstrates this approach:

```typescript
// Handle /v2/transactions POST endpoint
if (spec.paths['/v2/transactions'] && spec.paths['/v2/transactions']['post']) {
  const endpoint = spec.paths['/v2/transactions']['post'];
  // Use a single extension to identify the endpoint type
  endpoint['x-algorand-endpoint-type'] = 'raw-transaction';
}
```

This adds a single, clear vendor extension:

- `x-algorand-endpoint-type`: Identifies the type of endpoint (e.g., 'raw-transaction')

### Using Vendor Extensions in Mustache Templates

In the `api_test.mustache` template, we use this vendor extension to apply special handling:

```mustache
{{#vendorExtensions.x-algorand-endpoint-type}}
  {{#equals this 'raw-transaction'}}
  # Special handling for raw-transaction endpoint
  rawtxn = TestDataGenerator.random_signed_txn(api_instance, headers)
  response = api_instance.{{operationId}}(rawtxn=rawtxn, _headers=headers)
  {{/equals}}
  {{^equals this 'raw-transaction'}}
  # Standard parameter handling for other special endpoints
  ...
  {{/equals}}
{{/vendorExtensions.x-algorand-endpoint-type}}
{{^vendorExtensions.x-algorand-endpoint-type}}
# Standard endpoint handling for non-special endpoints
...
{{/vendorExtensions.x-algorand-endpoint-type}}
```

## Best Practices for Template Customization

1. **Use Clear Vendor Extensions**: Use a single, descriptive vendor extension to identify endpoint types
2. **Keep Templates Clean**: Use conditional blocks in templates to handle special cases
3. **Document Extensions**: Document all vendor extensions and their purpose
4. **Automate Extension Addition**: Use scripts to add extensions consistently
5. **Test Generated Code**: Always test the generated code to ensure it works as expected

## Adding New Special Cases

To add a new special case:

1. Add a new vendor extension in the `sync-vendored-extensions.ts` script
2. Update the Mustache template to handle the new special case
3. Document the new extension in this README

For example, to add special handling for a `/v2/accounts` endpoint:

```typescript
// Handle /v2/accounts GET endpoint
if (spec.paths['/v2/accounts'] && spec.paths['/v2/accounts']['get']) {
  const endpoint = spec.paths['/v2/accounts']['get'];
  endpoint['x-algorand-endpoint-type'] = 'account';
}
```

Then update the template:

```mustache
{{#vendorExtensions.x-algorand-endpoint-type}}
  {{#equals this 'account'}}
  # Special handling for Account endpoint
  ...
  {{/equals}}
{{/vendorExtensions.x-algorand-endpoint-type}}
```

## Running the Script

To run the script and add extensions to your OpenAPI specification:

```bash
bun run scripts/sync-vendored-extensions.ts specs/algod.oas3.json specs/algod.oas3_extended.json
```

## References

- [OpenAPI Generator Templating](https://openapi-generator.tech/docs/templating/)
- [Mustache Manual](https://mustache.github.io/mustache.5.html)
