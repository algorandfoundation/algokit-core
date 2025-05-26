import Foundation

@testable import AlgoKitTransact

struct TransactionTestData: Codable {
    struct AddressData: Codable {
        let address: String
        let pubKey: [UInt8]
    }

    struct TransactionData: Codable {
        let sender: AddressData
        let fee: UInt64
        let transactionType: String
        let firstValid: UInt64
        let lastValid: UInt64
        let genesisHash: [UInt8]
        let genesisId: String
        let note: [UInt8]?
        let rekeyTo: AddressData?
        let lease: [UInt8]?
        let group: [UInt8]?
        let payment: PaymentFieldsData
    }

    struct PaymentFieldsData: Codable {
        let receiver: AddressData
        let amount: UInt64
    }

    let transaction: TransactionData
    let id: String
    let idRaw: [UInt8]
    let unsignedBytes: [UInt8];
    let signedBytes: [UInt8];
    let signingPrivateKey: [UInt8];
}

struct TestData: Codable {
    let simplePayment: TransactionTestData
}

func loadTestData() throws -> TestData {
    let testDataURL = Bundle.module.url(forResource: "test_data", withExtension: "json")!
    let data = try Data(contentsOf: testDataURL)
    let decoder = JSONDecoder()
    return try decoder.decode(TestData.self, from: data)
}

func makeTransaction(from testData: TransactionTestData) -> Transaction {
    return Transaction(
        transactionType: .payment,
        sender: Address(
            address: testData.transaction.sender.address,
            pubKey: Data(testData.transaction.sender.pubKey)
        ),
        fee: testData.transaction.fee,
        firstValid: testData.transaction.firstValid,
        lastValid: testData.transaction.lastValid,
        genesisHash: Data(testData.transaction.genesisHash),
        genesisId: testData.transaction.genesisId,
        note: testData.transaction.note != nil ? Data(testData.transaction.note!) : nil,
        rekeyTo: testData.transaction.rekeyTo != nil ? Address(
            address: testData.transaction.rekeyTo!.address,
            pubKey: Data(testData.transaction.rekeyTo!.pubKey)
        ) : nil,
        lease: testData.transaction.lease != nil ? Data(testData.transaction.lease!) : nil,
        group: testData.transaction.group != nil ? Data(testData.transaction.group!) : nil,
        payment: PaymentTransactionFields(
            receiver: Address(
                address: testData.transaction.payment.receiver.address,
                pubKey: Data(testData.transaction.payment.receiver.pubKey)
            ),
            amount: testData.transaction.payment.amount,
            closeRemainderTo: nil
        ),
        assetTransfer: nil
    )
}
