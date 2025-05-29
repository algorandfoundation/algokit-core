import algosdk from 'algosdk';

async function decodeOriginal() {
  try {
    // Original base64 from user
    const originalBase64 = "hLZhbGxvdy1lbXB0eS1zaWduYXR1cmVzw7JhbGxvdy1tb3JlLWxvZ2dpbmfDt2FsbG93LXVubmFtZWQtcmVzb3VyY2Vzw6p0eG4tZ3JvdXBzkYGkdHhuc5GBo3R4bomjYW10zgAPQkCjZmVlzgAPQkCiZnbNA+ijZ2VurHRlc3RuZXQtdjEuMKJsds0H0KRub3RlxA9IZWxsbyBBbGdvcmFuZCGjcmN2xCCM2pMMG3b4XX/mFk1Wh4YZL5UVS8+se9j3zlmPHICncKNzbmTEID/+X9/u7HedjDKORv63/H2TyoTd3oT1HeWPEipaoEiOpHR5cGWjcGF5";

    // Decode from base64 to bytes
    const bytes = algosdk.base64ToBytes(originalBase64);

    // Decode the msgpack to get the SimulateRequest structure
    const decoded = algosdk.decodeMsgpack(bytes, algosdk.modelsv2.SimulateRequest);

    console.log('Decoded SimulateRequest:');
    console.log('- txnGroups length:', decoded.txnGroups.length);
    console.log('- allowEmptySignatures:', decoded.allowEmptySignatures);
    console.log('- allowMoreLogging:', decoded.allowMoreLogging);
    console.log('- allowUnnamedResources:', decoded.allowUnnamedResources);
    console.log('');

    if (decoded.txnGroups.length > 0) {
      const firstGroup = decoded.txnGroups[0];
      console.log('First transaction group:');
      console.log('- txns length:', firstGroup.txns.length);

      if (firstGroup.txns.length > 0) {
        const firstTxn = firstGroup.txns[0];
        console.log('First transaction:');
        console.log('- Type:', typeof firstTxn);

        if (firstTxn instanceof algosdk.SignedTransaction) {
          console.log('- Is SignedTransaction object');

          // Encode this specific SignedTransaction to base64
          const txnMsgpack = algosdk.encodeMsgpack(firstTxn);
          const txnBase64 = algosdk.bytesToBase64(txnMsgpack);

          console.log('');
          console.log('='.repeat(80));
          console.log('EXTRACTED SIGNEDTRANSACTION BASE64:');
          console.log('='.repeat(80));
          console.log(txnBase64);
          console.log('='.repeat(80));
        } else {
          console.log('- Is not SignedTransaction object');
          console.log('- Actual value:', firstTxn);
        }
      }
    }

  } catch (error) {
    console.error('Error decoding:', error);
  }
}

decodeOriginal();
