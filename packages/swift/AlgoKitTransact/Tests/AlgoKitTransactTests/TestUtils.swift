import Foundation

@testable import AlgoKitTransact

struct TransactionTestData: Codable {
    struct AddressData: Codable {
        let address: String
        let pubKey: [UInt8]
    }

    struct TransactionData: Codable {
        let header: HeaderData
        let payment: PaymentFieldsData
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

    struct PaymentFieldsData: Codable {
        let receiver: AddressData
        let amount: UInt64
    }

    let transaction: TransactionData
    let id: String
    let rawId: [UInt8]
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
