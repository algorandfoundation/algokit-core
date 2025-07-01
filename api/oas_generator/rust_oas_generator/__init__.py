"""
Rust OpenAPI Client Generator

A Jinja2-based generator that produces Rust API clients from OpenAPI specifications.
Designed for maintainability, LLM-friendliness, and architectural improvements.
"""

from .generator import RustCodeGenerator, RustTemplateEngine
from .parser import OASParser, ParsedSpec

__version__ = "1.0.0"
__author__ = "OpenAPI Rust Generator"

__all__ = [
    "OASParser",
    "ParsedSpec",
    "RustCodeGenerator",
    "RustTemplateEngine",
]
