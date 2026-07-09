# Polytest Test Plan

## Test Suites


### Payment

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#transaction-tests">Transaction Tests</a></td>
      <td>Tests that apply to all transaction types</td>
    </tr>
  </tbody>
</table>

### Asset Config

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#transaction-tests">Transaction Tests</a></td>
      <td>Tests that apply to all transaction types</td>
    </tr>
  </tbody>
</table>

### App Call

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#transaction-tests">Transaction Tests</a></td>
      <td>Tests that apply to all transaction types</td>
    </tr>
  </tbody>
</table>

### Generic Transaction

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#generic-transaction-tests">Generic Transaction Tests</a></td>
      <td>Generic transaction-related tests</td>
    </tr>
  </tbody>
</table>

### Transaction Group

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#transaction-group-tests">Transaction Group Tests</a></td>
      <td>Tests that apply to collections of transactions</td>
    </tr>
  </tbody>
</table>

### Key Registration

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#transaction-tests">Transaction Tests</a></td>
      <td>Tests that apply to all transaction types</td>
    </tr>
  </tbody>
</table>

### Heartbeat

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#transaction-tests">Transaction Tests</a></td>
      <td>Tests that apply to all transaction types</td>
    </tr>
  </tbody>
</table>

### State Proof

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#transaction-tests">Transaction Tests</a></td>
      <td>Tests that apply to all transaction types</td>
    </tr>
  </tbody>
</table>

## Test Groups


### Generic Transaction Tests

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#encode-0-bytes">encode 0 bytes</a></td>
      <td>Ensure a helpful error message is thrown when attempting to encode 0 bytes</td>
    </tr>
    <tr>
      <td><a href="#malformed-bytes">malformed bytes</a></td>
      <td>Ensure a helpful error message is thrown when attempting to decode malformed bytes</td>
    </tr>
  </tbody>
</table>

### Transaction Tests

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#encode">encode</a></td>
      <td>A transaction with valid fields is encoded properly</td>
    </tr>
    <tr>
      <td><a href="#encode-with-signature">encode with signature</a></td>
      <td>A signature can be attached to a encoded transaction</td>
    </tr>
    <tr>
      <td><a href="#encode-with-auth-address">encode with auth address</a></td>
      <td>An auth address can be attached to a encoded transaction with a signature</td>
    </tr>
    <tr>
      <td><a href="#decode-with-prefix">decode with prefix</a></td>
      <td>A transaction with TX prefix and valid fields is decoded properly</td>
    </tr>
    <tr>
      <td><a href="#decode-without-prefix">decode without prefix</a></td>
      <td>A transaction without TX prefix and valid fields is decoded properly</td>
    </tr>
    <tr>
      <td><a href="#get-encoded-transaction-type">get encoded transaction type</a></td>
      <td>The transaction type of an encoded transaction can be retrieved</td>
    </tr>
    <tr>
      <td><a href="#get-transaction-id">get transaction id</a></td>
      <td>A transaction id can be obtained from a transaction</td>
    </tr>
    <tr>
      <td><a href="#example">example</a></td>
      <td>A human-readable example of forming a transaction and signing it</td>
    </tr>
    <tr>
      <td><a href="#multisig-example">multisig example</a></td>
      <td>A human-readable example of forming a transaction and signing it with a multisignature sig</td>
    </tr>
    <tr>
      <td><a href="#assign-fee">assign fee</a></td>
      <td>A fee can be calculated and assigned to a transaction</td>
    </tr>
  </tbody>
</table>

### Transaction Group Tests

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#group-transactions">group transactions</a></td>
      <td>A collection of transactions can be grouped</td>
    </tr>
    <tr>
      <td><a href="#encode-transactions">encode transactions</a></td>
      <td>A collection of transactions can be encoded</td>
    </tr>
    <tr>
      <td><a href="#encode-signed-transactions">encode signed transactions</a></td>
      <td>A collection of signed transactions can be encoded</td>
    </tr>
  </tbody>
</table>

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

### multisig example

A human-readable example of forming a transaction and signing it with a multisignature sig

### assign fee

A fee can be calculated and assigned to a transaction

### group transactions

A collection of transactions can be grouped

### encode transactions

A collection of transactions can be encoded

### encode signed transactions

A collection of signed transactions can be encoded
