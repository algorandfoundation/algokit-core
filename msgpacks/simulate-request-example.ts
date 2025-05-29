import algosdk from 'algosdk';

/**
 * Example TypeScript file that demonstrates creating a SimulateRequest object
 * and encoding it to base64 using the js-algorand-sdk
 */

async function generateSimulateRequestBase64() {
  try {
    // Create a dummy account for demonstration
    const account = algosdk.generateAccount();
    const receiverAccount = algosdk.generateAccount();

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
    });

    console.log('Created SimulateRequest with options:');
    console.log('- allowEmptySignatures: true');
    console.log('- allowMoreLogging: true');
    console.log('- allowUnnamedResources: true');
    console.log('');

    // Encode the SimulateRequest to msgpack
    const msgpackEncoded = algosdk.encodeMsgpack(simulateRequest);
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

    // Additional information
    console.log('Additional Information:');
    console.log(`Base64 length: ${base64Encoded.length} characters`);
    console.log(`Original object: SimulateRequest with 1 transaction group`);
    console.log(`Transaction type: Payment`);
    console.log('');

    // Demonstrate decoding (optional verification)
    console.log('Verification - Decoding back from base64:');
    const decodedBytes = algosdk.base64ToBytes(base64Encoded);
    const decodedRequest = algosdk.decodeMsgpack(decodedBytes, algosdk.modelsv2.SimulateRequest);
    console.log(`Decoded successfully: ${decodedRequest.txnGroups.length} transaction group(s)`);
    console.log(`Allow empty signatures: ${decodedRequest.allowEmptySignatures}`);
    console.log(`Allow more logging: ${decodedRequest.allowMoreLogging}`);

  } catch (error) {
    console.error('Error generating SimulateRequest base64:', error);
    process.exit(1);
  }
}

// Run the example
generateSimulateRequestBase64();
