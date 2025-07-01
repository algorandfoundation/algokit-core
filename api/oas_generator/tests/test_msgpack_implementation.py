"""
Test msgpack implementation logic in the OAS parser.

This test validates that all schemas related to msgpack operations
(directly or indirectly through dependencies) properly implement the msgpack trait.
"""

from pathlib import Path

import pytest

from rust_oas_generator.parser.oas_parser import OASParser, ParsedSpec

# Constants for test thresholds
MIN_MSGPACK_COVERAGE = 50.0
EXPECTED_ALGOD_COVERAGE = 80.0


class TestMsgpackImplementation:
    """Test class for msgpack implementation validation."""

    @pytest.fixture
    def algod_spec_path(self) -> Path:
        """Get the path to the algod OAS spec."""
        spec_path = Path(__file__).parent.parent / "specs" / "algod.oas3.json"
        if not spec_path.exists():
            spec_path = Path(__file__).parent.parent.parent / "specs" / "algod.oas3.json"

        if not spec_path.exists():
            pytest.skip("algod.oas3.json not found")

        return spec_path

    @pytest.fixture
    def parsed_spec(self, algod_spec_path: Path) -> tuple[ParsedSpec, OASParser]:
        """Parse the algod OAS spec."""
        parser = OASParser()
        return parser.parse_file(algod_spec_path), parser

    def test_msgpack_operations_detected(self, parsed_spec: tuple[ParsedSpec, OASParser]) -> None:
        """Test that msgpack operations are correctly detected."""
        spec, parser = parsed_spec

        # Should have msgpack operations
        assert spec.has_msgpack_operations, "Should detect msgpack operations in algod spec"

        # Find msgpack operations
        msgpack_operations = [op for op in spec.operations if op.supports_msgpack]
        assert len(msgpack_operations) > 0, "Should find msgpack operations"

    def test_msgpack_request_body_detection(self, parsed_spec: tuple[ParsedSpec, OASParser]) -> None:
        """Test that request bodies with msgpack support are detected."""
        spec, parser = parsed_spec

        # Find operations with msgpack request bodies
        msgpack_request_ops = [op for op in spec.operations if op.request_body_supports_msgpack]

        assert len(msgpack_request_ops) > 0, "Should find operations with msgpack request bodies"

    def test_root_msgpack_schemas_identified(self, parsed_spec: tuple[ParsedSpec, OASParser]) -> None:
        """Test that root msgpack schemas are correctly identified."""
        spec, parser = parsed_spec

        root_schemas = parser._get_msgpack_root_schemas()  # noqa: SLF001
        assert len(root_schemas) > 0, "Should identify root msgpack schemas"

    def test_dependency_graph_built(self, parsed_spec: tuple[ParsedSpec, OASParser]) -> None:
        """Test that schema dependency graph is built correctly."""
        spec, parser = parsed_spec

        dependency_graph = parser._build_schema_dependency_graph()  # noqa: SLF001
        assert len(dependency_graph) > 0, "Should build dependency graph"

        # Check some known dependencies
        if "Account" in dependency_graph:
            account_deps = dependency_graph["Account"]
            expected_deps = {"ApplicationLocalState", "AssetHolding", "AccountParticipation"}
            for expected_dep in expected_deps:
                assert expected_dep in account_deps, f"Account should depend on {expected_dep}"

        if "SimulateRequest" in dependency_graph:
            simulate_deps = dependency_graph["SimulateRequest"]
            assert "SimulateRequestTransactionGroup" in simulate_deps, (
                "SimulateRequest should depend on SimulateRequestTransactionGroup"
            )

    def test_all_msgpack_schemas_implement_trait(self, parsed_spec: tuple[ParsedSpec, OASParser]) -> None:
        """Test that all schemas requiring msgpack implement the trait."""
        spec, parser = parsed_spec

        # Get root schemas and build dependency graph
        root_msgpack_schemas = parser._get_msgpack_root_schemas()  # noqa: SLF001
        dependency_graph = parser._build_schema_dependency_graph()  # noqa: SLF001

        # Find all schemas that should implement msgpack using BFS
        msgpack_schemas = set()
        queue = list(root_msgpack_schemas)
        visited = set()

        while queue:
            schema_name = queue.pop(0)
            if schema_name in visited:
                continue
            visited.add(schema_name)
            msgpack_schemas.add(schema_name)

            if schema_name in dependency_graph:
                for dep in dependency_graph[schema_name]:
                    if dep not in visited:
                        queue.append(dep)

        assert len(msgpack_schemas) > 0, "Should find schemas requiring msgpack"

        # Check that all these schemas implement msgpack
        missing_implementations = []
        for schema_name in msgpack_schemas:
            schema = spec.schemas.get(schema_name)
            if schema and not schema.implements_algokit_msgpack:
                missing_implementations.append(schema_name)

        assert len(missing_implementations) == 0, f"Schemas missing msgpack implementation: {missing_implementations}"

    def test_response_models_implement_msgpack(self, parsed_spec: tuple[ParsedSpec, OASParser]) -> None:
        """Test that response models for msgpack operations implement msgpack."""
        spec, parser = parsed_spec

        # Find msgpack operations
        msgpack_operations = [op for op in spec.operations if op.supports_msgpack]

        for operation in msgpack_operations:
            for _status_code, response in operation.responses.items():
                if response.rust_type and response.supports_msgpack:
                    # Find the schema for this response
                    response_schema = spec.schemas.get(response.rust_type)
                    if response_schema:
                        assert response_schema.implements_algokit_msgpack, (
                            f"Response schema {response.rust_type} for {operation.operation_id} "
                            "should implement msgpack"
                        )

    def test_no_false_positives(self, parsed_spec: tuple[ParsedSpec, OASParser]) -> None:
        """Test that schemas not related to msgpack don't unnecessarily implement it."""
        spec, parser = parsed_spec

        # Get all schemas that should implement msgpack
        root_msgpack_schemas = parser._get_msgpack_root_schemas()  # noqa: SLF001
        dependency_graph = parser._build_schema_dependency_graph()  # noqa: SLF001

        msgpack_schemas = set()
        queue = list(root_msgpack_schemas)
        visited = set()

        while queue:
            schema_name = queue.pop(0)
            if schema_name in visited:
                continue
            visited.add(schema_name)
            msgpack_schemas.add(schema_name)

            if schema_name in dependency_graph:
                for dep in dependency_graph[schema_name]:
                    if dep not in visited:
                        queue.append(dep)

        # Check that schemas implementing msgpack are in the expected set
        for schema_name, schema in spec.schemas.items():
            if schema.implements_algokit_msgpack:
                assert schema_name in msgpack_schemas, (
                    f"Schema {schema_name} implements msgpack but is not in expected msgpack schema set"
                )
