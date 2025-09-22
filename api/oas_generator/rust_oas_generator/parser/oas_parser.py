"""
OpenAPI Specification Parser for Rust Client Generation.

This module parses OpenAPI 3.x specifications and extracts information
needed to generate Rust API clients with comprehensive type mapping
and msgpack support.
"""

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Final

from rust_oas_generator.utils.string_case import (
    escape_rust_keyword,
    rust_pascal_case,
    rust_snake_case,
)

# Type mapping constants for OpenAPI to Rust conversion
_OPENAPI_TYPE_MAPPING: Final = {
    "string": {
        None: "String",
        "date": "String",
        "date-time": "String",
        "byte": "String",
        "binary": "Vec<u8>",
    },
    "integer": {
        None: "u64",
        "int32": "u32",
        "int64": "u64",
        "uint64": "u64",
    },
    "number": {
        None: "f64",
        "float": "f32",
        "double": "f64",
    },
    "boolean": {
        None: "bool",
    },
    "object": {
        None: "UnknownJsonValue",
    },
}

# Constants for integer type selection
_U32_MAX_VALUE: Final = 4294967295  # Value of u32::MAX
_SMALL_INTEGER_MAX: Final = 100  # Threshold for small bounded integers
_ENUM_KEYWORDS: Final = frozenset(
    ["value `1`", "value `2`", "value 1", "value 2", "refers to", "type.", "action.", "enum"]
)

# HTTP methods supported by OpenAPI
_HTTP_METHODS: Final = frozenset({"get", "post", "put", "delete", "patch", "head", "options"})

# Content types that indicate msgpack support
_MSGPACK_CONTENT_TYPES: Final = frozenset({"application/msgpack", "application/x-binary"})


def _select_integer_rust_type(schema: dict[str, Any]) -> str:
    """Select appropriate Rust integer type based on schema constraints.

    Args:
        schema: OpenAPI schema dictionary for an integer type.

    Returns:
        Rust type string: either "u32" or "u64".
    """
    # Check if explicit format is provided
    schema_format = schema.get("format")
    if schema_format:
        return _get_openapi_type_mapping("integer", schema_format)

    # Auto-detect u32 vs u64 based on constraints
    maximum = schema.get("maximum")
    minimum = schema.get("minimum")

    # Use u32 if maximum is within u32 range
    if maximum is not None and maximum <= _U32_MAX_VALUE:
        return "u32"

    # Use u32 for small bounded integers (common patterns)
    if minimum is not None and minimum >= 0 and maximum is not None and maximum <= _SMALL_INTEGER_MAX:
        return "u32"

    # Use u32 for enum-like descriptions (type discriminators)
    description = schema.get("description", "").lower()
    if any(keyword in description for keyword in _ENUM_KEYWORDS):
        return "u32"

    # Default to u64 for potentially large blockchain values
    return "u64"


def _extract_ref_name(ref_string: str) -> str:
    """Extract the reference name from an OpenAPI $ref string.

    Args:
        ref_string: The $ref value (e.g., "#/components/schemas/Model").

    Returns:
        The extracted reference name (e.g., "Model").
    """
    return ref_string.split("/")[-1]


def _get_openapi_type_mapping(schema_type: str, schema_format: str | None) -> str:
    """Get Rust type mapping for OpenAPI schema type and format.

    Args:
        schema_type: The OpenAPI schema type.
        schema_format: The OpenAPI schema format (optional).

    Returns:
        The corresponding Rust type.
    """
    type_formats = _OPENAPI_TYPE_MAPPING.get(schema_type, {})
    if isinstance(type_formats, dict):
        result = type_formats.get(schema_format, "String")
        return str(result) if result is not None else "String"
    return "String"


def rust_type_from_openapi(
    schema: dict[str, Any],
    schemas: dict[str, Any],
    visited: set[str] | None = None,
) -> str:
    """Convert OpenAPI schema type to Rust type string.

    Args:
        schema: The schema dictionary from OpenAPI spec.
        schemas: All available schemas for reference resolution.
        visited: Set of visited references to prevent cycles.

    Returns:
        Rust type string.
    """
    if visited is None:
        visited = set()

    # Handle x-algokit-bigint extension for u64 mapping
    if schema.get("x-algokit-bigint") is True:
        return "u64"

    # Handle references
    if "$ref" in schema:
        ref_name = _extract_ref_name(schema["$ref"])

        if ref_name in visited:
            return rust_pascal_case(ref_name)

        visited.add(ref_name)

        # Return the original name without ModelBox renaming
        return rust_pascal_case(ref_name)

    schema_type = schema.get("type", "string")

    # Handle array types
    if schema_type == "array":
        items_schema = schema.get("items", {})
        items_type = rust_type_from_openapi(items_schema, schemas, visited)
        return f"Vec<{items_type}>"

    # Smart integer type selection for non-bigint fields
    if schema_type == "integer" and not schema.get("x-algokit-bigint"):
        return _select_integer_rust_type(schema)

    # Handle primitive types
    schema_format = schema.get("format")
    return _get_openapi_type_mapping(schema_type, schema_format)


def detect_binary_field(prop_data: dict[str, Any]) -> bool:
    """Detect if a property contains binary data that needs base64 decoding.

    This function identifies fields that contain binary data (which could be
    raw bytes, cryptographic keys, program bytecode, or MessagePack data)
    that is transmitted as base64-encoded strings in JSON APIs.

    Args:
        prop_data: The property data dictionary.

    Returns:
        True if the property contains binary data that's base64-encoded in JSON.
    """
    # Check format
    if prop_data.get("format") == "byte":
        return True

    # Check description for base64 indicator
    description = prop_data.get("description", "").lower()
    if "base64" in description:
        return True

    # Check vendor extensions
    return bool(prop_data.get("x-msgpack-encoding"))


def detect_msgpack_support_for_operation(operation_data: dict[str, Any]) -> bool:
    """Detect if an operation supports msgpack content type or binary data.

    Args:
        operation_data: The operation data dictionary.

    Returns:
        True if the operation supports msgpack.
    """
    # Check request body content types
    request_body = operation_data.get("requestBody", {})
    content = request_body.get("content", {})

    if any(ct in content for ct in _MSGPACK_CONTENT_TYPES):
        return True

    # Check for binary format in request body
    if "application/x-binary" in content:
        binary_content = content["application/x-binary"]
        schema = binary_content.get("schema", {})
        if schema.get("format") == "binary":
            return True

    # Check response content types
    responses = operation_data.get("responses", {})
    for response_data in responses.values():
        response_content = response_data.get("content", {})
        if any(ct in response_content for ct in _MSGPACK_CONTENT_TYPES):
            return True

    return False


def should_implement_algokit_msgpack(
    schema_data: dict[str, Any],
    *,
    operation_msgpack_support: bool = False,
) -> bool:
    """Determine if a schema should implement AlgorandMsgpack trait.

    Args:
        schema_data: The schema data dictionary.
        operation_msgpack_support: Whether operations support msgpack.

    Returns:
        True if the schema should implement AlgorandMsgpack.
    """
    # Check schema-level vendor extensions
    if schema_data.get("x-algokit-signed-txn", False):
        return True

    # Check property-level vendor extensions
    properties = schema_data.get("properties", {})
    for prop_data in properties.values():
        if prop_data.get("x-algokit-signed-txn", False):
            return True

        # Check array items for signed transaction markers
        if prop_data.get("type") == "array":
            items = prop_data.get("items", {})
            if items.get("x-algokit-signed-txn", False):
                return True

    return operation_msgpack_support


def rust_type_with_msgpack(
    schema: dict[str, Any],
    schemas: dict[str, Any],
    visited: set[str] | None = None,
) -> str:
    """Convert OpenAPI schema type to Rust type with msgpack considerations.

    Args:
        schema: The schema dictionary.
        schemas: All available schemas.
        visited: Set of visited references.

    Returns:
        Rust type string, using Vec<u8> for msgpack fields.
    """
    return "Vec<u8>" if detect_binary_field(schema) else rust_type_from_openapi(schema, schemas, visited)


@dataclass
class Parameter:
    """Represents an OpenAPI parameter."""

    name: str
    param_type: str
    rust_type: str
    required: bool
    description: str | None = None
    enum_values: list[str] = field(default_factory=list)
    rust_name: str = field(init=False)
    rust_field_name: str = field(init=False)

    def __post_init__(self) -> None:
        self.rust_name = rust_snake_case(self.name)
        self.rust_field_name = escape_rust_keyword(self.rust_name)

    @property
    def rust_enum_type(self) -> str | None:
        """Generate Rust enum type name if this parameter has enum constraints."""
        if not self.enum_values:
            return None
        return rust_pascal_case(self.name)

    @property
    def is_enum_parameter(self) -> bool:
        """Check if this parameter should use an enum type."""
        return bool(self.enum_values)

    @property
    def is_array(self) -> bool:
        """Check if this parameter is an array type."""
        return self.rust_type.startswith("Vec<")

    @property
    def effective_rust_type(self) -> str:
        """Get the effective Rust type, using enum if available, otherwise the original rust_type."""
        if self.is_enum_parameter and self.rust_enum_type:
            return self.rust_enum_type
        return self.rust_type


@dataclass
class Response:
    """Represents an OpenAPI response."""

    status_code: str
    description: str
    rust_type: str | None = None
    content_types: list[str] = field(default_factory=list)
    supports_msgpack: bool = False


@dataclass
class Operation:
    """Represents an OpenAPI operation."""

    operation_id: str
    method: str
    path: str
    summary: str | None
    description: str | None
    parameters: list[Parameter]
    request_body: dict[str, Any] | None
    responses: dict[str, Response]
    tags: list[str]
    rust_function_name: str = field(init=False)
    rust_error_enum: str = field(init=False)
    supports_msgpack: bool = False
    request_body_supports_msgpack: bool = False
    request_body_supports_text_plain: bool = False
    has_optional_string: bool = False

    def __post_init__(self) -> None:
        for param in self.parameters:
            if not param.required and not param.is_enum_parameter and param.rust_type == "String":
                self.has_optional_string = True
                break
        self.rust_function_name = rust_snake_case(self.operation_id)
        self.rust_error_enum = f"{rust_pascal_case(self.operation_id)}Error"


@dataclass
class Property:
    """Represents a schema property."""

    name: str
    rust_type: str
    required: bool
    description: str | None = None
    is_base64_encoded: bool = False  # True if field contains binary data encoded as base64
    vendor_extensions: list[tuple[str, Any]] = field(default_factory=list)
    format: str | None = None
    items: "Property | None" = None
    rust_name: str = field(init=False)
    rust_field_name: str = field(init=False)
    rust_type_with_msgpack: str = field(init=False)
    is_msgpack_field: bool = field(init=False)
    is_signed_transaction: bool = field(init=False)

    def __post_init__(self) -> None:
        # Check for field name override from vendor extension
        field_name_override = self._get_field_name_override()
        field_name = field_name_override if field_name_override else self.name

        self.rust_name = rust_snake_case(field_name)
        self.rust_field_name = escape_rust_keyword(self.rust_name)

        # Check for bytes base64 override from vendor extension
        if self._has_bytes_base64_extension():
            self.is_base64_encoded = True

        if self.is_base64_encoded:
            self.rust_type_with_msgpack = "Vec<u8>"
        elif self.items and self.items.is_base64_encoded and self.rust_type.startswith("Vec<"):
            self.rust_type_with_msgpack = "Vec<Vec<u8>>"
        else:
            self.rust_type_with_msgpack = self.rust_type
        self.is_msgpack_field = self.is_base64_encoded

        self.is_signed_transaction = any(
            "x-algokit-signed-txn" in ext_name and ext_value for ext_name, ext_value in self.vendor_extensions
        )

        if self.items and hasattr(self.items, "vendor_extensions"):
            self.is_signed_transaction = self.is_signed_transaction or any(
                "x-algokit-signed-txn" in ext_name and ext_value for ext_name, ext_value in self.items.vendor_extensions
            )

    def _get_field_name_override(self) -> str | None:
        """Get field name override from vendor extension."""
        for ext_name, ext_value in self.vendor_extensions:
            if ext_name == "x-algokit-field-rename" and isinstance(ext_value, str):
                return ext_value
        return None

    def _has_bytes_base64_extension(self) -> bool:
        """Check if this property has the bytes base64 vendor extension."""
        for ext_name, ext_value in self.vendor_extensions:
            if ext_name == "x-algokit-bytes-base64" and ext_value is True:
                return True
        return False


@dataclass
class Schema:
    """Represents an OpenAPI schema."""

    name: str
    schema_type: str
    description: str | None
    properties: list[Property]
    required_fields: list[str]
    vendor_extensions: dict[str, Any] = field(default_factory=dict)
    rust_struct_name: str = field(init=False)
    rust_file_name: str = field(init=False)
    has_msgpack_fields: bool = field(init=False)
    has_required_fields: bool = field(init=False)
    implements_algokit_msgpack: bool = field(init=False)
    has_signed_transaction_fields: bool = field(init=False)
    # For non-object schemas (e.g. top-level arrays) we capture the underlying rust type
    underlying_rust_type: str | None = None
    # For string enum schemas
    enum_values: list[str] = field(default_factory=list)
    is_string_enum: bool = field(init=False)

    def __post_init__(self) -> None:
        # Keep the original struct name without renaming
        self.rust_struct_name = rust_pascal_case(self.name)
        # Use _model suffix for file name only when there's a conflict (like Box)
        self.rust_file_name = f"{self.name.lower()}_model" if self.name == "Box" else rust_snake_case(self.name)
        self.has_msgpack_fields = any(
            prop.is_base64_encoded or (prop.items and prop.items.is_base64_encoded) for prop in self.properties
        )
        self.has_required_fields = len(self.required_fields) > 0
        self.has_signed_transaction_fields = any(prop.is_signed_transaction for prop in self.properties)
        self.is_string_enum = self.schema_type == "string" and len(self.enum_values) > 0


@dataclass
class ParsedSpec:
    """Represents a parsed OpenAPI specification."""

    info: dict[str, Any]
    servers: list[dict[str, Any]]
    operations: list[Operation]
    schemas: dict[str, Schema]
    content_types: list[str]
    has_msgpack_operations: bool = False


class OASParser:
    """Parser for OpenAPI 3.x specifications."""

    def __init__(self) -> None:
        self.spec_data: dict[str, Any] | None = None
        self.schemas: dict[str, Any] = {}
        self.msgpack_operations: list[str] = []

    def parse_file(self, file_path: str | Path) -> ParsedSpec:
        """Parse OpenAPI specification from file."""
        path = Path(file_path)
        with path.open(encoding="utf-8") as f:
            self.spec_data = json.load(f)
        return self._parse_spec()

    def parse_dict(self, spec_dict: dict[str, Any]) -> ParsedSpec:
        """Parse OpenAPI specification from dictionary."""
        self.spec_data = spec_dict
        return self._parse_spec()

    def _parse_spec(self) -> ParsedSpec:
        """Parse the loaded specification."""
        if not self.spec_data:
            msg = "No specification data loaded"
            raise ValueError(msg)

        self.schemas = self.spec_data.get("components", {}).get("schemas", {})

        info = self.spec_data.get("info", {})
        servers = self.spec_data.get("servers", [])
        operations = self._parse_operations()
        schemas = self._parse_schemas()
        content_types = self._extract_content_types()

        has_msgpack_operations = len(self.msgpack_operations) > 0
        self._update_schemas_for_msgpack(schemas, has_msgpack_operations=has_msgpack_operations)

        return ParsedSpec(
            info=info,
            servers=servers,
            operations=operations,
            schemas=schemas,
            content_types=content_types,
            has_msgpack_operations=has_msgpack_operations,
        )

    def _update_schemas_for_msgpack(
        self,
        schemas: dict[str, Schema],
        *,
        has_msgpack_operations: bool,
    ) -> None:
        """Update schemas to implement AlgorandMsgpack trait when appropriate."""
        if not has_msgpack_operations:
            for schema_name, schema in schemas.items():
                raw_schema = self.schemas.get(schema_name, {})
                schema.implements_algokit_msgpack = should_implement_algokit_msgpack(raw_schema)
            return

        dependency_graph = self._build_schema_dependency_graph()
        root_msgpack_schemas = self._get_msgpack_root_schemas()

        all_msgpack_schemas = set()
        queue = list(root_msgpack_schemas)
        visited = set()

        while queue:
            schema_name = queue.pop(0)
            if schema_name in visited:
                continue
            visited.add(schema_name)
            all_msgpack_schemas.add(schema_name)

            if schema_name in dependency_graph:
                for dep in dependency_graph[schema_name]:
                    if dep not in visited:
                        queue.append(dep)

        for schema_name, schema in schemas.items():
            raw_schema = self.schemas.get(schema_name, {})
            is_msgpack_related = schema_name in all_msgpack_schemas
            schema.implements_algokit_msgpack = should_implement_algokit_msgpack(
                raw_schema, operation_msgpack_support=is_msgpack_related
            )

    def _get_msgpack_root_schemas(self) -> set[str]:
        """Get the root schemas that require msgpack support."""
        root_schemas = self._get_msgpack_schemas_from_operations()
        root_schemas.update(self._get_msgpack_schemas_from_extensions())

        # Also include response schemas from msgpack operations that were created during parsing
        # This covers cases where inline response schemas get converted to named schemas
        if self.msgpack_operations and self.spec_data:
            for path_item in self.spec_data.get("paths", {}).values():
                for method, op_data in path_item.items():
                    if (
                        method not in _HTTP_METHODS
                        or not detect_msgpack_support_for_operation(op_data)
                        or op_data.get("operationId") not in self.msgpack_operations
                    ):
                        continue

                    # Check response schemas for this msgpack operation
                    for resp_data in op_data.get("responses", {}).values():
                        content = resp_data.get("content", {})
                        for ct, _ in content.items():
                            if ct in _MSGPACK_CONTENT_TYPES:
                                operation_id = op_data.get("operationId")
                                if operation_id and operation_id in self.schemas:
                                    root_schemas.add(operation_id)

        return root_schemas

    def _get_msgpack_schemas_from_operations(self) -> set[str]:
        """Get schemas used in request/response bodies of msgpack-enabled operations."""
        schemas: set[str] = set()
        if not self.spec_data:
            return schemas

        for path_item in self.spec_data.get("paths", {}).values():
            for method, op_data in path_item.items():
                if method not in _HTTP_METHODS or not detect_msgpack_support_for_operation(op_data):
                    continue

                # Request body
                request_body = op_data.get("requestBody", {})
                content = request_body.get("content", {})
                for ct, cd in content.items():
                    if ct in _MSGPACK_CONTENT_TYPES and "$ref" in cd.get("schema", {}):
                        schemas.add(_extract_ref_name(cd["schema"]["$ref"]))

                # Response bodies
                for resp_data in op_data.get("responses", {}).values():
                    content = resp_data.get("content", {})
                    for ct, cd in content.items():
                        if ct in _MSGPACK_CONTENT_TYPES and "$ref" in cd.get("schema", {}):
                            schemas.add(_extract_ref_name(cd["schema"]["$ref"]))
        return schemas

    def _get_msgpack_schemas_from_extensions(self) -> set[str]:
        """Get schemas with x-algokit-signed-txn vendor extension."""
        return {
            schema_name
            for schema_name, schema_data in self.schemas.items()
            if should_implement_algokit_msgpack(schema_data)
        }

    def _build_schema_dependency_graph(self) -> dict[str, set[str]]:
        """Build a dependency graph for all schemas."""
        dependency_graph = {}
        for schema_name, schema_data in self.schemas.items():
            dependency_graph[schema_name] = self._extract_refs_from_schema_part(schema_data)
        return dependency_graph

    def _extract_refs_from_schema_part(self, schema_part: Any) -> set[str]:  # noqa: ANN401
        """Extract all schema references from a part of a schema."""
        refs = set()
        if isinstance(schema_part, dict):
            refs.update(self._extract_refs_from_dict(schema_part))
        elif isinstance(schema_part, list):
            for item in schema_part:
                refs.update(self._extract_refs_from_schema_part(item))
        return refs

    def _extract_refs_from_dict(self, schema_dict: dict[str, Any]) -> set[str]:
        """Helper to extract refs from a dictionary part of a schema."""
        refs = set()
        if "$ref" in schema_dict:
            refs.add(_extract_ref_name(schema_dict["$ref"]))

        if "properties" in schema_dict and isinstance(
            schema_dict["properties"],
            dict,
        ):
            for prop_schema in schema_dict["properties"].values():
                refs.update(self._extract_refs_from_schema_part(prop_schema))

        if "items" in schema_dict:
            refs.update(self._extract_refs_from_schema_part(schema_dict["items"]))

        for key in ("allOf", "oneOf", "anyOf"):
            if key in schema_dict and isinstance(schema_dict[key], list):
                for sub_schema in schema_dict[key]:
                    refs.update(self._extract_refs_from_schema_part(sub_schema))
        return refs

    def _parse_operations(self) -> list[Operation]:
        """Parse all operations from paths."""
        operations: list[Operation] = []
        if not self.spec_data:
            return operations
        paths = self.spec_data.get("paths", {})

        for path, path_item in paths.items():
            for method, operation_data in path_item.items():
                if method.lower() in _HTTP_METHODS:
                    operation = self._parse_operation(
                        path,
                        method.upper(),
                        operation_data,
                    )
                    if operation:
                        operations.append(operation)

                        if operation.supports_msgpack:
                            self.msgpack_operations.append(operation.operation_id)

        return operations

    def _parse_operation(
        self,
        path: str,
        method: str,
        operation_data: dict[str, Any],
    ) -> Operation | None:
        """Parse a single operation."""
        operation_id = operation_data.get("operationId")
        if not operation_id:
            return None

        supports_msgpack = detect_msgpack_support_for_operation(operation_data)
        request_body_supports_msgpack = self._check_request_body_msgpack_support(
            operation_data,
        )
        request_body_supports_text_plain = self._check_request_body_text_plain_support(
            operation_data,
        )

        parameters = []
        for param_data in operation_data.get("parameters", []):
            param = self._parse_parameter(param_data)
            if param:
                parameters.append(param)

        responses = {}
        for status_code, response_data in operation_data.get("responses", {}).items():
            response = self._parse_response(status_code, response_data, operation_id)
            responses[status_code] = response

        return Operation(
            operation_id=operation_id,
            method=method,
            path=path,
            summary=operation_data.get("summary"),
            description=operation_data.get("description"),
            parameters=parameters,
            request_body=operation_data.get("requestBody"),
            responses=responses,
            tags=operation_data.get("tags", []),
            supports_msgpack=supports_msgpack,
            request_body_supports_msgpack=request_body_supports_msgpack,
            request_body_supports_text_plain=request_body_supports_text_plain,
        )

    def _check_request_body_msgpack_support(
        self,
        operation_data: dict[str, Any],
    ) -> bool:
        """Check if request body supports msgpack or binary transmission."""
        request_body = operation_data.get("requestBody", {})
        content = request_body.get("content", {})

        if "application/msgpack" in content:
            return True

        if "application/x-binary" in content:
            binary_content = content["application/x-binary"]
            schema = binary_content.get("schema", {})
            format_value: str | None = schema.get("format")
            return format_value == "binary"

        return False

    def _check_request_body_text_plain_support(
        self,
        operation_data: dict[str, Any],
    ) -> bool:
        """Check if request body uses text/plain content type."""
        request_body = operation_data.get("requestBody", {})
        content = request_body.get("content", {})

        return "text/plain" in content

    def _parse_parameter(self, param_data: dict[str, Any]) -> Parameter | None:
        """Parse a parameter."""
        if "$ref" in param_data:
            param_data = self._resolve_reference(param_data["$ref"])

        name = param_data.get("name")
        if not name:
            return None

        schema = param_data.get("schema", {})
        rust_type = rust_type_from_openapi(schema, self.schemas, set())
        enum_values = schema.get("enum", []) if schema.get("type") == "string" else []

        return Parameter(
            name=name,
            param_type=param_data.get("in", "query"),
            rust_type=rust_type,
            required=param_data.get("required", False),
            description=param_data.get("description"),
            enum_values=enum_values,
        )

    def _resolve_reference(self, ref: str) -> dict[str, Any]:
        """Resolve a JSON reference."""
        if not self.spec_data:
            return {}

        ref_path = ref.split("/")
        resolved: dict[str, Any] | None = self.spec_data
        for part in ref_path[1:]:  # Skip '#'
            if resolved is None:
                return {}
            resolved = resolved.get(part)
        return resolved or {}

    def _parse_response(
        self,
        status_code: str,
        response_data: dict[str, Any],
        operation_id: str,
    ) -> Response:
        """Parse a response."""
        content = response_data.get("content", {})
        content_types = list(content.keys())
        supports_msgpack = "application/msgpack" in content_types

        rust_type = self._determine_response_rust_type(
            content_types,
            content,
            status_code,
            operation_id,
            response_data,
        )

        return Response(
            status_code=status_code,
            description=response_data.get("description", ""),
            rust_type=rust_type,
            content_types=content_types,
            supports_msgpack=supports_msgpack,
        )

    def _determine_response_rust_type(
        self,
        content_types: list[str],
        content: dict[str, Any],
        status_code: str,
        operation_id: str,
        response_data: dict[str, Any],
    ) -> str | None:
        """Determine the Rust type for a response."""
        if not content_types:
            return None

        first_content = content[content_types[0]]
        schema = first_content.get("schema", {})

        if self._should_create_response_model(schema, status_code):
            response_model_name = operation_id

            self.schemas[response_model_name] = self._create_response_schema(
                response_model_name,
                schema,
                response_data.get("description", ""),
            )

            return rust_pascal_case(response_model_name)

        return rust_type_from_openapi(schema, self.schemas, set())

    def _should_create_response_model(
        self,
        schema: dict[str, Any],
        status_code: str,
    ) -> bool:
        """Determine if we should create a response model for this schema."""
        if not status_code.startswith("2") or "$ref" in schema:
            return False

        if schema.get("type") == "object" and "properties" in schema:
            return True

        return bool("required" in schema or "allOf" in schema or "oneOf" in schema)

    def _create_response_schema(
        self,
        _name: str,
        schema: dict[str, Any],
        description: str,
    ) -> dict[str, Any]:
        """Create a response schema from an inline schema."""
        response_schema = schema.copy()
        if description and "description" not in response_schema:
            response_schema["description"] = description
        return response_schema

    def _parse_schemas(self) -> dict[str, Schema]:
        """Parse all schemas."""
        schemas = {}

        for schema_name, schema_data in self.schemas.items():
            schema = self._parse_schema(schema_name, schema_data)
            if schema:
                schemas[schema_name] = schema

        return schemas

    def _parse_schema(self, name: str, schema_data: dict[str, Any]) -> Schema | None:
        """Parse a single schema."""
        schema_type = schema_data.get("type", "object")
        properties_data = schema_data.get("properties", {})
        required_fields = schema_data.get("required", [])

        vendor_extensions = self._extract_vendor_extensions(schema_data)
        enum_values = self._extract_enum_values(schema_type, schema_data)

        underlying_rust_type = None
        properties = []

        if schema_type == "array":
            underlying_rust_type = self._handle_array_schema(schema_data)
        else:
            properties = self._parse_properties(properties_data, required_fields)

        return Schema(
            name=name,
            schema_type=schema_type,
            description=schema_data.get("description"),
            properties=properties,
            required_fields=required_fields,
            vendor_extensions=vendor_extensions,
            underlying_rust_type=underlying_rust_type,
            enum_values=enum_values,
        )

    def _extract_vendor_extensions(self, schema_data: dict[str, Any]) -> dict[str, Any]:
        """Extract vendor extensions from schema data."""
        vendor_extensions = {}
        for key, value in schema_data.items():
            if key.startswith("x-"):
                vendor_extensions[key] = value
        return vendor_extensions

    def _extract_enum_values(self, schema_type: str, schema_data: dict[str, Any]) -> list[str]:
        """Extract enum values if this is a string enum."""
        if schema_type == "string" and "enum" in schema_data:
            return list(schema_data["enum"])
        return []

    def _handle_array_schema(self, schema_data: dict[str, Any]) -> str:
        """Handle array schema and return underlying rust type."""
        items = schema_data.get("items", {})
        return f"Vec<{rust_type_from_openapi(items, self.schemas, set())}>"

    def _parse_properties(self, properties_data: dict[str, Any], required_fields: list[str]) -> list[Property]:
        """Parse properties from properties data."""
        properties = []
        for prop_name, prop_data in properties_data.items():
            prop = self._create_property(prop_name, prop_data, required_fields)
            properties.append(prop)
        return properties

    def _create_property(self, prop_name: str, prop_data: dict[str, Any], required_fields: list[str]) -> Property:
        """Create a Property object from property data."""
        rust_type = rust_type_from_openapi(prop_data, self.schemas, set())
        is_binary_field = detect_binary_field(prop_data)
        prop_vendor_extensions = self._extract_property_vendor_extensions(prop_data)
        items_property = self._create_items_property_if_needed(prop_name, prop_data)

        return Property(
            name=prop_name,
            rust_type=rust_type,
            required=prop_name in required_fields,
            description=prop_data.get("description"),
            is_base64_encoded=is_binary_field,
            vendor_extensions=prop_vendor_extensions,
            format=prop_data.get("format"),
            items=items_property,
        )

    def _extract_property_vendor_extensions(self, prop_data: dict[str, Any]) -> list[tuple[str, Any]]:
        """Extract vendor extensions for a property."""
        prop_vendor_extensions = []
        for key, value in prop_data.items():
            if key.startswith("x-"):
                prop_vendor_extensions.append((key, value))
        return prop_vendor_extensions

    def _create_items_property_if_needed(self, prop_name: str, prop_data: dict[str, Any]) -> Property | None:
        """Create items property for array properties if needed."""
        if prop_data.get("type") == "array" and "items" in prop_data:
            items_data = prop_data["items"]
            items_vendor_extensions = self._extract_property_vendor_extensions(items_data)

            return Property(
                name=f"{prop_name}_item",
                rust_type=rust_type_from_openapi(items_data, self.schemas, set()),
                required=False,
                description=items_data.get("description"),
                is_base64_encoded=detect_binary_field(items_data),
                vendor_extensions=items_vendor_extensions,
                format=items_data.get("format"),
            )
        return None

    def _extract_content_types(self) -> list[str]:
        """Extract all content types used in the API."""
        content_types = set()

        if not self.spec_data:
            return []

        for path_item in self.spec_data.get("paths", {}).values():
            for operation in path_item.values():
                if isinstance(operation, dict):
                    request_body = operation.get("requestBody", {})
                    content = request_body.get("content", {})
                    content_types.update(content.keys())

                    for response in operation.get("responses", {}).values():
                        response_content = response.get("content", {})
                        content_types.update(response_content.keys())

        return sorted(content_types)
