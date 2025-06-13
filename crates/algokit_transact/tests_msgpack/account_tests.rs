use algokit_msgpack::{Account, AlgoKitMsgPackError, ModelType};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

#[test]
fn test_account_decoding_error() {
    // Test that encoding returns the expected error
    let mut account = Account::default();
    account.address = "ABCDEFG".to_string();
    account.amount = 100;
    account.min_balance = 50;
    account.amount_without_pending_rewards = 90;
    account.pending_rewards = 10;
    account.rewards = 20;
    account.round = 1234;
    account.status = "Online".to_string();
    account.total_apps_opted_in = 0;
    account.total_assets_opted_in = 0;
    account.total_created_apps = 0;
    account.total_created_assets = 0;

    // Convert to JSON
    let json = serde_json::to_string(&account).unwrap();

    // Test error response for encoding
    let result = algokit_msgpack::encode_json_to_msgpack(ModelType::Account, &json);
    assert!(result.is_err());
    if let Err(AlgoKitMsgPackError::MsgpackWriteError(msg)) = result {
        assert!(msg.contains("not supported"));
    } else {
        panic!("Expected MsgpackWriteError");
    }
}

#[test]
fn test_decode_simplified_account_msgpack() {
    // This is a complex MessagePack structure with an "algo" field that contains the account amount
    // The structure includes multiple fields with both string and integer keys
    let msgpack_base64 = "haRhbGdvzgADh4SkYXBhcoHNBC+IomFuplNUUC0jMaJhddlCaXBmczovL2JhZnliZWljZGR6N2tidXhhamo2Ym9iNWJqcXR3ZXE2d2Noa2RraXE0dnZod3J3cm5lN2l6NGYyNXhpoWPEIDB4md+EJXeeSDFN4bcfh84mIJ1V2fPQfw0osXuAyMvtomRmw6FtxCAweJnfhCV3nkgxTeG3H4fOJiCdVdnz0H8NKLF7gMjL7aFyxCAweJnfhCV3nkgxTeG3H4fOJiCdVdnz0H8NKLF7gMjL7aF0AaJ1bqNTVFClYXNzZXSBzQQvgaFhAaN0YngBpHRieGJI";
    let msgpack_bytes = BASE64
        .decode(msgpack_base64)
        .expect("Failed to decode base64");

    let json_str = algokit_msgpack::decode_msgpack_to_json(ModelType::Account, &msgpack_bytes)
        .expect("Failed to decode simplified account MessagePack");

    println!("Decoded JSON: {}", json_str);

    // The result should be a JSON object with the amount field set to 231300 (0x0387 84)
    let account: serde_json::Value =
        serde_json::from_str(&json_str).expect("Failed to parse decoded JSON");

    assert_eq!(account["amount"], 231300);
}

#[test]
fn test_decode_complex_account_msgpack() {
    // This is a more complex MessagePack structure with multiple fields
    let msgpack_base64 = "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";
    let msgpack_bytes = BASE64
        .decode(msgpack_base64)
        .expect("Failed to decode base64");

    let json_str = algokit_msgpack::decode_msgpack_to_json(ModelType::Account, &msgpack_bytes)
        .expect("Failed to decode complex account MessagePack");

    println!("Decoded JSON: {}", json_str);

    // Parse the resulting JSON
    let account: serde_json::Value =
        serde_json::from_str(&json_str).expect("Failed to parse decoded JSON");

    // Verify the amount was extracted correctly - should be 100080000 (0x05F719 80)
    assert_eq!(account["amount"], 100080000);

    // Verify created_assets, created_apps, and assets fields are populated using the actual key names
    assert!(account["created-assets"].is_array());
    assert!(account["created-apps"].is_array());
    assert!(account["assets"].is_array());

    // Check counts
    assert_eq!(account["total-created-assets"], 10);
    assert_eq!(account["total-created-apps"], 10);
    assert_eq!(account["total-assets-opted-in"], 10);
}
