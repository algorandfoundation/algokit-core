"""
Foreign trait testing for asset freeze operations.

This test demonstrates the foreign trait testing pattern where:
1. Python controls the async context (asyncio)
2. Rust orchestrates test logic
3. Python provides I/O implementations (AlgodClient, Composer, Signer)
"""

import pytest
from algokit_utils.ffi_algod_client import PythonAlgodClient
from algokit_utils.ffi_composer import PythonComposerFactory
from tests.test_utils import HttpClientImpl, MultiAccountSignerGetter
from algokit_utils.algokit_utils_ffi import AlgodClient


@pytest.mark.asyncio
async def test_asset_freeze_comprehensive():
    """Test the full async FFI pipeline for asset freeze operations"""

    # Create Python implementations of async traits using existing HttpClient
    http_client = HttpClientImpl()
    algod_client = PythonAlgodClient(http_client)

    # Create the concrete FFI components
    ffi_algod = AlgodClient(http_client)
    ffi_signer_getter = MultiAccountSignerGetter()

    # Create composer factory
    composer_factory = PythonComposerFactory(ffi_algod, ffi_signer_getter)

    # Run the async Rust test suite - dispenser mnemonic is now fetched internally
    try:
        from algokit_utils.algokit_utils_ffi import run_asset_freeze_test_suite

        # Run the async Rust test suite
        # Rust will fetch dispenser mnemonic from localnet internally
        result = await run_asset_freeze_test_suite(
            algod_client,          # PythonAlgodClient (foreign trait impl)
            composer_factory,      # PythonComposerFactory (foreign trait impl)
            ffi_signer_getter      # MultiAccountSignerGetter (foreign trait impl)
        )

        # Assert all tests passed
        assert result.all_passed, f"Test suite failed: {result.name}"

        # Print detailed results
        print(f"\n{'='*60}")
        print(f"Test Suite: {result.name}")
        print(f"Total Duration: {result.total_duration_ms}ms")
        print(f"{'='*60}")

        for test in result.results:
            status = "✓" if test.passed else "✗"
            print(f"{status} {test.name} ({test.duration_ms}ms)")
            if not test.passed and test.error:
                print(f"  Error: {test.error}")

        print(f"{'='*60}")
        print(f"Overall Result: {'PASS' if result.all_passed else 'FAIL'}")

    except ImportError:
        pytest.skip("FFI bindings need to be regenerated to include run_asset_freeze_test_suite")


if __name__ == "__main__":
    pass