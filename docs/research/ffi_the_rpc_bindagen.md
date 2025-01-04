# FFI RPC Bindings

Lets see if we can't generate ourselves out of this problem!

Run this in the root directory to generate `algo_fetch`

```bash
npx @openapitools/openapi-generator-cli generate \
  -i https://raw.githubusercontent.com/algorand/go-algorand/v3.26.0-stable/daemon/algod/api/algod.oas3.yml \
  -g rust \
  -o ./crates/algo_fetch \
  --skip-validate-spec \
  --additional-properties=packageName=algo_fetch,supportAsync=true
```

Thoughts:

- Uses Serde/compatible with FFI approach
- Conforms to the specification/shared type system
- Allows all options (one off generated RPC clients ala-carte)
- Unix philosophy(also package principles)

Strong Thoughts(Regardless of approach):

- We should not own the RPC contract (the service should)
- RPC should not have any abstractions (no more statusAfterBlock vs waitForBlock)

Luke Warm Thoughts:

We should be able to design the interfaces in a way where all boundaries are respected. 
The POST endpoints for RPC accept Bytes. This requires no integration nor special knowledge.
