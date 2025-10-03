use std::collections::BTreeMap;

use crate::{
    Byte32, TransactionHeaderBuilder,
    transactions::state_proof::{
        FalconSignatureStruct, FalconVerifier, HashFactory, MerkleArrayProof,
        MerkleSignatureVerifier, Participant, Reveal, SigslotCommit, StateProof, StateProofMessage,
        StateProofTransactionBuilder,
    },
};
use base64::{Engine, prelude::BASE64_STANDARD};

pub struct StateProofTransactionMother {}

fn convert_to_bytes(str: &str) -> Vec<u8> {
    BASE64_STANDARD.decode(str).unwrap()
}

impl StateProofTransactionMother {
    pub fn state_proof() -> StateProofTransactionBuilder {
        // Load state proof data from JSON file
        let json_data = algokit_test_artifacts::msgpack::TESTNET_STATE_PROOF_TX;
        let json_value: serde_json::Value = serde_json::from_str(json_data).unwrap();

        let genesis_hash: Byte32 = convert_to_bytes(json_value["genesis-hash"].as_str().unwrap())
            .try_into()
            .unwrap();

        let header = TransactionHeaderBuilder::default()
            .sender(json_value["sender"].as_str().unwrap().parse().unwrap())
            .first_valid(json_value["first-valid"].as_u64().unwrap())
            .last_valid(json_value["last-valid"].as_u64().unwrap())
            .fee(json_value["fee"].as_u64().unwrap())
            .genesis_hash(genesis_hash)
            .build()
            .unwrap();

        StateProofTransactionBuilder::default()
            .header(header)
            .state_proof_type(json_value["state-proof-type"].as_u64().unwrap_or(0))
            .message({
                let msg = &json_value["state-proof-transaction"]["message"];
                StateProofMessage {
                    block_headers_commitment: BASE64_STANDARD
                        .decode(msg["block-headers-commitment"].as_str().unwrap())
                        .unwrap(),
                    first_attested_round: msg["first-attested-round"].as_u64().unwrap(),
                    last_attested_round: msg["latest-attested-round"].as_u64().unwrap(),
                    ln_proven_weight: msg["ln-proven-weight"].as_u64().unwrap(),
                    voters_commitment: BASE64_STANDARD
                        .decode(msg["voters-commitment"].as_str().unwrap())
                        .unwrap(),
                }
            })
            .state_proof({
                let sp = &json_value["state-proof-transaction"]["state-proof"];

                // Build reveals BTreeMap from JSON array
                let mut reveals = BTreeMap::new();
                for reveal_obj in sp["reveals"].as_array().unwrap() {
                    let position = reveal_obj["position"].as_u64().unwrap();
                    let reveal = Reveal {
                        sigslot: SigslotCommit {
                            sig: FalconSignatureStruct {
                                signature: BASE64_STANDARD
                                    .decode(
                                        reveal_obj["sig-slot"]["signature"]["falcon-signature"]
                                            .as_str()
                                            .unwrap(),
                                    )
                                    .unwrap(),
                                vector_commitment_index:
                                    reveal_obj["sig-slot"]["signature"]["merkle-array-index"]
                                        .as_u64()
                                        .unwrap(),
                                proof: MerkleArrayProof {
                                    hash_factory: HashFactory { hash_type: 1 },
                                    path: reveal_obj["sig-slot"]["signature"]["proof"]["path"]
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .map(|p| {
                                            BASE64_STANDARD.decode(p.as_str().unwrap()).unwrap()
                                        })
                                        .collect(),
                                    tree_depth:
                                        reveal_obj["sig-slot"]["signature"]["proof"]["tree-depth"]
                                            .as_u64()
                                            .unwrap(),
                                },
                                verifying_key: FalconVerifier {
                                    public_key: BASE64_STANDARD
                                        .decode(
                                            reveal_obj["sig-slot"]["signature"]["verifying-key"]
                                                .as_str()
                                                .unwrap(),
                                        )
                                        .unwrap(),
                                },
                            },
                            lower_sig_weight: reveal_obj["sig-slot"]["lower-sig-weight"]
                                .as_u64()
                                .unwrap(),
                        },
                        participant: Participant {
                            verifier: MerkleSignatureVerifier {
                                commitment: BASE64_STANDARD
                                    .decode(
                                        reveal_obj["participant"]["verifier"]["commitment"]
                                            .as_str()
                                            .unwrap(),
                                    )
                                    .unwrap()
                                    .try_into()
                                    .unwrap(),
                                key_lifetime: reveal_obj["participant"]["verifier"]["key-lifetime"]
                                    .as_u64()
                                    .unwrap(),
                            },
                            weight: reveal_obj["participant"]["weight"].as_u64().unwrap(),
                        },
                    };
                    reveals.insert(position, reveal);
                }

                StateProof {
                    part_proofs: MerkleArrayProof {
                        hash_factory: HashFactory { hash_type: 1 },
                        path: sp["part-proofs"]["path"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|p| BASE64_STANDARD.decode(p.as_str().unwrap()).unwrap())
                            .collect(),
                        tree_depth: sp["part-proofs"]["tree-depth"].as_u64().unwrap(),
                    },
                    sig_commit: BASE64_STANDARD
                        .decode(sp["sig-commit"].as_str().unwrap())
                        .unwrap(),
                    signed_weight: sp["signed-weight"].as_u64().unwrap(),
                    sig_proofs: MerkleArrayProof {
                        hash_factory: HashFactory { hash_type: 1 },
                        path: sp["sig-proofs"]["path"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|p| BASE64_STANDARD.decode(p.as_str().unwrap()).unwrap())
                            .collect(),
                        tree_depth: sp["sig-proofs"]["tree-depth"].as_u64().unwrap(),
                    },
                    merkle_signature_salt_version: sp["salt-version"].as_u64().unwrap_or(0),
                    reveals,
                    positions_to_reveal: sp["positions-to-reveal"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_u64().unwrap())
                        .collect(),
                }
            })
            .to_owned()
    }
}
