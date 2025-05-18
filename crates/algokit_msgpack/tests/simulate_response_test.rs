use algokit_msgpack::{decode_base64_msgpack_to_json, ModelType, SimulateTransaction200Response};

#[test]
fn test_decode_simulate_response_200() {
    // Base64‐encoded MessagePack payload for a /v2/transactions/simulate 200-OK response
    let base64_msgpack = "hq5ldmFsLW92ZXJyaWRlc4S2YWxsb3ctZW1wdHktc2lnbmF0dXJlc8O3YWxsb3ctdW5uYW1lZC1yZXNvdXJjZXPDrW1heC1sb2ctY2FsbHPNCACsbWF4LWxvZy1zaXplzgABAACxZXhlYy10cmFjZS1jb25maWeEpmVuYWJsZcOuc2NyYXRjaC1jaGFuZ2XDrHN0YWNrLWNoYW5nZcOsc3RhdGUtY2hhbmdlw65pbml0aWFsLXN0YXRlc4CqbGFzdC1yb3VuZDWqdHhuLWdyb3Vwc5GBq3R4bi1yZXN1bHRzkYGqdHhuLXJlc3VsdIKqcG9vbC1lcnJvcqCjdHhugqNzaWfEQMRvOrLGLclzOfFppoyvhgTXsC+h/Qw59v5hc4k7CA9oVmEJZpcqjxweDlJg1C/vElTWwXL0zA/U59Ua/DjLhw+jdHhuiaNhbXTOAA9CQKNmZWXNA+iiZnY1o2dlbqxkb2NrZXJuZXQtdjGiZ2jEIEeJCm8ejvOqNCXVH+4GP95TdhioDiMH0wMRTIiwAmAUomx2zQQdo3JjdsQgOpJtq/2KwvdRn45on+Fhv0qXhguGb2ZMduXle8VCoPSjc25kxCA6km2r/YrC91Gfjmif4WG/SpeGC4ZvZkx25eV7xUKg9KR0eXBlo3Bhead2ZXJzaW9uAg==";

    // Attempt to decode into JSON string first
    let json_str =
        decode_base64_msgpack_to_json(ModelType::SimulateTransaction200Response, base64_msgpack)
            .expect("Failed to decode MessagePack");

    // Ensure JSON parses into our Rust struct
    let resp: SimulateTransaction200Response = serde_json::from_str(&json_str)
        .expect("Failed to parse JSON into SimulateTransaction200Response");

    // Basic sanity assertions
    assert!(resp.version >= 1, "version should be positive");
    assert!(
        !resp.txn_groups.is_empty(),
        "should contain at least one transaction group"
    );
}
