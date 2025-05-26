use algokit_msgpack::{
    decode_base64_msgpack_to_json, encode_json_to_base64_msgpack, encode_json_to_msgpack,
    ModelType, SimulateTransaction200Response,
};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

#[test]
fn test_decode_simulate_response_200() {
    let base64_msgpack = "hq5ldmFsLW92ZXJyaWRlc4S2YWxsb3ctZW1wdHktc2lnbmF0dXJlc8O3YWxsb3ctdW5uYW1lZC1yZXNvdXJjZXPDrW1heC1sb2ctY2FsbHPNCACsbWF4LWxvZy1zaXplzgABAACxZXhlYy10cmFjZS1jb25maWeEpmVuYWJsZcOuc2NyYXRjaC1jaGFuZ2XDrHN0YWNrLWNoYW5nZcOsc3RhdGUtY2hhbmdlw65pbml0aWFsLXN0YXRlc4CqbGFzdC1yb3VuZDWqdHhuLWdyb3Vwc5GBq3R4bi1yZXN1bHRzkYGqdHhuLXJlc3VsdIKqcG9vbC1lcnJvcqCjdHhugqNzaWfEQMRvOrLGLclzOfFppoyvhgTXsC+h/Qw59v5hc4k7CA9oVmEJZpcqjxweDlJg1C/vElTWwXL0zA/U59Ua/DjLhw+jdHhuiaNhbXTOAA9CQKNmZWXNA+iiZnY1o2dlbqxkb2NrZXJuZXQtdjGiZ2jEIEeJCm8ejvOqNCXVH+4GP95TdhioDiMH0wMRTIiwAmAUomx2zQQdo3JjdsQgOpJtq/2KwvdRn45on+Fhv0qXhguGb2ZMduXle8VCoPSjc25kxCA6km2r/YrC91Gfjmif4WG/SpeGC4ZvZkx25eV7xUKg9KR0eXBlo3Bhead2ZXJzaW9uAg==";

    let json_str =
        decode_base64_msgpack_to_json(ModelType::SimulateTransaction200Response, base64_msgpack)
            .expect("Failed to decode MessagePack");

    let resp: SimulateTransaction200Response = serde_json::from_str(&json_str)
        .expect("Failed to parse JSON into SimulateTransaction200Response");

    assert!(resp.version >= 1, "version should be positive");
    assert!(
        !resp.txn_groups.is_empty(),
        "should contain at least one transaction group"
    );
}

#[test]
fn test_encode() {
    let simulate_request_json = r#"{"txn-groups": [{"txns": ["gqNzaWfEQC0RQ1E6Y+/iS6luFP6Q9c6Veo838jRIABcV+jSzetx61nlrmasonRDbxN02mbCESJw98o7IfKgQvSMvk9kE0gqjdHhuiaNhbXTOAA9CQKNmZWXNA+iiZnYzo2dlbqxkb2NrZXJuZXQtdjGiZ2jEIEeJCm8ejvOqNCXVH+4GP95TdhioDiMH0wMRTIiwAmAUomx2zQQbo3JjdsQg/x0nrFM+VxALq2Buu1UscgDBy0OKIY2MGnDzg8xkNaOjc25kxCD/HSesUz5XEAurYG67VSxyAMHLQ4ohjYwacPODzGQ1o6R0eXBlo3BheQ=="]}], "allow-empty-signatures": true, "allow-more-logging": true, "allow-unnamed-resources": true, "exec-trace-config": {"enable": true, "stack-change": true, "scratch-change": true, "state-change": true}}"#;

    let expected_base64 = "hbZhbGxvdy1lbXB0eS1zaWduYXR1cmVzw7JhbGxvdy1tb3JlLWxvZ2dpbmfDt2FsbG93LXVubmFtZWQtcmVzb3VyY2Vzw7FleGVjLXRyYWNlLWNvbmZpZ4SmZW5hYmxlw6xzdGFjay1jaGFuZ2XDrnNjcmF0Y2gtY2hhbmdlw6xzdGF0ZS1jaGFuZ2XDqnR4bi1ncm91cHORgaR0eG5zkYKjc2lnxEAtEUNROmPv4kupbhT+kPXOlXqPN/I0SAAXFfo0s3rcetZ5a5mrKJ0Q28TdNpmwhEicPfKOyHyoEL0jL5PZBNIKo3R4bomjYW10zgAPQkCjZmVlzQPoomZ2M6NnZW6sZG9ja2VybmV0LXYxomdoxCBHiQpvHo7zqjQl1R/uBj/eU3YYqA4jB9MDEUyIsAJgFKJsds0EG6NyY3bEIP8dJ6xTPlcQC6tgbrtVLHIAwctDiiGNjBpw84PMZDWjo3NuZMQg/x0nrFM+VxALq2Buu1UscgDBy0OKIY2MGnDzg8xkNaOkdHlwZaNwYXk=";

    let msgpack_bytes = encode_json_to_msgpack(ModelType::SimulateRequest, simulate_request_json)
        .expect("Failed to encode SimulateRequest");

    let actual_base64 = BASE64.encode(&msgpack_bytes);
    println!("Actual base64: {}", actual_base64);

    assert_eq!(
        actual_base64, expected_base64,
        "Base64 encoded MessagePack doesn't match expected value"
    );
}

#[test]
fn test_decode() {
    let simulate_request_json = r#"{"txn-groups": [{"txns": ["gqNzaWfEQC0RQ1E6Y+/iS6luFP6Q9c6Veo838jRIABcV+jSzetx61nlrmasonRDbxN02mbCESJw98o7IfKgQvSMvk9kE0gqjdHhuiaNhbXTOAA9CQKNmZWXNA+iiZnYzo2dlbqxkb2NrZXJuZXQtdjGiZ2jEIEeJCm8ejvOqNCXVH+4GP95TdhioDiMH0wMRTIiwAmAUomx2zQQbo3JjdsQg/x0nrFM+VxALq2Buu1UscgDBy0OKIY2MGnDzg8xkNaOjc25kxCD/HSesUz5XEAurYG67VSxyAMHLQ4ohjYwacPODzGQ1o6R0eXBlo3BheQ=="]}], "allow-empty-signatures": true, "allow-more-logging": true, "allow-unnamed-resources": true, "exec-trace-config": {"enable": true, "stack-change": true, "scratch-change": true, "state-change": true}}"#;

    let msgpack_bytes =
        encode_json_to_base64_msgpack(ModelType::SimulateRequest, simulate_request_json)
            .expect("Failed to encode SimulateRequest");

    // SimulateRequest decoding is intentionally not supported
    let result = decode_base64_msgpack_to_json(ModelType::SimulateRequest, &msgpack_bytes);
    assert!(
        result.is_err(),
        "SimulateRequest decoding should return an error"
    );

    if let Err(e) = result {
        assert!(
            e.to_string().contains("not supported"),
            "Error should mention 'not supported'"
        );
    }
}
