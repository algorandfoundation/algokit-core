// Polytest Suite: GET v2_blocks_ROUND
// Polytest Group: Common Tests
//
// Deferred: the endpoint only returns msgpack, but the generated BlockResponse types the block as a
// JSON value, which cannot represent the binary fields msgpack carries. Filling this needs a block
// model that decodes msgpack binary.
#[tokio::test]
#[ignore = "deferred: BlockResponse cannot decode the msgpack block body"]
async fn basic_request_and_response_validation() {}
