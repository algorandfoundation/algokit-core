"""
Rust Template Engine for OpenAPI Client Generation

This module uses Jinja2 templates to generate Rust API client code
from parsed OpenAPI specifications.
"""

from __future__ import annotations

from collections.abc import Callable
from functools import partial
from pathlib import Path
from typing import Any

from jinja2 import Environment, FileSystemLoader, select_autoescape

from rust_oas_generator.generator.filters import FILTERS, detect_client_type
from rust_oas_generator.parser.oas_parser import (
    Operation,
    Parameter,
    ParsedSpec,
    Response,
    Schema,
    rust_type_from_openapi,
)
from rust_oas_generator.utils.string_case import (
    normalize_rust_identifier as normalize_name,
)
from rust_oas_generator.utils.string_case import (
    rust_pascal_case,
    rust_snake_case,
)

# Constants for type checking
PRIMITIVE_TYPES = frozenset(
    {
        "String",
        "str",
        "u32",
        "u64",
        "f32",
        "f64",
        "bool",
        "Vec<u8>",
        "Vec<String>",
        "Vec<u32>",
        "Vec<u64>",
        "serde_json::Value",
        "std::path::PathBuf",
        "()",
    }
)

# Types that conflict with std types and need qualification
STD_CONFLICTING_TYPES = frozenset({"Box"})


def qualify_type_name(type_name: str | None) -> str | None:
    """Qualify type names that conflict with std types."""
    if not type_name:
        return type_name

    # Extract base type from generics
    base_type = type_name
    if "<" in type_name:
        base_type = type_name.split("<")[0]

    # If the base type conflicts with std types, qualify it
    if base_type in STD_CONFLICTING_TYPES:
        if "<" in type_name:
            # Handle generic types like Vec<Box>
            generic_part = type_name[type_name.index("<") :]
            return f"crate::models::{base_type}{generic_part}"
        return f"crate::models::{type_name}"

    return type_name


RUST_KEYWORDS = frozenset(
    {
        "box",
        "type",
        "match",
        "fn",
        "let",
        "use",
        "mod",
        "struct",
        "enum",
        "impl",
        "trait",
        "true",
        "false",
        "if",
        "else",
        "while",
        "for",
        "loop",
        "break",
        "continue",
        "return",
    }
)


class ParameterEnumAnalyzer:
    """Analyzes parameters to collect enum definitions."""

    @staticmethod
    def collect_parameter_enums(operations: list[Operation]) -> dict[str, dict[str, Any]]:
        """Collect all unique parameter enums from operations."""
        enums = {}

        for operation in operations:
            for param in operation.parameters:
                if param.is_enum_parameter:
                    enum_name = param.rust_enum_type
                    if enum_name and enum_name not in enums:
                        enums[enum_name] = {
                            "enum_values": param.enum_values,
                            "description": param.description,
                            "parameter_name": param.name,
                        }

        return enums


class OperationAnalyzer:
    """Analyzes operations for parameters, types, and responses."""

    @staticmethod
    def get_unique_tags(operations: list[Operation]) -> list[str]:
        """Get unique tags from operations."""
        tags = {tag for op in operations for tag in op.tags}
        return sorted(tags)

    @staticmethod
    def group_operations_by_tag(operations: list[Operation]) -> dict[str, list[Operation]]:
        """Group operations by their first tag."""
        groups: dict[str, list[Operation]] = {}
        for operation in operations:
            tag = operation.tags[0] if operation.tags else "default"
            groups.setdefault(tag, []).append(operation)
        return groups

    @staticmethod
    def get_parameters_by_type(operation: Operation, param_type: str) -> list[Parameter]:
        """Get parameters of specific type for an operation."""
        return [p for p in operation.parameters if p.param_type == param_type]

    @staticmethod
    def has_parameter_type(operation: Operation, param_type: str) -> bool:
        """Check if operation has parameters of given type."""
        return any(p.param_type == param_type for p in operation.parameters)

    @staticmethod
    def get_request_body_type(operation: Operation) -> str | None:
        """Get the request body type for an operation."""
        if not operation.request_body:
            return None

        content = operation.request_body.get("content", {})
        if not content:
            return None

        first_content_type = next(iter(content.keys()))
        schema = content[first_content_type].get("schema", {})

        if "$ref" in schema:
            ref_name = schema["$ref"].split("/")[-1]
            type_name = rust_pascal_case(ref_name)
            return qualify_type_name(type_name)

        return rust_type_from_openapi(schema, {})


class ResponseAnalyzer:
    """Analyzes responses for types and error handling."""

    @staticmethod
    def is_error_status(status_code: str) -> bool:
        """Check if status code represents an error."""
        return status_code.startswith(("4", "5")) or status_code == "default"

    @staticmethod
    def get_success_response_type(operation: Operation) -> str | None:
        """Get the success response type for an operation."""
        for status_code, response in operation.responses.items():
            if status_code.startswith("2"):
                return qualify_type_name(response.rust_type)
        return None

    @staticmethod
    def get_error_types(operation: Operation) -> list[str]:
        """Get error response types for an operation."""
        error_types = []
        for status_code, response in operation.responses.items():
            if ResponseAnalyzer.is_error_status(status_code):
                qualified_type = qualify_type_name(response.rust_type)
                error_type = f"Status{status_code}({qualified_type})" if qualified_type else f"Status{status_code}()"
                error_types.append(error_type)

        if not any("DefaultResponse" in t for t in error_types):
            error_types.append("DefaultResponse()")
        error_types.append("UnknownValue(serde_json::Value)")

        return error_types

    @staticmethod
    def get_response_types_by_filter(
        operations: list[Operation],
        filter_func: Callable[[str, Response], bool],
    ) -> list[str]:
        """Get response types filtered by a condition."""
        response_types: set[str] = set()
        for operation in operations:
            for status_code, response in operation.responses.items():
                if response.rust_type and filter_func(status_code, response):
                    response_types.add(response.rust_type)
        return sorted(response_types)

    @classmethod
    def get_all_response_types(cls, operations: list[Operation]) -> list[str]:
        """Get all unique response types used across operations."""

        def is_success_response(status_code: str, response: Response) -> bool:
            return (
                status_code.startswith("2")
                and response.rust_type is not None
                and response.rust_type.endswith("Response")
            )

        return cls.get_response_types_by_filter(operations, is_success_response)


class TypeAnalyzer:
    """Analyzes types for imports and dependencies."""

    @staticmethod
    def extract_base_type(type_str: str) -> str:
        """Extract base type from Vec<Type> or Option<Type>."""
        if type_str.startswith("Vec<") and type_str.endswith(">"):
            return type_str[4:-1]
        if type_str.startswith("Option<") and type_str.endswith(">"):
            return type_str[7:-1]
        return type_str

    @classmethod
    def should_import_request_body_type(cls, request_body_type: str) -> bool:
        """Check if a request body type is a custom model that needs to be imported."""
        if not request_body_type or request_body_type in PRIMITIVE_TYPES or "<" in request_body_type:
            return False
        return request_body_type[0].isupper() and request_body_type.isalnum()

    @classmethod
    def collect_types_from_responses(cls, operation: Operation, used_types: set[str]) -> None:
        """Collect types from operation responses."""
        for _status_code, response in operation.responses.items():
            if response.rust_type:
                base_type = cls.extract_base_type(response.rust_type)
                if base_type not in PRIMITIVE_TYPES:
                    used_types.add(base_type)

    @classmethod
    def collect_types_from_parameters(cls, operation: Operation, used_types: set[str]) -> None:
        """Collect types from operation parameters."""
        for param in operation.parameters:
            base_type = cls.extract_base_type(param.rust_type)
            if base_type not in PRIMITIVE_TYPES:
                used_types.add(base_type)

    @classmethod
    def get_all_used_types(cls, operations: list[Operation]) -> list[str]:
        """Get all unique custom types used across operations for imports."""
        used_types: set[str] = set()
        for operation in operations:
            cls.collect_types_from_responses(operation, used_types)
            cls.collect_types_from_parameters(operation, used_types)
        return sorted(used_types)

    @classmethod
    def get_operation_used_types(cls, operation: Operation) -> list[str]:
        """Get all unique custom types used by a single operation for imports."""
        used_types: set[str] = set()
        cls.collect_types_from_responses(operation, used_types)
        cls.collect_types_from_parameters(operation, used_types)
        return sorted(used_types)


class RustTemplateEngine:
    """Template engine for generating Rust code."""

    def __init__(self, template_dir: Path | None = None) -> None:
        """Initialize the template engine."""
        if template_dir is None:
            current_dir = Path(__file__).parent
            template_dir = current_dir.parent / "templates"

        self.template_dir = Path(template_dir)
        self.env = Environment(
            loader=FileSystemLoader(str(self.template_dir)),
            autoescape=select_autoescape(["html", "xml"]),
            trim_blocks=True,
            lstrip_blocks=True,
        )

        self._register_filters()
        self._register_globals()

    def _register_filters(self) -> None:
        """Register custom Jinja2 filters for Rust code generation."""
        # Built-in filters
        builtin_filters = {
            "snake_case": rust_snake_case,
            "pascal_case": rust_pascal_case,
            "normalize_name": normalize_name,
            "rust_type": lambda schema, schemas: rust_type_from_openapi(schema, schemas),
            "rust_doc_comment": self._rust_doc_comment,
            "rust_string_literal": self._rust_string_literal,
            "rust_optional": self._rust_optional,
            "rust_vec": self._rust_vec,
        }

        # Register all filters
        self.env.filters.update(builtin_filters)
        self.env.filters.update(FILTERS)

    def _register_globals(self) -> None:
        """Register global functions available in templates."""
        # Create analyzers
        param_enum_analyzer = ParameterEnumAnalyzer()
        op_analyzer = OperationAnalyzer()
        resp_analyzer = ResponseAnalyzer()
        type_analyzer = TypeAnalyzer()

        globals_map: dict[str, Any] = {
            # Parameter enum analysis
            "collect_parameter_enums": param_enum_analyzer.collect_parameter_enums,
            # Operation analysis
            "get_unique_tags": op_analyzer.get_unique_tags,
            "group_operations_by_tag": op_analyzer.group_operations_by_tag,
            # Response analysis
            "get_error_types": resp_analyzer.get_error_types,
            "get_success_response_type": resp_analyzer.get_success_response_type,
            "get_all_response_types": resp_analyzer.get_all_response_types,
            "get_endpoint_response_types": lambda op: resp_analyzer.get_all_response_types([op]),
            # Type analysis
            "get_all_used_types": type_analyzer.get_all_used_types,
            "get_operation_used_types": type_analyzer.get_operation_used_types,
            # Parameter-related functions
            "has_format_parameter": lambda op: any(param.name == "format" for param in op.parameters),
            "has_path_parameters": partial(op_analyzer.has_parameter_type, param_type="path"),
            "has_query_parameters": partial(op_analyzer.has_parameter_type, param_type="query"),
            "has_header_parameters": partial(op_analyzer.has_parameter_type, param_type="header"),
            "get_path_parameters": partial(op_analyzer.get_parameters_by_type, param_type="path"),
            "get_query_parameters": partial(op_analyzer.get_parameters_by_type, param_type="query"),
            "get_header_parameters": partial(op_analyzer.get_parameters_by_type, param_type="header"),
            # Request body functions
            "has_request_body": lambda op: op.request_body is not None,
            "get_request_body_type": op_analyzer.get_request_body_type,
            "get_request_body_name": lambda op: "request" if op.request_body else None,
            "is_request_body_required": lambda op: bool(op.request_body and op.request_body.get("required", False)),
            "should_import_request_body_type": type_analyzer.should_import_request_body_type,
            # Client type detection
            "get_client_type": lambda spec: self._detect_client_type_from_spec(spec),
        }

        self.env.globals.update(globals_map)

    def render_template(self, template_name: str, context: dict[str, Any]) -> str:
        """Render a template with the given context."""
        template = self.env.get_template(template_name)
        return template.render(**context)

    @staticmethod
    def _rust_doc_comment(text: str, indent: int = 0) -> str:
        """Format text as Rust doc comment."""
        if not text:
            return ""

        lines = text.strip().split("\n")
        prefix = " " * indent + "/// "
        return "\n".join(prefix + line.strip() for line in lines)

    @staticmethod
    def _rust_string_literal(text: str) -> str:
        """Format text as Rust string literal."""
        escaped = text.replace("\\", "\\\\").replace('"', '\\"')
        return f'"{escaped}"'

    @staticmethod
    def _rust_optional(rust_type: str) -> str:
        """Wrap Rust type in Option if not already optional."""
        return rust_type if rust_type.startswith("Option<") else f"Option<{rust_type}>"

    @staticmethod
    def _rust_vec(rust_type: str) -> str:
        """Wrap Rust type in Vec."""
        return f"Vec<{rust_type}>"

    def _detect_client_type_from_spec(self, spec: ParsedSpec | dict[str, Any]) -> str:
        """Detect client type from the OpenAPI specification.

        Args:
            spec: The parsed OpenAPI specification.

        Returns:
            The appropriate client type string (e.g., "Algod", "Indexer").
        """
        title = spec.info.get("title", "") if hasattr(spec, "info") else ""
        return detect_client_type(title)


class RustCodeGenerator:
    """Main code generator for Rust clients."""

    def __init__(self, template_engine: RustTemplateEngine | None = None) -> None:
        """Initialize the code generator."""
        self.template_engine = template_engine or RustTemplateEngine()

    def generate_client(
        self,
        spec: ParsedSpec,
        output_dir: Path,
        package_name: str = "api_client",
        custom_description: str | None = None,
    ) -> dict[Path, str]:
        """Generate complete Rust client from OpenAPI spec."""
        output_dir = Path(output_dir)
        context = {
            "spec": spec,
            "package_name": package_name,
            "operations": spec.operations,
            "schemas": spec.schemas,
            "content_types": spec.content_types,
            "custom_description": custom_description,
        }

        files = {}
        files.update(self._generate_base_files(context, output_dir))
        files.update(self._generate_model_files(spec.schemas, context, output_dir))
        files.update(self._generate_parameter_enums(spec.operations, context, output_dir))
        files.update(self._generate_api_files(spec.operations, context, output_dir))
        files.update(self._generate_project_files(context, output_dir))

        return files

    def _generate_base_files(self, context: dict[str, Any], output_dir: Path) -> dict[Path, str]:
        """Generate base library files."""
        src_dir = output_dir / "src"
        return {
            src_dir / "lib.rs": self.template_engine.render_template("base/lib.rs.j2", context),
        }

    def _generate_model_files(
        self,
        schemas: dict[str, Schema],
        context: dict[str, Any],
        output_dir: Path,
    ) -> dict[Path, str]:
        """Generate model files."""
        files = {}
        models_dir = output_dir / "src" / "models"

        for _, schema in schemas.items():
            model_context = {**context, "schema": schema}
            content = self.template_engine.render_template("models/model.rs.j2", model_context)

            filename = f"{schema.rust_file_name}.rs"
            files[models_dir / filename] = content

        models_context = {**context, "schemas": schemas}
        files[models_dir / "mod.rs"] = self.template_engine.render_template("models/mod.rs.j2", models_context)

        return files

    def _generate_parameter_enums(
        self,
        operations: list[Operation],
        context: dict[str, Any],
        output_dir: Path,
    ) -> dict[Path, str]:
        """Generate parameter enum files."""
        files = {}
        param_enums = ParameterEnumAnalyzer.collect_parameter_enums(operations)

        if param_enums:
            apis_dir = output_dir / "src" / "apis"
            enum_context = {**context, "parameter_enums": param_enums}
            content = self.template_engine.render_template("apis/parameter_enums.rs.j2", enum_context)
            files[apis_dir / "parameter_enums.rs"] = content

        return files

    def _generate_api_files(
        self,
        operations: list[Operation],
        context: dict[str, Any],
        output_dir: Path,
    ) -> dict[Path, str]:
        """Generate individual API files per endpoint."""
        files = {}
        apis_dir = output_dir / "src" / "apis"

        for operation in operations:
            endpoint_context = {**context, "operation": operation}
            content = self.template_engine.render_template("apis/endpoint.rs.j2", endpoint_context)
            files[apis_dir / f"{operation.rust_function_name}.rs"] = content

        client_context = {**context, "operations": operations}
        files[apis_dir / "client.rs"] = self.template_engine.render_template("apis/client.rs.j2", client_context)

        api_context = {**context, "operations": operations}
        files[apis_dir / "mod.rs"] = self.template_engine.render_template("apis/mod.rs.j2", api_context)

        return files

    def _generate_project_files(self, context: dict[str, Any], output_dir: Path) -> dict[Path, str]:
        """Generate project configuration files."""
        return {
            output_dir / "Cargo.toml": self.template_engine.render_template("base/Cargo.toml.j2", context),
            output_dir / "README.md": self.template_engine.render_template("base/README.md.j2", context),
        }
