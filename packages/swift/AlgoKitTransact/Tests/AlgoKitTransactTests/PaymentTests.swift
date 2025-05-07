import Foundation
import Testing
import ed25519swift

@testable import AlgoKitTransact

// Polytest Suite: Payment

// Polytest Group: Transaction Tests

@Test("Payment: example")
func paymentExample() throws {
    let aliceKeyPair = Ed25519.generateKeyPair()
    let alice = try addressFromPubKey(pubKey: Data(aliceKeyPair.publicKey))
    let bob = try addressFromString(
        address: "B72WNFFEZ7EOGMQPP7ROHYS3DSLL5JW74QASYNWGZGQXWRPJECJJLJIJ2Y"
    )

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
        message: [UInt8](try encodeTransaction(tx: txn)), secretKey: aliceKeyPair.secretKey)

    let signedTxn = try attachSignature(
        encodedTx: try encodeTransaction(tx: txn),
        signature: Data(sig)
    )

    #expect(signedTxn.count > 0)
}

@Test("Payment: get encoded transaction type")
func paymentGetEncodedTransactionType() throws {
    let testData = try loadTestData()
    let simplePayment = testData.simplePayment
    let txType = try getEncodedTransactionType(bytes: Data(simplePayment.unsignedBytes))
    #expect(txType == .payment)
}

@Test("Payment: decode without prefix")
func paymentDecodeWithoutPrefix() throws {
    let testData = try loadTestData()
    let simplePayment = testData.simplePayment
    let transaction = makeTransaction(from: simplePayment)
    let bytesWithoutPrefix = Data(simplePayment.unsignedBytes.dropFirst(2))
    let decoded = try decodeTransaction(bytes: bytesWithoutPrefix)
    #expect(decoded == transaction)
}

@Test("Payment: decode with prefix")
func paymentDecodeWithPrefix() throws {
    let testData = try loadTestData()
    let simplePayment = testData.simplePayment
    let transaction = makeTransaction(from: simplePayment)
    let decoded = try decodeTransaction(bytes: Data(simplePayment.unsignedBytes))
    #expect(decoded == transaction)
}

@Test("Payment: encode with signature")
func paymentEncodeWithSignature() throws {
    let testData = try loadTestData()
    let simplePayment = testData.simplePayment
    let signature = Ed25519.sign(
        message: simplePayment.unsignedBytes, secretKey: simplePayment.signingPrivateKey)
    let signedTx = try attachSignature(
        encodedTx: Data(simplePayment.unsignedBytes),
        signature: Data(signature)
    )
    #expect([UInt8](signedTx) == simplePayment.signedBytes)
}

@Test("Payment: encode")
func paymentEncode() throws {
    let testData = try loadTestData()
    let simplePayment = testData.simplePayment
    let transaction = makeTransaction(from: simplePayment)
    let encoded = try encodeTransaction(tx: transaction)
    #expect([UInt8](encoded) == simplePayment.unsignedBytes)
}
