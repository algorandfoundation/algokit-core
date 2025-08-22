"""
Enhanced Jinja2 filters for Rust code generation with msgpack support.

This module provides custom Jinja2 filters specifically designed for
generating Rust code from OpenAPI specifications.
"""

from __future__ import annotations

import re
from typing import Any

# Semantic versioning constants
_MIN_SEMVER_PARTS = 2
_MAX_SEMVER_PARTS = 3
_DEFAULT_VERSION = "0.1.0"

# Documentation patterns for Rust
_DOC_BULLET_PREFIXES = frozenset({"* ", "- ", "+ "})
_DOC_INDENT_PREFIX = "///   "
_DOC_NORMAL_PREFIX = "/// "


def rust_doc_comment(text: str, indent: int = 0) -> str:
    """Convert text to Rust doc comment format.

    This function handles single-line and multi-line documentation,
    applying proper Rust doc comment formatting with intelligent
    indentation for bullet points.

    Args:
        text: The text to convert to doc comments.
        indent: Number of spaces for base indentation.

    Returns:
        Formatted Rust doc comment string.

    Example:
        >>> rust_doc_comment("This is a function")
        '/// This is a function'
        >>> rust_doc_comment("* Item 1\\n* Item 2")
        '///   * Item 1\\n///   * Item 2'
    """
    if not text:
        return ""

    lines = text.strip().split("\n")
    indent_str = " " * indent

    if len(lines) == 1:
        return f"{indent_str}{_DOC_NORMAL_PREFIX}{lines[0]}"

    result: list[str] = []
    for i, line in enumerate(lines):
        stripped_line = line.strip()

        # Check if this line is a bullet point
        is_bullet = any(stripped_line.startswith(p) for p in _DOC_BULLET_PREFIXES)

        # Check if we need a blank line before this line for proper formatting
        if (
            i > 0
            and stripped_line
            and not is_bullet
            and not result[-1].strip().endswith("///")
            and any(lines[j].strip().startswith(p) for p in _DOC_BULLET_PREFIXES for j in range(max(0, i - 3), i))
        ):
            # Add blank doc comment line before starting new paragraph after bullet points
            result.append(f"{indent_str}///")

        prefix = _DOC_INDENT_PREFIX if is_bullet else _DOC_NORMAL_PREFIX
        result.append(f"{indent_str}{prefix}{stripped_line}")

    return "\n".join(result)


def is_signed_transaction_field(vendor_extensions: dict[str, Any]) -> bool:
    """Detect if this schema represents a SignedTransaction.

    Args:
        vendor_extensions: Dictionary of vendor-specific extensions.

    Returns:
        True if this schema represents a signed transaction.
    """
    return bool(vendor_extensions.get("x-algokit-signed-txn", False))


def needs_msgpack_trait(schema: dict[str, Any]) -> bool:
    """Determine if schema needs AlgorandMsgpack trait implementation.

    Args:
        schema: The schema dictionary to check.

    Returns:
        True if the schema should implement the AlgorandMsgpack trait.
    """
    vendor_extensions = schema.get("vendor_extensions", {})
    return any("msgpack" in key.lower() for key in vendor_extensions)


def get_dependencies_for_schema(schema: dict[str, Any]) -> list[str]:
    """Get list of dependencies needed for this schema.

    Args:
        schema: The schema dictionary to analyze.

    Returns:
        List of import statements needed for this schema.
    """
    dependencies = ["use serde::{Deserialize, Serialize};"]

    if schema.get("has_msgpack_fields", False):
        dependencies.append("use serde_with::serde_as;")

    vendor_extensions = schema.get("vendor_extensions", {})
    if vendor_extensions.get("x-algokit-signed-txn"):
        dependencies.extend(
            [
                "use algokit_transact::SignedTransaction as AlgokitSignedTransaction;",
                "use algokit_transact::AlgorandMsgpack;",
            ]
        )

    return dependencies


def _parse_version_parts(version_str: str) -> list[str]:
    """Parse version string into numeric parts.

    Args:
        version_str: Version string to parse.

    Returns:
        List of numeric version parts as strings.
    """
    if not version_str:
        return []

    # Remove 'v' prefix and split by dots
    cleaned_version = version_str.lstrip("v")
    parts = [part.strip() for part in cleaned_version.split(".") if part.strip()]

    # Ensure all parts are numeric, replace invalid parts with "0"
    return [part if part.isdigit() else "0" for part in parts]


def ensure_semver(version_str: str) -> str:
    """Ensure version string is valid semantic versioning format.

    Args:
        version_str: Version string to validate and format.

    Returns:
        Valid semantic version string (e.g., "1.2.3").

    Examples:
        >>> ensure_semver("1")
        '1.0.0'
        >>> ensure_semver("1.2")
        '1.2.0'
        >>> ensure_semver("v1.2.3")
        '1.2.3'
    """
    if not version_str:
        return _DEFAULT_VERSION

    parts = _parse_version_parts(version_str)

    if not parts:
        return _DEFAULT_VERSION

    # Ensure we have exactly 3 parts (major.minor.patch)
    match len(parts):
        case 1:
            parts.extend(["0", "0"])
        case 2:
            parts.append("0")
        case n if n > _MAX_SEMVER_PARTS:
            parts = parts[:_MAX_SEMVER_PARTS]

    return ".".join(parts)


def semver_string(version: str) -> str:
    """Format version string for Cargo.toml semver compatibility.

    This is an alias for ensure_semver to maintain backward compatibility.

    Args:
        version: Version string to format.

    Returns:
        Valid semantic version string.
    """
    return ensure_semver(version)


def is_valid_rust_identifier(name: str) -> bool:
    """Check if a string is a valid Rust identifier.

    Args:
        name: String to check.

    Returns:
        True if the string is a valid Rust identifier.
    """
    return bool(name) and (name[0].isalpha() or name[0] == "_") and all(char.isalnum() or char == "_" for char in name)


def sanitize_rust_string_literal(text: str) -> str:
    """Sanitize text for use in Rust string literals.

    Args:
        text: Text to sanitize.

    Returns:
        Sanitized text safe for Rust string literals.
    """
    if not text:
        return ""

    # Escape special characters
    escape_map = {
        "\\": "\\\\",
        '"': '\\"',
        "\n": "\\n",
        "\t": "\\t",
    }

    result = text
    for char, escaped in escape_map.items():
        result = result.replace(char, escaped)

    return result


def http_method_enum(method: str) -> str:
    """Convert HTTP method string to HttpMethod enum variant.

    Args:
        method: HTTP method string (e.g., "GET", "POST").

    Returns:
        HttpMethod enum variant (e.g., "HttpMethod::Get", "HttpMethod::Post").
    """
    method_mapping = {
        "GET": "HttpMethod::Get",
        "POST": "HttpMethod::Post",
        "PUT": "HttpMethod::Put",
        "DELETE": "HttpMethod::Delete",
        "PATCH": "HttpMethod::Patch",
        "HEAD": "HttpMethod::Head",
        "OPTIONS": "HttpMethod::Options",
    }

    return method_mapping.get(method.upper(), f"HttpMethod::{method.title()}")


def detect_client_type(spec_title: str) -> str:
    """Detect the client type from OpenAPI spec title.

    Args:
        spec_title: The title field from the OpenAPI spec info section.

    Returns:
        The appropriate client type string (e.g., "Algod", "Indexer").

    Examples:
        >>> detect_client_type("Algod REST API.")
        'Algod'
        >>> detect_client_type("Indexer")
        'Indexer'
        >>> detect_client_type("Unknown API")
        'Api'
    """
    if not spec_title:
        return "Api"

    title_lower = spec_title.lower().strip()

    # Check for known API types
    if "algod" in title_lower:
        return "Algod"
    if "indexer" in title_lower:
        return "Indexer"

    # Fallback: extract first word and capitalize
    first_word = spec_title.split()[0] if spec_title.split() else "Api"
    # Clean up common suffixes/prefixes
    clean_word = first_word.strip(".,!?")
    return clean_word.title() if clean_word else "Api"


def rust_path_params(path: str) -> str:
    """Replace hyphens with underscores only within path parameter placeholders.

    This filter processes OpenAPI paths to convert parameter names from kebab-case
    to snake_case within {} placeholders, while leaving the rest of the path unchanged.

    Args:
        path: The OpenAPI path string (e.g., "/v2/accounts/{account-id}/transactions")

    Returns:
        Path with hyphens replaced by underscores only within {} placeholders.
    """
    if not path:
        return ""

    # Replace hyphens with underscores only within {} placeholders
    def replace_param(match: re.Match[str]) -> str:
        param_content = match.group(1)  # Content inside {}
        return "{" + param_content.replace("-", "_") + "}"

    return re.sub(r"\{([^}]+)\}", replace_param, path)


# Register filters that will be available in Jinja templates
FILTERS = {
    "rust_doc_comment": rust_doc_comment,
    "is_signed_transaction_field": is_signed_transaction_field,
    "needs_msgpack_trait": needs_msgpack_trait,
    "get_dependencies_for_schema": get_dependencies_for_schema,
    "ensure_semver": ensure_semver,
    "semver_string": semver_string,
    "is_valid_rust_identifier": is_valid_rust_identifier,
    "sanitize_rust_string_literal": sanitize_rust_string_literal,
    "http_method_enum": http_method_enum,
    "detect_client_type": detect_client_type,
    "rust_path_params": rust_path_params,
}
