import algosdk from 'algosdk';

/**
 * Example TypeScript file that demonstrates creating a SimulateRequest object
 * and encoding it to base64 using the js-algorand-sdk
 */

async function generateSimulateRequestBase64() {
  try {
    // Create a dummy account for demonstration
    const account =  algosdk.mnemonicToSecretKey("fire erode idle bleak sense tape crop gossip quick extra please accuse trend harbor curve elephant list hammer fury energy force bike immune ability leisure");
    const receiverAccount = algosdk.mnemonicToSecretKey("fire erode idle bleak sense tape crop gossip quick extra please accuse trend harbor curve elephant list hammer fury energy force bike immune ability leisure");

    console.log('Generated accounts:');
    console.log(`Sender: ${account.addr}`);
    console.log(`Receiver: ${receiverAccount.addr}`);
    console.log('');

    // Create suggested transaction parameters (using dummy values for demonstration)
    const suggestedParams: algosdk.SuggestedParams = {
      minFee: 1_000_000,
      fee: 1_000_000,
      firstValid: 1000,
      lastValid: 2000,
      genesisHash: new Uint8Array(32),
      genesisID: 'testnet-v1.0',
      flatFee: true,
    };

    // Create a simple payment transaction
    const paymentTxn = algosdk.makePaymentTxnWithSuggestedParamsFromObject({
      sender: account.addr,
      receiver: receiverAccount.addr,
      amount: 1000000, // 1 ALGO in microAlgos
      suggestedParams,
      note: new Uint8Array(Buffer.from('Hello Algorand!')),
    });

    console.log('Created payment transaction:');
    console.log(`Amount: 1 ALGO`);
    console.log(`Note: Hello Algorand!`);
    console.log('');

    // Create a signed transaction (for simulation, we can use unsigned)
    // For simulation purposes, we can create an unsigned transaction
    const signedTxn = new algosdk.SignedTransaction({
      txn: paymentTxn,
      sig: new Uint8Array(64), // Empty signature for simulation
    });

    console.log('Signed transaction:');
    console.log(`Signed transaction msgpack: ${algosdk.bytesToBase64(algosdk.encodeMsgpack(signedTxn))}`);
    console.log('');

    // Create SimulateRequestTransactionGroup
    const txnGroup = new algosdk.modelsv2.SimulateRequestTransactionGroup({
      txns: [signedTxn],
    });

    // Create SimulateRequest object
    const simulateRequest = new algosdk.modelsv2.SimulateRequest({
      txnGroups: [txnGroup],
      allowEmptySignatures: true, // Allow unsigned transactions for simulation
      allowMoreLogging: true,
      allowUnnamedResources: true,
      execTraceConfig: new algosdk.modelsv2.SimulateTraceConfig({
        enable: true,
        scratchChange: true,
        stackChange: true,
        stateChange: true,
      }),
      extraOpcodeBudget: 1000000,
      fixSigners: true,
      round: 1000000,
    });

    console.log('Created SimulateRequest with options:');
    console.log('- allowEmptySignatures: true');
    console.log('- allowMoreLogging: true');
    console.log('- allowUnnamedResources: true');
    console.log('');

    // Encode the SimulateRequest to msgpack
    console.log("json", algosdk.encodeJSON(simulateRequest));
    const msgpackEncoded = algosdk.msgpackRawEncode(simulateRequest.toEncodingData());
    console.log('Encoded SimulateRequest to msgpack');
    console.log(`Msgpack size: ${msgpackEncoded.length} bytes`);
    console.log('');

    // Convert msgpack to base64
    const base64Encoded = algosdk.bytesToBase64(msgpackEncoded);

    console.log('='.repeat(80));
    console.log('BASE64 ENCODED SIMULATEREQUEST:');
    console.log('='.repeat(80));
    console.log(base64Encoded);
    console.log('='.repeat(80));
    console.log('');
  } catch (error) {
    console.error('Error generating SimulateRequest base64:', error);
    process.exit(1);
  }
}

// Run the example
generateSimulateRequestBase64();
