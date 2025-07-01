"""
String case conversion utilities for Rust client generation.

This module provides comprehensive string case conversion utilities with
specific support for Rust naming conventions and keyword handling.

Based on https://github.com/okunishinishi/python-stringcase
with additional Rust-specific naming conventions.
"""

import re
from collections.abc import Callable
from typing import Final

# Regex patterns for case conversion
_SNAKE_CASE_DELIMITER_PATTERN: Final = re.compile(r"[\-\.\s]")
_ACRONYM_PATTERN: Final = re.compile(r"([A-Z])([A-Z][a-z])")
_LOWER_UPPER_PATTERN: Final = re.compile(r"([a-z0-9])([A-Z])")
_NON_ALPHANUMERIC_PATTERN: Final = re.compile(r"[^a-zA-Z0-9_]")
_NON_WORD_PATTERN: Final = re.compile(r"[^\w\-]")

# Reserved Rust keywords that need to be escaped with r#
RUST_KEYWORDS: Final = frozenset(
    {
        # Strict keywords (cannot be used as identifiers)
        "as",
        "break",
        "const",
        "continue",
        "crate",
        "else",
        "enum",
        "extern",
        "false",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "match",
        "mod",
        "move",
        "mut",
        "pub",
        "ref",
        "return",
        "self",
        "Self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
        # Weak keywords (context-dependent, but safer to escape)
        "async",
        "await",
        "dyn",
        "union",
        "try",
        # Reserved keywords (not yet used but reserved for future use)
        "abstract",
        "become",
        "box",
        "do",
        "final",
        "macro",
        "override",
        "priv",
        "typeof",
        "unsized",
        "virtual",
        "yield",
        # Special identifiers
        "'static",
    }
)


def _convert_if_not_empty(string: str | None, conversion_func: Callable[[str], str]) -> str:
    """Safely convert a string, returning empty string if input is None or empty."""
    return conversion_func(string) if string else ""


def snakecase(string: str | None) -> str:
    """Convert string into snake_case.

    Handles various formats including camelCase with acronyms.

    Args:
        string: String to convert.

    Returns:
        Snake case string.

    Examples:
        >>> snakecase("HelloWorld")
        'hello_world'
        >>> snakecase("hello-world")
        'hello_world'
        >>> snakecase("getHTTPResponse")
        'get_http_response'
    """

    def _snakecase(s: str) -> str:
        s = _SNAKE_CASE_DELIMITER_PATTERN.sub("_", s)
        s = _ACRONYM_PATTERN.sub(r"\1_\2", s)
        s = _LOWER_UPPER_PATTERN.sub(r"\1_\2", s)
        return s.lower()

    return _convert_if_not_empty(string, _snakecase)


def camelcase(string: str | None) -> str:
    """Convert string into camel case.

    Args:
        string: String to convert.

    Returns:
        Camel case string.

    Examples:
        >>> camelcase("hello_world")
        'helloWorld'
        >>> camelcase("hello-world")
        'helloWorld'
        >>> camelcase("getHTTPResponse")
        'getHttpResponse'
    """

    def _camelcase(s: str) -> str:
        words = snakecase(s).split("_")
        if not words:
            return ""
        return words[0] + "".join(word.capitalize() for word in words[1:])

    return _convert_if_not_empty(string, _camelcase)


def capitalcase(string: str | None) -> str:
    """Convert string into capital case (first letter uppercase).

    Args:
        string: String to convert.

    Returns:
        Capital case string.

    Examples:
        >>> capitalcase("hello world")
        'Hello world'
    """

    def _capitalcase(s: str) -> str:
        return s[0].upper() + s[1:]

    return _convert_if_not_empty(string, _capitalcase)


def constcase(string: str | None) -> str:
    """Convert string into CONSTANT_CASE (upper snake case).

    Args:
        string: String to convert.

    Returns:
        Constant case string.

    Examples:
        >>> constcase("hello_world")
        'HELLO_WORLD'
        >>> constcase("helloWorld")
        'HELLO_WORLD'
    """
    return snakecase(string).upper()


def lowercase(string: str | None) -> str:
    """Convert string into lowercase.

    Args:
        string: String to convert.

    Returns:
        Lowercase string.
    """
    return string.lower() if string else ""


def pascalcase(string: str | None) -> str:
    """Convert string into PascalCase.

    Args:
        string: String to convert.

    Returns:
        PascalCase string.

    Examples:
        >>> pascalcase("hello_world")
        'HelloWorld'
        >>> pascalcase("hello-world")
        'HelloWorld'
        >>> pascalcase("getHTTPResponse")
        'GetHttpResponse'
    """

    def _pascalcase(s: str) -> str:
        return "".join(word.capitalize() for word in snakecase(s).split("_"))

    return _convert_if_not_empty(string, _pascalcase)


def spinalcase(string: str | None) -> str:
    """Convert string into spinal-case (kebab-case).

    Args:
        string: String to convert.

    Returns:
        Spinal case string.

    Examples:
        >>> spinalcase("hello_world")
        'hello-world'
    """
    return snakecase(string).replace("_", "-")


def titlecase(string: str | None) -> str:
    """Convert string into Title Case.

    Args:
        string: String to convert.

    Returns:
        Title case string.

    Examples:
        >>> titlecase("hello_world")
        'Hello World'
    """
    return " ".join(capitalcase(word) for word in snakecase(string).split("_") if word)


def trimcase(string: str | None) -> str:
    """Convert string into trimmed string.

    Args:
        string: String to convert.

    Returns:
        Trimmed string.
    """
    return string.strip() if string else ""


def uppercase(string: str | None) -> str:
    """Convert string into uppercase.

    Args:
        string: String to convert.

    Returns:
        Uppercase string.
    """
    return string.upper() if string else ""


def alphanumcase(string: str | None) -> str:
    """Remove all non-alphanumeric characters (keeps only 0-9, a-z, A-Z).

    Args:
        string: String to convert.

    Returns:
        String with only alphanumeric characters.

    Examples:
        >>> alphanumcase("hello@world#123")
        'helloworld123'
    """

    def _alphanumcase(s: str) -> str:
        return "".join(char for char in s if char.isalnum())

    return _convert_if_not_empty(string, _alphanumcase)


# Rust-specific naming utilities

rust_snake_case = snakecase
rust_const_case = constcase
rust_pascal_case = pascalcase


def normalize_rust_identifier(name: str | None) -> str:
    """Normalize name to be a valid Rust identifier.

    This function ensures the resulting string is a valid Rust identifier:
    - Replaces invalid characters with underscores
    - Ensures it doesn't start with a digit
    - Preserves valid alphanumeric characters and underscores

    Args:
        name: The string to normalize.

    Returns:
        A valid Rust identifier.

    Examples:
        >>> normalize_rust_identifier("123invalid")
        '_123invalid'
        >>> normalize_rust_identifier("valid@name")
        'valid_name'
    """

    def _normalize(s: str) -> str:
        # Replace invalid characters with underscores
        normalized = _NON_ALPHANUMERIC_PATTERN.sub("_", s)

        # Ensure it doesn't start with a digit
        if normalized and normalized[0].isdigit():
            normalized = f"_{normalized}"

        return normalized

    return _convert_if_not_empty(name, _normalize)


def escape_rust_keyword(name: str) -> str:
    """Escape Rust keywords with r# prefix if necessary.

    Args:
        name: The identifier name to check.

    Returns:
        The name with r# prefix if it's a Rust keyword, otherwise unchanged.

    Examples:
        >>> escape_rust_keyword("type")
        'r#type'
        >>> escape_rust_keyword("name")
        'name'
    """
    return f"r#{name}" if name in RUST_KEYWORDS else name


def is_rust_keyword(name: str) -> bool:
    """Check if a name is a Rust keyword.

    Args:
        name: The identifier name to check.

    Returns:
        True if the name is a Rust keyword, False otherwise.
    """
    return name in RUST_KEYWORDS
