import Foundation

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

func makeTransaction(from testData: TestData) -> Transaction {
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