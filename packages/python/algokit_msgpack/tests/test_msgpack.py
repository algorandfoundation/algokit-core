"""Tests for JSON serialization and deserialization of MessagePack models."""

import json
import pytest
from algokit_msgpack import (
    SimulateRequest,
    SimulateRequestTransactionGroup,
    SimulateTraceConfig,
    Account,
    encode_simulate_request,
    simulate_request_to_json,
    simulate_request_from_json,
    account_to_json,
    account_from_json,
)


class TestJSONSerialization:
    """Test JSON serialization and deserialization functionality."""

    def setup_method(self):
        """Set up test data based on lib.rs simulate test."""
        # Base64 encoded signed transaction from lib.rs test
        self.encoded_signed_txn = "gqNzaWfEQOeqkYx2i+fq1t7p5y7Epr3BRDZ7yfAcGY0B8QgdfmOWN2TYWkverOJYdf+h+2Xp/R7tQqPmjI/2vzWYbhZbSQajdHhuiaNhbXTOAA9CQKNmZWXOAA9CQKJmds0D6KNnZW6sdGVzdG5ldC12MS4womx2zQfQpG5vdGXED0hlbGxvIEFsZ29yYW5kIaNyY3bEIItZIVfmJZKkg9R82h4zw5tbm7SUTdiLUiv0hOddwUnPo3NuZMQgi1khV+YlkqSD1HzaHjPDm1ubtJRN2ItSK/SE513BSc+kdHlwZaNwYXk="

        # Create transaction group
        self.txns = [self.encoded_signed_txn]
        self.group = SimulateRequestTransactionGroup(txns=self.txns)

        # Create exec trace config
        self.exec_trace_config = SimulateTraceConfig(
            enable=True,
            stack_change=True,
            scratch_change=True,
            state_change=True
        )

        # Create simulate request
        self.simulate_request = SimulateRequest(
            txn_groups=[self.group],
            round=1_000_000,
            allow_empty_signatures=True,
            allow_more_logging=True,
            allow_unnamed_resources=True,
            extra_opcode_budget=1_000_000,
            exec_trace_config=self.exec_trace_config,
        )

    def test_simulate_request_to_json(self):
        """Test converting SimulateRequest to JSON string."""
        json_str = simulate_request_to_json(self.simulate_request)

        # Verify it's valid JSON
        parsed = json.loads(json_str)

        # Verify key fields are present
        assert "txn-groups" in parsed
        assert "round" in parsed
        assert "allow-empty-signatures" in parsed
        assert "allow-more-logging" in parsed
        assert "allow-unnamed-resources" in parsed
        assert "extra-opcode-budget" in parsed
        assert "exec-trace-config" in parsed

        # Verify values
        assert parsed["round"] == 1_000_000
        assert parsed["allow-empty-signatures"] is True
        assert parsed["allow-more-logging"] is True
        assert parsed["allow-unnamed-resources"] is True
        assert parsed["extra-opcode-budget"] == 1_000_000

    def test_simulate_request_from_json(self):
        """Test creating SimulateRequest from JSON string."""
        # First convert to JSON
        json_str = simulate_request_to_json(self.simulate_request)

        # Then convert back from JSON
        restored_request = simulate_request_from_json(json_str)

        # Verify the restored object has the same properties
        restored_json_str = simulate_request_to_json(restored_request)
        original_parsed = json.loads(json_str)
        restored_parsed = json.loads(restored_json_str)

        assert original_parsed == restored_parsed

    def test_simulate_request_json_round_trip(self):
        """Test complete round-trip: SimulateRequest -> JSON -> SimulateRequest -> JSON."""
        # Original -> JSON
        json_str1 = simulate_request_to_json(self.simulate_request)

        # JSON -> SimulateRequest
        restored_request = simulate_request_from_json(json_str1)

        # SimulateRequest -> JSON again
        json_str2 = simulate_request_to_json(restored_request)

        # Verify both JSON strings are identical
        assert json.loads(json_str1) == json.loads(json_str2)

    def test_account_json_serialization(self):
        """Test Account JSON serialization."""
        # Create test account
        account = Account(
            address="AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            amount=1000000,
            min_balance=100000,
            amount_without_pending_rewards=900000,
            total_apps_opted_in=0,
            total_assets_opted_in=0,
            total_created_apps=0,
            total_created_assets=0,
            pending_rewards=0,
            rewards=0,
            round=1000,
            status="Online"
        )

        # Test JSON serialization
        json_str = account_to_json(account)
        parsed = json.loads(json_str)

        # Verify key fields
        assert parsed["address"] == "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        assert parsed["amount"] == 1000000
        assert parsed["min-balance"] == 100000
        assert parsed["status"] == "Online"

        # Test round-trip
        restored_account = account_from_json(json_str)
        restored_json_str = account_to_json(restored_account)

        assert json.loads(json_str) == json.loads(restored_json_str)

    def test_json_modification_workflow(self):
        """Test realistic workflow: decode msgpack -> modify JSON -> encode back."""
        # Convert to JSON
        json_str = simulate_request_to_json(self.simulate_request)
        parsed = json.loads(json_str)

        # Modify like a real application might
        parsed["round"] = 2_000_000
        parsed["allow-empty-signatures"] = False
        parsed["extra-opcode-budget"] = 2_000_000

        # Add custom processing field (will be ignored by model)
        parsed["_processed_at"] = "2024-01-01T00:00:00Z"

        # Convert back (custom fields ignored)
        clean_data = {k: v for k, v in parsed.items() if not k.startswith("_")}
        modified_json_str = json.dumps(clean_data)
        modified_request = simulate_request_from_json(modified_json_str)

        # Verify modifications
        final_json_str = simulate_request_to_json(modified_request)
        final_parsed = json.loads(final_json_str)

        assert final_parsed["round"] == 2_000_000
        assert final_parsed["allow-empty-signatures"] is False
        assert final_parsed["extra-opcode-budget"] == 2_000_000

    def test_account_dict_manipulation(self):
        """Test working with Account as Python dictionary."""
        account = Account(
            address="TEST123456789012345678901234567890123456789012345678",
            amount=5000000,
            min_balance=200000,
            amount_without_pending_rewards=4800000,
            total_apps_opted_in=2,
            total_assets_opted_in=3,
            total_created_apps=1,
            total_created_assets=1,
            pending_rewards=50000,
            rewards=100000,
            round=5000,
            status="Online"
        )

        # Convert to dict for manipulation
        json_str = account_to_json(account)
        account_dict = json.loads(json_str)

        # Add computed fields
        account_dict["available_balance"] = account_dict["amount"] - account_dict["min-balance"]
        account_dict["total_balance"] = account_dict["amount"] + account_dict["pending-rewards"]

        # Verify computations
        assert account_dict["available_balance"] == 4800000
        assert account_dict["total_balance"] == 5050000

        # Modify and convert back (removing computed fields)
        account_dict["amount"] = 6000000
        clean_dict = {k: v for k, v in account_dict.items()
                     if k in ["address", "amount", "min-balance", "amount-without-pending-rewards",
                             "total-apps-opted-in", "total-assets-opted-in", "total-created-apps",
                             "total-created-assets", "pending-rewards", "rewards", "round", "status"]}

        modified_account = account_from_json(json.dumps(clean_dict))
        final_json = account_to_json(modified_account)
        final_dict = json.loads(final_json)

        assert final_dict["amount"] == 6000000

    def test_error_handling(self):
        """Test error handling for invalid JSON."""
        # Test invalid JSON syntax
        with pytest.raises(Exception):  # Will be MsgpackError when properly bound
            simulate_request_from_json("invalid json {")

        # Test empty JSON
        with pytest.raises(Exception):
            simulate_request_from_json("{}")

        # Test wrong type
        with pytest.raises(Exception):
            simulate_request_from_json("[]")

    def test_msgpack_and_json_consistency(self):
        """Test that msgpack encoding and JSON work consistently."""
        # Encode to msgpack
        msgpack_bytes = encode_simulate_request(self.simulate_request)
        assert isinstance(msgpack_bytes, bytes)
        assert len(msgpack_bytes) > 0

        # Convert same object to JSON
        json_str = simulate_request_to_json(self.simulate_request)
        parsed = json.loads(json_str)

        # Verify both contain the same semantic data
        assert "txn-groups" in parsed
        assert len(parsed["txn-groups"]) == 1
        assert parsed["round"] == 1_000_000

        # Verify JSON round-trip works
        restored = simulate_request_from_json(json_str)
        json_str2 = simulate_request_to_json(restored)
        assert json.loads(json_str) == json.loads(json_str2)

