import Foundation
import Testing

@testable import AlgoKitTransact

// Polytest Suite: Generic Transaction

// Polytest Group: Generic Transaction Tests

@Test("Generic Transaction: malformed bytes")
func genericTransactionMalformedBytes() throws {
  let testData = try loadTestData()
  let simplePayment = testData.simplePayment
  let badBytes = Data(simplePayment.unsignedBytes[13..<37])
  do {
    _ = try decodeTransaction(encodedTx: badBytes)
    #expect(Bool(false), "Expected DecodingError to be thrown")
  } catch AlgoKitTransactError.DecodingError {
    // Success - expected error was thrown
    #expect(Bool(true))
  }
}

@Test("Generic Transaction: encode 0 bytes")
func genericTransactionEncode0Bytes() throws {
  do {
    _ = try decodeTransaction(encodedTx: Data())
    #expect(Bool(false), "Expected DecodingError to be thrown")
  } catch AlgoKitTransactError.InputError(let message) {
    #expect(message == "attempted to decode 0 bytes")
  }
}

@Test("Generic Transaction: ed25519SignTransaction")
func testEd25519SignTransaction() throws {
  let testData = try loadTestData()
  let transaction = makeTransaction(from: testData.simplePayment)

  let signed = try ed25519SignTransaction(
    secretKey: Data(repeating: 1, count: 32), txn: transaction)

  #expect(signed.transaction == transaction)
  #expect(signed.signature?.isEmpty == false)
}
