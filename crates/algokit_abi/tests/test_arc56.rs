use algokit_abi::arc56_contract::Arc56Contract;
use algokit_test_artifacts::{
    arc56_struct_operations, complex_struct_test, constant_product_amm, nested_contract_calls,
    nested_struct_storage, nfd, reti, state_management_demo, template_variables, void_return_test,
    zero_coupon_bond,
};
use rstest::rstest;

#[rstest]
#[case(template_variables::APPLICATION_ARC56, "template_variables")]
#[case(state_management_demo::APPLICATION_ARC56, "state_management_demo")]
#[case(constant_product_amm::APPLICATION_ARC56, "constant_product_amm")]
#[case(nested_struct_storage::APPLICATION_ARC56, "nested_struct_storage")]
#[case(arc56_struct_operations::APPLICATION_ARC56, "arc56_struct_operations")]
#[case(complex_struct_test::APPLICATION_ARC56, "complex_struct_test")]
#[case(zero_coupon_bond::APPLICATION_ARC56, "zero_coupon_bond")]
#[case(nfd::APPLICATION_ARC56, "nfd")]
#[case(reti::APPLICATION_ARC56, "reti")]
#[case(void_return_test::APPLICATION_ARC56, "void_return_test")]
#[case(nested_contract_calls::APPLICATION_ARC56, "nested_contract_calls")]
fn test_arc56_json_roundtrip(
    #[case] artifact_content: &str,
    #[case] case_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse JSON string to Arc56Contract
    let arc56_contract = Arc56Contract::from_json(artifact_content)?;

    // Serialize back to JSON with 4-space indentation
    let serialized_json = arc56_contract.to_json(Some(4))?;
    let parsed_json: serde_json::Value = serde_json::from_str(&serialized_json)?;

    // Snapshot test for regression detection
    insta::with_settings!({snapshot_suffix => case_name}, {
        insta::assert_json_snapshot!(parsed_json);
    });

    Ok(())
}

#[test]
fn test_arc56_state_keys_are_not_normalized() -> Result<(), Box<dyn std::error::Error>> {
    let arc56_contract = Arc56Contract::from_json(state_management_demo::APPLICATION_ARC56)?;
    let original_json: serde_json::Value =
        serde_json::from_str(state_management_demo::APPLICATION_ARC56)?;

    // Test keys that should preserve non-snake-case formatting
    let test_keys = [
        (
            &original_json["state"]["keys"]["global"],
            &arc56_contract.state.keys.global_state,
            "bytesNotInSnakeCase",
        ),
        (
            &original_json["state"]["keys"]["local"],
            &arc56_contract.state.keys.local_state,
            "localBytesNotInSnakeCase",
        ),
        (
            &original_json["state"]["keys"]["box"],
            &arc56_contract.state.keys.box_keys,
            "boxNotInSnakeCase",
        ),
    ];

    // Test box maps separately since they have a different structure (StorageMap vs StorageKey)
    let box_maps_test = (
        &original_json["state"]["maps"]["box"],
        &arc56_contract.state.maps.box_maps,
        "boxMapNotInSnakeCase",
    );

    // Verify original and parsed structures contain non-snake-case keys
    for (json_obj, parsed_map, key) in &test_keys {
        assert!(json_obj.as_object().unwrap().contains_key(*key));
        assert!(parsed_map.contains_key(*key));
    }

    // Verify box maps contain non-snake-case keys
    assert!(
        box_maps_test
            .0
            .as_object()
            .unwrap()
            .contains_key(box_maps_test.2)
    );
    assert!(box_maps_test.1.contains_key(box_maps_test.2));

    // Verify roundtrip preserves non-snake-case keys
    let exported_json = arc56_contract.to_json(Some(4))?;
    let exported_parsed: serde_json::Value = serde_json::from_str(&exported_json)?;

    for (_, _, key) in &test_keys {
        let paths = [
            &exported_parsed["state"]["keys"]["global"],
            &exported_parsed["state"]["keys"]["local"],
            &exported_parsed["state"]["keys"]["box"],
        ];
        assert!(
            paths
                .iter()
                .any(|obj| obj.as_object().unwrap().contains_key(*key))
        );
    }

    // Verify box maps preserve non-snake-case keys
    assert!(
        exported_parsed["state"]["maps"]["box"]
            .as_object()
            .unwrap()
            .contains_key(box_maps_test.2)
    );

    insta::assert_json_snapshot!(exported_parsed);
    Ok(())
}
