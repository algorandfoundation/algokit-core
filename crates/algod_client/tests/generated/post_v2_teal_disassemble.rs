// Polytest Suite: POST v2_teal_disassemble
// Polytest Group: Common Tests
//
// Deferred: the generated client types the request as a `String` and sends it as JSON-encoded text
// under a `Content-Type: application/msgpack` header, but the endpoint expects raw program bytes.
// Filling this test needs a fix to the generated request handling.
#[tokio::test]
#[ignore = "deferred: disassemble request handling sends a JSON string, not program bytes"]
async fn basic_request_and_response_validation() {}
