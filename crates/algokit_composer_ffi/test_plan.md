# Polytest Test Plan
## Test Suites

### Composer

| Name | Description |
| --- | --- |
| [Composer Tests](#composer-tests) | Tests that exercise compose() and sign_transactions() across transaction types |
| [Simulate Tests](#simulate-tests) | Tests that exercise the simulate request builder and response mapper |

## Test Groups

### Composer Tests

| Name | Description |
| --- | --- |
| [compose payment](#compose-payment) | compose() with a single Payment TxnParams produces the canonical encoded transaction |
| [compose asset opt in](#compose-asset-opt-in) | compose() with an AssetOptIn TxnParams produces a 0-amount asset transfer to self |
| [compose asset transfer](#compose-asset-transfer) | compose() with an AssetTransfer TxnParams produces the canonical encoded transaction |
| [compose online keyreg](#compose-online-keyreg) | compose() with an OnlineKeyReg TxnParams produces the canonical encoded transaction |
| [compose offline keyreg](#compose-offline-keyreg) | compose() with an OfflineKeyReg TxnParams produces the canonical encoded transaction |
| [compose multi-entry assigns shared group](#compose-multi-entry-assigns-shared-group) | compose() with more than one TxnParams returns transactions that share an atomic group ID |
| [sign single transaction](#sign-single-transaction) | sign_transactions() with one encoded transaction and one secret key produces the canonical signed bytes |
| [sign paired transactions](#sign-paired-transactions) | sign_transactions() pairs each transaction with the secret key at the same index |
| [sign rejects length mismatch](#sign-rejects-length-mismatch) | sign_transactions() returns an error when txn count and key count differ |

### Simulate Tests

| Name | Description |
| --- | --- |
| [build unsigned simulate request omits signatures](#build-unsigned-simulate-request-omits-signatures) | build_unsigned_simulate_request() produces txn entries carrying only a txn key, with no sig |
| [build signed simulate request preserves signatures](#build-signed-simulate-request-preserves-signatures) | build_signed_simulate_request() keeps the supplied signatures on each txn entry |
| [skip signatures forces allow empty signatures and fix signers](#skip-signatures-forces-allow-empty-signatures-and-fix-signers) | skip_signatures overrides caller-supplied allow_empty_signatures and fix_signers to true |
| [simulate options merge per field](#simulate-options-merge-per-field) | Setting one SimulateOptions field leaves the remaining request fields unset |
| [build simulate request rejects empty group](#build-simulate-request-rejects-empty-group) | Building a simulate request with no transactions returns an error |
| [map simulate response returns per transaction results](#map-simulate-response-returns-per-transaction-results) | map_simulate_response() projects tx ids and confirmations from the first group |
| [map simulate response surfaces group failure](#map-simulate-response-surfaces-group-failure) | map_simulate_response() reports failure_message and failed_at as data rather than raising |
| [map simulate response rejects group count mismatch](#map-simulate-response-rejects-group-count-mismatch) | map_simulate_response() errors when the response does not carry exactly one group |

## Test Cases

### compose payment

compose() with a single Payment TxnParams produces the canonical encoded transaction

### compose asset opt in

compose() with an AssetOptIn TxnParams produces a 0-amount asset transfer to self

### compose asset transfer

compose() with an AssetTransfer TxnParams produces the canonical encoded transaction

### compose online keyreg

compose() with an OnlineKeyReg TxnParams produces the canonical encoded transaction

### compose offline keyreg

compose() with an OfflineKeyReg TxnParams produces the canonical encoded transaction

### compose multi-entry assigns shared group

compose() with more than one TxnParams returns transactions that share an atomic group ID

### sign single transaction

sign_transactions() with one encoded transaction and one secret key produces the canonical signed bytes

### sign paired transactions

sign_transactions() pairs each transaction with the secret key at the same index

### sign rejects length mismatch

sign_transactions() returns an error when txn count and key count differ

### build unsigned simulate request omits signatures

build_unsigned_simulate_request() produces txn entries carrying only a txn key, with no sig

### build signed simulate request preserves signatures

build_signed_simulate_request() keeps the supplied signatures on each txn entry

### skip signatures forces allow empty signatures and fix signers

skip_signatures overrides caller-supplied allow_empty_signatures and fix_signers to true

### simulate options merge per field

Setting one SimulateOptions field leaves the remaining request fields unset

### build simulate request rejects empty group

Building a simulate request with no transactions returns an error

### map simulate response returns per transaction results

map_simulate_response() projects tx ids and confirmations from the first group

### map simulate response surfaces group failure

map_simulate_response() reports failure_message and failed_at as data rather than raising

### map simulate response rejects group count mismatch

map_simulate_response() errors when the response does not carry exactly one group
