"""
OpenAPI Parser Module for Rust Client Generation

This module provides parsing capabilities for OpenAPI specifications
to extract information needed for Rust client generation.
"""

from rust_oas_generator.utils.string_case import (
    normalize_rust_identifier as normalize_name,
)
from rust_oas_generator.utils.string_case import rust_pascal_case as pascal_case
from rust_oas_generator.utils.string_case import rust_snake_case as snake_case

from .oas_parser import (
    OASParser,
    Operation,
    Parameter,
    ParsedSpec,
    Property,
    Response,
    Schema,
    rust_type_from_openapi,
)

__all__ = [
    "OASParser",
    "Operation",
    "Parameter",
    "ParsedSpec",
    "Property",
    "Response",
    "Schema",
    "normalize_name",
    "pascal_case",
    "rust_type_from_openapi",
    "snake_case",
]
