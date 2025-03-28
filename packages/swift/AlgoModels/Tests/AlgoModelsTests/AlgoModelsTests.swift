import Foundation
import Testing
import ed25519swift

@testable import AlgoModels

struct TestData: Codable {
    struct AddressData: Codable {
        let address: String
        let pubKey: [UInt8]
    }

    struct TransactionData: Codable {
        let header: HeaderData
        let payFields: PayFieldsData
    }

    struct HeaderData: Codable {
        let sender: AddressData
        let fee: UInt64
        let transactionType: String
        let firstValid: UInt64
        let lastValid: UInt64
        let genesisHash: [UInt8]
        let genesisId: String
    }

    struct PayFieldsData: Codable {
        let receiver: AddressData
        let amount: UInt64
    }

    let privKey: [UInt8]
    let transaction: TransactionData
    let expectedBytesForSigning: [UInt8]
    let expectedSignedTxn: [UInt8]
}

func loadTestData() throws -> TestData {
    let testDataURL = Bundle.module.url(forResource: "test_data", withExtension: "json")!
    let data = try Data(contentsOf: testDataURL)
    let decoder = JSONDecoder()
    return try decoder.decode(TestData.self, from: data)
}

@Suite("AlgoModels Tests")
struct AlgoModelsTests {
    @Test("encode transaction")
    func testEncode() async throws {
        let testData = try loadTestData()
        let transaction = makeTransaction(from: testData)
        let encoded = try encodeTransaction(tx: transaction)
        #expect([UInt8](encoded) == testData.expectedBytesForSigning)
    }

    @Test("encode with signature")
    func testEncodeWithSignature() async throws {
        let testData = try loadTestData()
        // Note: You'll need to implement actual ed25519 expectedBytesForSigning
        let signature = Ed25519.sign(
            message: testData.expectedBytesForSigning, secretKey: testData.privKey)
        let signedTx = try attachSignature(
            encodedTx: Data(testData.expectedBytesForSigning),
            signature: Data(signature)
        )
        #expect([UInt8](signedTx) == testData.expectedSignedTxn)
    }

    @Test("decode with TX prefix")
    func testDecodeWithTxPrefix() async throws {
        let testData = try loadTestData()
        let transaction = makeTransaction(from: testData)
        let decoded = try decodeTransaction(bytes: Data(testData.expectedBytesForSigning))
        #expect(decoded == transaction)
    }

    @Test("decode without TX prefix")
    func testDecodeWithoutTxPrefix() async throws {
        let testData = try loadTestData()
        let transaction = makeTransaction(from: testData)
        let bytesWithoutPrefix = Data(testData.expectedBytesForSigning.dropFirst(2))
        let decoded = try decodeTransaction(bytes: bytesWithoutPrefix)
        #expect(decoded == transaction)
    }

    @Test("get encoded transaction type")
    func testGetEncodedTransactionType() async throws {
        let testData = try loadTestData()
        let txType = try getEncodedTransactionType(bytes: Data(testData.expectedBytesForSigning))
        #expect(txType == .payment)
    }

    @Test("decoding error - 0 bytes")
    func testDecodingError0Bytes() async throws {
        do {
            _ = try decodeTransaction(bytes: Data())
            #expect(Bool(false), "Expected DecodingError to be thrown")
        } catch AlgoModelsError.DecodingError(let message) {
            #expect(message == "attempted to decode 0 bytes")
        }
    }

    @Test("decoding error - malformed bytes")
    func testDecodingErrorMalformedBytes() async throws {
        let testData = try loadTestData()
        let badBytes = Data(testData.expectedBytesForSigning[13..<37])
        do {
            _ = try decodeTransaction(bytes: badBytes)
            #expect(Bool(false), "Expected DecodingError to be thrown")
        } catch AlgoModelsError.DecodingError {
            // Success - expected error was thrown
            #expect(Bool(true))
        }
    }

    @Test("Example")
    func testExample() async throws {
        let aliceKeyPair = Ed25519.generateKeyPair()
        let alice = try addressFromPubKey(pubKey: Data(aliceKeyPair.publicKey))
        let bob = try addressFromString(
            address: "B72WNFFEZ7EOGMQPP7ROHYS3DSLL5JW74QASYNWGZGQXWRPJECJJLJIJ2Y"
        )

        let txn: Transaction = Transaction(
            header: TransactionHeader(
                transactionType: .payment,
                sender: alice,
                fee: 1000,
                firstValid: 1337,
                lastValid: 1347,
                genesisHash: Data(repeating: 65, count: 32),  // pretend this is a valid hash
                genesisId: "localnet"
            ),
            payFields: PayTransactionFields(
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

    // Helper function to create transaction from test data
    private func makeTransaction(from testData: TestData) -> Transaction {
        return Transaction(
            header: TransactionHeader(
                transactionType: .payment,
                sender: Address(
                    address: testData.transaction.header.sender.address,
                    pubKey: Data(testData.transaction.header.sender.pubKey)
                ),
                fee: testData.transaction.header.fee,
                firstValid: testData.transaction.header.firstValid,
                lastValid: testData.transaction.header.lastValid,
                genesisHash: Data(testData.transaction.header.genesisHash),
                genesisId: testData.transaction.header.genesisId,
                note: nil,
                rekeyTo: nil,
                lease: nil,
                group: nil
            ),
            payFields: PayTransactionFields(
                receiver: Address(
                    address: testData.transaction.payFields.receiver.address,
                    pubKey: Data(testData.transaction.payFields.receiver.pubKey)
                ),
                amount: testData.transaction.payFields.amount,
                closeRemainderTo: nil
            ),
            assetTransferFields: nil
        )
    }
}
