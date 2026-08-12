"""
Test vendor-extension handling in the OAS parser.

Covers the extensions the Rust generator honors to stay at parity with the
utils-ts/-py generators:

- x-algorand-format: "Address"  -> algokit_transact::Address
- x-algokit-byte-length: N      -> fixed [u8; N] arrays
- x-algokit-box/holding/locals-reference -> schema-level markers
"""

from typing import Any

from rust_oas_generator.parser.oas_parser import OASParser, Schema

# Fixed byte-array lengths exercised by the byte-length tests.
BYTE_LENGTH_32 = 32
BYTE_LENGTH_64 = 64


def _spec_with_schema(name: str, schema: dict[str, Any]) -> dict[str, Any]:
    """Build a minimal OAS3 document wrapping a single component schema."""
    return {
        "openapi": "3.0.0",
        "info": {"title": "test", "version": "1.0.0"},
        "paths": {},
        "components": {"schemas": {name: schema}},
    }


def _parse_single_schema(name: str, schema: dict[str, Any]) -> Schema:
    parsed = OASParser().parse_dict(_spec_with_schema(name, schema))
    return parsed.schemas[name]


class TestAddressFormat:
    """x-algorand-format: 'Address' maps to algokit_transact::Address."""

    def test_address_property_maps_to_address_type(self) -> None:
        schema = _parse_single_schema(
            "AddrHolder",
            {
                "type": "object",
                "required": ["addr"],
                "properties": {"addr": {"type": "string", "x-algorand-format": "Address"}},
            },
        )
        prop = schema.properties[0]
        assert prop.is_address is True
        assert prop.rust_type_with_msgpack == "Address"

    def test_non_address_format_is_not_address(self) -> None:
        schema = _parse_single_schema(
            "Other",
            {
                "type": "object",
                "properties": {"prog": {"type": "string", "x-algorand-format": "TEALProgram"}},
            },
        )
        assert schema.properties[0].is_address is False

    def test_spec_flags_address_fields(self) -> None:
        parsed = OASParser().parse_dict(
            _spec_with_schema(
                "AddrHolder",
                {
                    "type": "object",
                    "properties": {"addr": {"type": "string", "x-algorand-format": "Address"}},
                },
            )
        )
        assert parsed.has_address_fields is True

    def test_spec_without_address_fields_is_flagged_false(self) -> None:
        parsed = OASParser().parse_dict(
            _spec_with_schema(
                "Plain",
                {"type": "object", "properties": {"name": {"type": "string"}}},
            )
        )
        assert parsed.has_address_fields is False


class TestByteLength:
    """x-algokit-byte-length: N emits a fixed [u8; N] array."""

    def test_byte_length_32_emits_fixed_array(self) -> None:
        schema = _parse_single_schema(
            "Hashes",
            {
                "type": "object",
                "required": ["hash"],
                "properties": {
                    "hash": {"type": "string", "format": "byte", "x-algokit-byte-length": 32}
                },
            },
        )
        prop = schema.properties[0]
        assert prop.byte_length == BYTE_LENGTH_32
        assert prop.rust_type_with_msgpack == "[u8; 32]"
        # Fixed byte arrays serialize via serde_with::Bytes, same path as Vec<u8>.
        assert prop.is_base64_encoded is True

    def test_byte_length_64_emits_fixed_array(self) -> None:
        schema = _parse_single_schema(
            "Sig",
            {
                "type": "object",
                "properties": {
                    "sig": {"type": "string", "format": "byte", "x-algokit-byte-length": 64}
                },
            },
        )
        assert schema.properties[0].rust_type_with_msgpack == "[u8; 64]"

    def test_no_byte_length_stays_vec_u8(self) -> None:
        schema = _parse_single_schema(
            "Blob",
            {
                "type": "object",
                "properties": {"data": {"type": "string", "format": "byte"}},
            },
        )
        prop = schema.properties[0]
        assert prop.byte_length is None
        assert prop.rust_type_with_msgpack == "Vec<u8>"


class TestReferenceMarkers:
    """Box/holding/locals reference extensions surface as schema vendor extensions."""

    def test_box_reference_extension_preserved(self) -> None:
        schema = _parse_single_schema(
            "ApplicationBoxReference",
            {
                "type": "object",
                "x-algokit-box-reference": True,
                "properties": {"app": {"type": "integer"}, "name": {"type": "string"}},
            },
        )
        assert schema.vendor_extensions.get("x-algokit-box-reference") is True

    def test_holding_reference_extension_preserved(self) -> None:
        schema = _parse_single_schema(
            "AssetHoldingReference",
            {
                "type": "object",
                "x-algokit-holding-reference": True,
                "properties": {"account": {"type": "string"}, "asset": {"type": "integer"}},
            },
        )
        assert schema.vendor_extensions.get("x-algokit-holding-reference") is True

    def test_locals_reference_extension_preserved(self) -> None:
        schema = _parse_single_schema(
            "ApplicationLocalReference",
            {
                "type": "object",
                "x-algokit-locals-reference": True,
                "properties": {"account": {"type": "string"}, "app": {"type": "integer"}},
            },
        )
        assert schema.vendor_extensions.get("x-algokit-locals-reference") is True
