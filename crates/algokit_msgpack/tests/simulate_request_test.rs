use algokit_msgpack::{
    decode_base64_msgpack_to_json, encode_json_to_base64_msgpack, encode_json_to_msgpack,
    ModelType, SimulateRequest,
};
use base64::encode;

#[test]
fn test_encode() {
    // Sample SimulateRequest with base64 encoded transaction
    let simulate_request_json = r#"{"txn-groups": [{"txns": ["gqNzaWfEQC0RQ1E6Y+/iS6luFP6Q9c6Veo838jRIABcV+jSzetx61nlrmasonRDbxN02mbCESJw98o7IfKgQvSMvk9kE0gqjdHhuiaNhbXTOAA9CQKNmZWXNA+iiZnYzo2dlbqxkb2NrZXJuZXQtdjGiZ2jEIEeJCm8ejvOqNCXVH+4GP95TdhioDiMH0wMRTIiwAmAUomx2zQQbo3JjdsQg/x0nrFM+VxALq2Buu1UscgDBy0OKIY2MGnDzg8xkNaOjc25kxCD/HSesUz5XEAurYG67VSxyAMHLQ4ohjYwacPODzGQ1o6R0eXBlo3BheQ=="]}], "allow-empty-signatures": true, "allow-more-logging": true, "allow-unnamed-resources": true, "exec-trace-config": {"enable": true, "stack-change": true, "scratch-change": true, "state-change": true}}"#;

    // Expected base64 encoded msgpack result
    let expected_base64 = "hbZhbGxvdy1lbXB0eS1zaWduYXR1cmVzw7JhbGxvdy1tb3JlLWxvZ2dpbmfDt2FsbG93LXVubmFtZWQtcmVzb3VyY2Vzw7FleGVjLXRyYWNlLWNvbmZpZ4SmZW5hYmxlw6xzdGFjay1jaGFuZ2XDrnNjcmF0Y2gtY2hhbmdlw6xzdGF0ZS1jaGFuZ2XDqnR4bi1ncm91cHORgaR0eG5zkYKjc2lnxEAtEUNROmPv4kupbhT+kPXOlXqPN/I0SAAXFfo0s3rcetZ5a5mrKJ0Q28TdNpmwhEicPfKOyHyoEL0jL5PZBNIKo3R4bomjYW10zgAPQkCjZmVlzQPoomZ2M6NnZW6sZG9ja2VybmV0LXYxomdoxCBHiQpvHo7zqjQl1R/uBj/eU3YYqA4jB9MDEUyIsAJgFKJsds0EG6NyY3bEIP8dJ6xTPlcQC6tgbrtVLHIAwctDiiGNjBpw84PMZDWjo3NuZMQg/x0nrFM+VxALq2Buu1UscgDBy0OKIY2MGnDzg8xkNaOkdHlwZaNwYXk=";

    // Encode the SimulateRequest to MsgPack
    let msgpack_bytes = encode_json_to_msgpack(ModelType::SimulateRequest, simulate_request_json)
        .expect("Failed to encode SimulateRequest");

    // Encode the msgpack bytes to base64
    let actual_base64 = encode(&msgpack_bytes);

    // Print the actual result for debugging if needed
    println!("Actual base64: {}", actual_base64);

    // Verify the actual result matches the expected result
    assert_eq!(
        actual_base64, expected_base64,
        "Base64 encoded MessagePack doesn't match expected value"
    );
}

#[test]
fn test_decode() {
    // Sample SimulateRequest with base64 encoded transaction
    let simulate_request_json = r#"{"txn-groups": [{"txns": ["gqNzaWfEQC0RQ1E6Y+/iS6luFP6Q9c6Veo838jRIABcV+jSzetx61nlrmasonRDbxN02mbCESJw98o7IfKgQvSMvk9kE0gqjdHhuiaNhbXTOAA9CQKNmZWXNA+iiZnYzo2dlbqxkb2NrZXJuZXQtdjGiZ2jEIEeJCm8ejvOqNCXVH+4GP95TdhioDiMH0wMRTIiwAmAUomx2zQQbo3JjdsQg/x0nrFM+VxALq2Buu1UscgDBy0OKIY2MGnDzg8xkNaOjc25kxCD/HSesUz5XEAurYG67VSxyAMHLQ4ohjYwacPODzGQ1o6R0eXBlo3BheQ=="]}], "allow-empty-signatures": true, "allow-more-logging": true, "allow-unnamed-resources": true, "exec-trace-config": {"enable": true, "stack-change": true, "scratch-change": true, "state-change": true}}"#;

    // Encode the SimulateRequest to MsgPack
    let msgpack_bytes =
        encode_json_to_base64_msgpack(ModelType::SimulateRequest, simulate_request_json)
            .expect("Failed to encode SimulateRequest");

    // Encode the msgpack bytes to base64
    let decoded_json = decode_base64_msgpack_to_json(ModelType::SimulateRequest, &msgpack_bytes)
        .expect("Failed to decode SimulateRequest");

    let loaded_json: SimulateRequest =
        serde_json::from_str(&decoded_json).expect("Failed to parse decoded JSON");

    println!("Decoded JSON: {:?}", loaded_json);
}
