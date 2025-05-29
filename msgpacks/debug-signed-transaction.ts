import algosdk from 'algosdk';

/**
 * Debug script to extract individual SignedTransaction base64 string
 */

async function debugSignedTransaction() {
  try {
    // Use the same account addresses that produced the expected result
    // We need to create deterministic accounts instead of random ones
    const senderSeed = new Uint8Array(32);
    senderSeed.fill(63); // Fill with same value to get consistent address
    const senderAccount = algosdk.mnemonicFromSeed(senderSeed);
    const sender = algosdk.mnemonicToSecretKey(senderAccount);

    const receiverSeed = new Uint8Array(32);
    receiverSeed.fill(140); // Different fill value for receiver
    const receiverAccount = algosdk.mnemonicFromSeed(receiverSeed);
    const receiver = algosdk.mnemonicToSecretKey(receiverAccount);

    console.log('Deterministic accounts:');
    console.log(`Sender: ${sender.addr}`);
    console.log(`Receiver: ${receiver.addr}`);
    console.log('');

    // Create suggested transaction parameters (exact same as before)
    const suggestedParams: algosdk.SuggestedParams = {
      fee: 1000,
      minFee: 1000,
      firstValid: 1000,
      lastValid: 2000,
      genesisHash: algosdk.base64ToBytes('wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8='),
      genesisID: 'testnet-v1.0',
      flatFee: true,
    };

    // Create a simple payment transaction
    const paymentTxn = algosdk.makePaymentTxnWithSuggestedParamsFromObject({
      sender: sender.addr,
      receiver: receiver.addr,
      amount: 1000000, // 1 ALGO in microAlgos
      suggestedParams,
      note: algosdk.encodeObj({ message: 'Hello Algorand!' }),
    });

    // Create signed transaction with empty signature
    const signedTxn = new algosdk.SignedTransaction({
      txn: paymentTxn,
      sig: new Uint8Array(64), // Empty signature for simulation
    });

    // Encode the individual SignedTransaction to msgpack
    const signedTxnMsgpack = algosdk.encodeMsgpack(signedTxn);
    const signedTxnBase64 = algosdk.bytesToBase64(signedTxnMsgpack);

    console.log('='.repeat(80));
    console.log('INDIVIDUAL SIGNEDTRANSACTION BASE64:');
    console.log('='.repeat(80));
    console.log(signedTxnBase64);
    console.log('='.repeat(80));
    console.log('');

    // Now create the SimulateRequestTransactionGroup with SignedTransaction objects
    // The TypeScript SDK uses the modelsv2 namespace
    const txnGroup = new algosdk.modelsv2.SimulateRequestTransactionGroup({
      txns: [signedTxn],
    });

    const simulateRequest = new algosdk.modelsv2.SimulateRequest({
      txnGroups: [txnGroup],
      allowEmptySignatures: true,
      allowMoreLogging: true,
      allowUnnamedResources: true,
    });

    const msgpackEncoded = algosdk.encodeMsgpack(simulateRequest);
    const base64Encoded = algosdk.bytesToBase64(msgpackEncoded);

    console.log('COMPLETE SIMULATEREQUEST BASE64:');
    console.log(base64Encoded);
    console.log('');

    console.log('SignedTransaction details:');
    console.log(`Length: ${signedTxnMsgpack.length} bytes`);
    console.log(`Base64 length: ${signedTxnBase64.length} characters`);

  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
}

// Run the debug script
debugSignedTransaction();
