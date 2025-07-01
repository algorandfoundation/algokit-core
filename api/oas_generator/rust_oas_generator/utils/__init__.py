"""
Utilities Module for Rust Client Generation

This module provides utility functions for file operations, string case conversions,
and other common tasks in the Rust client generation process.
"""

from .file_utils import clean_output_directory, ensure_directory, write_files_to_disk
from .string_case import (
    alphanumcase,
    camelcase,
    constcase,
    escape_rust_keyword,
    lowercase,
    normalize_rust_identifier,
    pascalcase,
    rust_const_case,
    rust_pascal_case,
    rust_snake_case,
    snakecase,
    spinalcase,
    titlecase,
    trimcase,
    uppercase,
)

__all__ = [
    "alphanumcase",
    "camelcase",
    "clean_output_directory",
    "constcase",
    "ensure_directory",
    "escape_rust_keyword",
    "lowercase",
    "normalize_rust_identifier",
    "pascalcase",
    "rust_const_case",
    "rust_pascal_case",
    "rust_snake_case",
    "snakecase",
    "spinalcase",
    "titlecase",
    "trimcase",
    "uppercase",
    "write_files_to_disk",
]
