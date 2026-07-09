# Polytest Test Plan

## Test Suites


### Composer

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#composer-tests">Composer Tests</a></td>
      <td>Tests that exercise compose() and sign_transactions() across transaction types</td>
    </tr>
  </tbody>
</table>

## Test Groups


### Composer Tests

<table>
  <thead>
    <tr>
      <th>Name</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="#compose-payment">compose payment</a></td>
      <td>compose() with a single Payment TxnParams produces the canonical encoded transaction</td>
    </tr>
    <tr>
      <td><a href="#compose-asset-opt-in">compose asset opt in</a></td>
      <td>compose() with an AssetOptIn TxnParams produces a 0-amount asset transfer to self</td>
    </tr>
    <tr>
      <td><a href="#compose-asset-transfer">compose asset transfer</a></td>
      <td>compose() with an AssetTransfer TxnParams produces the canonical encoded transaction</td>
    </tr>
    <tr>
      <td><a href="#compose-online-keyreg">compose online keyreg</a></td>
      <td>compose() with an OnlineKeyReg TxnParams produces the canonical encoded transaction</td>
    </tr>
    <tr>
      <td><a href="#compose-offline-keyreg">compose offline keyreg</a></td>
      <td>compose() with an OfflineKeyReg TxnParams produces the canonical encoded transaction</td>
    </tr>
    <tr>
      <td><a href="#compose-multi-entry-assigns-shared-group">compose multi-entry assigns shared group</a></td>
      <td>compose() with more than one TxnParams returns transactions that share an atomic group ID</td>
    </tr>
    <tr>
      <td><a href="#sign-single-transaction">sign single transaction</a></td>
      <td>sign_transactions() with one encoded transaction and one secret key produces the canonical signed bytes</td>
    </tr>
    <tr>
      <td><a href="#sign-paired-transactions">sign paired transactions</a></td>
      <td>sign_transactions() pairs each transaction with the secret key at the same index</td>
    </tr>
    <tr>
      <td><a href="#sign-rejects-length-mismatch">sign rejects length mismatch</a></td>
      <td>sign_transactions() returns an error when txn count and key count differ</td>
    </tr>
  </tbody>
</table>

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
