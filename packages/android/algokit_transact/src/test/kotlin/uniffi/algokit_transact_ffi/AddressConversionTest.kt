package uniffi.algokit_transact_ffi

import org.junit.Test
import org.junit.Assert.*
import org.junit.Before

/**
 * Test suite for address conversion functions.
 *
 * Tests the bidirectional conversion between Algorand addresses and public keys.
 */
class AddressConversionTest {

    @Before
    fun setup() {
        // Ensure the FFI library is initialized before running tests
        uniffiEnsureInitialized()
    }

    @Test
    fun testPublicKeyToAddressConversion() {
        // Test converting a public key to an address
        // This is a valid 32-byte public key (all zeros for simplicity)
        val publicKey = ByteArray(32) { 0 }

        val address = addressFromPublicKey(publicKey)

        // Algorand addresses are 58 characters long (base32 encoded)
        assertEquals(58, address.length)
        // The address for a zero public key should be a specific value
        assertEquals("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ", address)
    }

    @Test
    fun testAddressToPublicKeyConversion() {
        // Test converting an address back to a public key
        val address = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ"

        val publicKey = publicKeyFromAddress(address)

        // Public keys are 32 bytes
        assertEquals(32, publicKey.size)
        // Should be all zeros
        assertTrue(publicKey.all { it == 0.toByte() })
    }

    @Test
    fun testRoundTripConversion() {
        // Test that converting public key -> address -> public key preserves the original value
        val originalPublicKey = ByteArray(32) { it.toByte() }

        val address = addressFromPublicKey(originalPublicKey)
        val recoveredPublicKey = publicKeyFromAddress(address)

        assertArrayEquals(originalPublicKey, recoveredPublicKey)
    }

    @Test
    fun testRealWorldAddress() {
        // Test with a real Algorand address
        val realAddress = "7ZUECA7HFLZTXENRV24SHLU4AVPUTMTTDUFUBNBD64C73F3UHRTHAIOF6Q"

        val publicKey = publicKeyFromAddress(realAddress)
        val recoveredAddress = addressFromPublicKey(publicKey)

        assertEquals(realAddress, recoveredAddress)
        assertEquals(32, publicKey.size)
    }

    @Test(expected = AlgoKitTransactException.DecodingException::class)
    fun testInvalidAddressFormat() {
        // Test that an invalid address throws an exception
        val invalidAddress = "INVALID"

        publicKeyFromAddress(invalidAddress)
    }

    @Test(expected = AlgoKitTransactException.DecodingException::class)
    fun testInvalidAddressChecksum() {
        // Test that an address with an invalid checksum throws an exception
        // This address has the right length but wrong checksum
        val invalidAddress = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA5HFKQ"

        publicKeyFromAddress(invalidAddress)
    }

    @Test(expected = AlgoKitTransactException.EncodingException::class)
    fun testInvalidPublicKeyLength() {
        // Test that a public key with wrong length throws an exception
        val invalidPublicKey = ByteArray(16) { 0 } // Too short, should be 32 bytes

        addressFromPublicKey(invalidPublicKey)
    }

    @Test
    fun testPublicKeyAllOnes() {
        // Test with a public key of all 1s
        val publicKey = ByteArray(32) { 0xFF.toByte() }

        val address = addressFromPublicKey(publicKey)
        val recoveredPublicKey = publicKeyFromAddress(address)

        assertArrayEquals(publicKey, recoveredPublicKey)
        assertEquals(58, address.length)
    }

    @Test
    fun testPublicKeyPattern() {
        // Test with a patterned public key (alternating bytes)
        val publicKey = ByteArray(32) { i ->
            if (i % 2 == 0) 0xAA.toByte() else 0x55.toByte()
        }

        val address = addressFromPublicKey(publicKey)
        val recoveredPublicKey = publicKeyFromAddress(address)

        assertArrayEquals(publicKey, recoveredPublicKey)
    }
}
