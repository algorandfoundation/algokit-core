// Polytest Suite: POST v2_teal_dryrun
// Polytest Group: Common Tests
//
// Deferred: the node returns null for empty arrays (e.g. DryrunTxnResult.disassembly,
// DryrunState.stack) that the generated response models type as required, non-optional Vecs, so a
// real dryrun response fails to decode. Filling this needs the generated models to accept null as
// an empty Vec.
#[tokio::test]
#[ignore = "deferred: dryrun response models reject the node's null arrays"]
async fn basic_request_and_response_validation() {}
