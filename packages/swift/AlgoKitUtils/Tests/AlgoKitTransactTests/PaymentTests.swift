import Foundation
import Testing
import ed25519swift

@testable import AlgoKitTransact

// Polytest Suite: Payment

// Polytest Group: Transaction Tests

@Test("Payment: example")
func paymentExample() throws {
  let aliceKeyPair = Ed25519.generateKeyPair()
  let alice = try addressFromPublicKey(publicKey: Data(aliceKeyPair.publicKey))
  let bob = "B72WNFFEZ7EOGMQPP7ROHYS3DSLL5JW74QASYNWGZGQXWRPJECJJLJIJ2Y"

  let txn: Transaction = Transaction(
    transactionType: .payment,
    sender: alice,
    fee: 1000,
    firstValid: 1337,
    lastValid: 1347,
    genesisHash: Data(repeating: 65, count: 32),  // pretend this is a valid hash
    genesisId: "localnet",
    payment: PaymentTransactionFields(
      receiver: bob,
      amount: 1337
    )
  )

  let sig = Ed25519.sign(
    message: [UInt8](try encodeTransaction(transaction: txn)), secretKey: aliceKeyPair.secretKey)

  let signedTxn = SignedTransaction(
    transaction: txn,
    signature: Data(sig)
  )

  let encodedStxn = try encodeSignedTransaction(signedTransaction: signedTxn)

  #expect(encodedStxn.count > 0)
}

@Test("Payment: get encoded transaction type")
func paymentGetEncodedTransactionType() throws {
  let testData = try loadTestData()
  let simplePayment = testData.simplePayment
  let txType = try getEncodedTransactionType(encodedTransaction: Data(simplePayment.unsignedBytes))
  #expect(txType == .payment)
}

@Test("Payment: decode without prefix")
func paymentDecodeWithoutPrefix() throws {
  let testData = try loadTestData()
  let simplePayment = testData.simplePayment
  let transaction = makeTransaction(from: simplePayment)
  let bytesWithoutPrefix = Data(simplePayment.unsignedBytes.dropFirst(2))
  let decoded = try decodeTransaction(encodedTx: bytesWithoutPrefix)
  #expect(decoded == transaction)
}

@Test("Payment: decode with prefix")
func paymentDecodeWithPrefix() throws {
  let testData = try loadTestData()
  let simplePayment = testData.simplePayment
  let transaction = makeTransaction(from: simplePayment)
  let decoded = try decodeTransaction(encodedTx: Data(simplePayment.unsignedBytes))
  #expect(decoded == transaction)
}

@Test("Payment: encode with signature")
func paymentEncodeWithSignature() throws {
  let testData = try loadTestData()
  let simplePayment = testData.simplePayment
  let signature = Ed25519.sign(
    message: simplePayment.unsignedBytes, secretKey: simplePayment.signingPrivateKey)

  let signedTx = SignedTransaction(
    transaction: makeTransaction(from: simplePayment),
    signature: Data(signature)
  )

  let encodedStxn = try encodeSignedTransaction(signedTransaction: signedTx)

  #expect([UInt8](encodedStxn) == simplePayment.signedBytes)
}

@Test("Payment: encode")
func paymentEncode() throws {
  let testData = try loadTestData()
  let simplePayment = testData.simplePayment
  let transaction = makeTransaction(from: simplePayment)
  let encoded = try encodeTransaction(transaction: transaction)
  #expect([UInt8](encoded) == simplePayment.unsignedBytes)
}
