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

@Test("Generic Transaction: empty transaction signer")
func testEmptyTransactionSigner() throws {
  let signer = EmptyTransactionSigner()
  let testData = try loadTestData()
  let transaction = makeTransaction(from: testData.simplePayment)
  let transactions = [transaction]

  // Sign with index 0
  let signed = try signer.signTransactions(
    transactions: transactions,
    indexesToSign: Data([0])
  )

  #expect(signed.count == 1)
  #expect(signed[0].signature != nil)
  #expect(signed[0].signature == Data(repeating: 0, count: 64))

  // Don't sign with any index
  let unsigned = try signer.signTransactions(
    transactions: transactions,
    indexesToSign: Data()
  )

  #expect(unsigned.count == 1)
  #expect(unsigned[0].signature == nil)
}
