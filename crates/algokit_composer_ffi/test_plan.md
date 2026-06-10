# Polytest Test Plan
## Test Suites

### Composer

| Name | Description |
| --- | --- |
| [Composer Tests](#composer-tests) | Tests that exercise compose() and sign_transactions() across transaction types |

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
