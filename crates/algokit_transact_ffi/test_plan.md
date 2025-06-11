# Polytest Test Plan
## Test Suites

### Payment

| Name | Description |
| --- | --- |
| [Transaction Tests](#transaction-tests) | Tests that apply to all transaction types |

### Generic Transaction

| Name | Description |
| --- | --- |
| [Generic Transaction Tests](#generic-transaction-tests) | Generic transaction-related tests |

## Test Groups

### Generic Transaction Tests

| Name | Description |
| --- | --- |
| [encode 0 bytes](#encode-0-bytes) | Ensure a helpful error message is thrown when attempting to encode 0 bytes |
| [malformed bytes](#malformed-bytes) | Ensure a helpful error message is thrown when attempting to decode malformed bytes |

### Transaction Tests

| Name | Description |
| --- | --- |
| [encode](#encode) | A transaction with valid fields is encoded properly |
| [encode with signature](#encode-with-signature) | A signature can be attached to a encoded transaction |
| [encode with auth address](#encode-with-auth-address) | An auth address can be attached to a encoded transaction with a signature |
| [decode with prefix](#decode-with-prefix) | A transaction with TX prefix and valid fields is decoded properly |
| [decode without prefix](#decode-without-prefix) | A transaction without TX prefix and valid fields is decoded properly |
| [get encoded transaction type](#get-encoded-transaction-type) | The transaction type of an encoded transaction can be retrieved |
| [get transaction id](#get-transaction-id) | A transaction id can be obtained from a transaction |
| [example](#example) | A human-readable example of forming a transaction and signing it |

## Test Cases

### encode 0 bytes

Ensure a helpful error message is thrown when attempting to encode 0 bytes

### malformed bytes

Ensure a helpful error message is thrown when attempting to decode malformed bytes

### encode

A transaction with valid fields is encoded properly

### encode with signature

A signature can be attached to a encoded transaction

### encode with auth address

An auth address can be attached to a encoded transaction with a signature

### decode with prefix

A transaction with TX prefix and valid fields is decoded properly

### decode without prefix

A transaction without TX prefix and valid fields is decoded properly

### get encoded transaction type

The transaction type of an encoded transaction can be retrieved

### get transaction id

A transaction id can be obtained from a transaction

### example

A human-readable example of forming a transaction and signing it
