#!/usr/bin/env python3
"""
Script to add vendor extensions to Swagger/OpenAPI models indicating
whether they support msgpack encoding/decoding based on endpoint usage.
"""

import json
import sys
from typing import Dict, Set, Any, List, Optional, Tuple
from collections import defaultdict


class SwaggerModelAnalyzer:
    def __init__(self, spec: Dict[str, Any]):
        self.spec = spec
        self.encodable_models = set()  # Models used in msgpack requests
        self.decodable_models = set()  # Models used in msgpack responses
        self.model_references = defaultdict(set)  # Track nested model references
        self.response_models = defaultdict(set)  # Track which responses use which models
        self.debug_info = defaultdict(list)  # Track where models are used for debugging

    def analyze(self):
        """Main analysis function."""
        # First, build model reference graph
        self._build_model_references()

        # Analyze all response definitions first
        self._analyze_response_definitions()

        # Analyze all paths and operations
        for path, path_item in self.spec.get('paths', {}).items():
            for method, operation in path_item.items():
                if method in ['get', 'post', 'put', 'delete', 'patch', 'head', 'options']:
                    self._analyze_operation(operation, path, method)

        # Propagate encode/decode requirements through model references
        self._propagate_requirements()

        # Add vendor extensions to models
        self._add_vendor_extensions()

    def _build_model_references(self):
        """Build a graph of model references."""
        definitions = self.spec.get('definitions', {})

        for model_name, model_def in definitions.items():
            refs = self._find_refs_in_schema(model_def)
            for ref in refs:
                ref_model = ref.split('/')[-1]
                if ref_model != model_name:  # Avoid self-references
                    self.model_references[model_name].add(ref_model)

    def _find_refs_in_schema(self, schema: Any) -> Set[str]:
        """Recursively find all $ref references in a schema."""
        refs = set()

        if isinstance(schema, dict):
            if '$ref' in schema:
                refs.add(schema['$ref'])

            for key, value in schema.items():
                if key != '$ref':  # Avoid infinite recursion
                    refs.update(self._find_refs_in_schema(value))

        elif isinstance(schema, list):
            for item in schema:
                refs.update(self._find_refs_in_schema(item))

        return refs

    def _analyze_response_definitions(self):
        """Analyze response definitions in the responses section."""
        responses = self.spec.get('responses', {})

        for response_name, response_def in responses.items():
            if 'schema' in response_def:
                schema = response_def['schema']

                # If this is an inline object definition, treat the response name as a model
                if isinstance(schema, dict) and (schema.get('type') == 'object' or 'properties' in schema):
                    # Add this response as a model that might need extensions
                    self.response_models[response_name].add(response_name)
                    self.debug_info[response_name].append(f"Response definition: {response_name}")

                # Also extract any referenced models
                models = self._extract_models_from_schema(schema)
                self.response_models[response_name].update(models)

                # Track for debugging
                for model in models:
                    self.debug_info[model].append(f"Response definition: {response_name}")

    def _analyze_operation(self, operation: Dict[str, Any], path: str, method: str):
        """Analyze a single operation for msgpack usage."""
        operation_id = operation.get('operationId', f"{method} {path}")

        # Check if operation consumes/produces msgpack
        consumes = operation.get('consumes', self.spec.get('consumes', []))
        produces = operation.get('produces', self.spec.get('produces', []))

        supports_msgpack_input = 'application/msgpack' in consumes
        supports_msgpack_output = 'application/msgpack' in produces

        # Special case: some operations might have format parameter that affects content type
        has_format_param = any(
            param.get('name') == 'format' and param.get('enum') == ['json', 'msgpack']
            for param in operation.get('parameters', [])
        )

        if has_format_param:
            supports_msgpack_output = True

        # Analyze request models (for encoding)
        if supports_msgpack_input:
            # Check body parameters
            for param in operation.get('parameters', []):
                if param.get('in') == 'body' and 'schema' in param:
                    models = self._extract_models_from_schema(param['schema'])
                    self.encodable_models.update(models)

                    # Track for debugging
                    for model in models:
                        self.debug_info[model].append(f"Request body in {operation_id}")

        # Analyze response models (for decoding)
        if supports_msgpack_output:
            for status_code, response in operation.get('responses', {}).items():
                models_found = set()

                # Direct schema in response
                if 'schema' in response:
                    models = self._extract_models_from_schema(response['schema'])
                    models_found.update(models)

                # Response reference
                elif '$ref' in response:
                    ref = response['$ref']
                    if ref.startswith('#/responses/'):
                        response_name = ref.split('/')[-1]

                        # Add the response name itself as a model if it has an inline schema
                        response_def = self.spec.get('responses', {}).get(response_name, {})
                        if 'schema' in response_def and isinstance(response_def['schema'], dict):
                            schema = response_def['schema']
                            # Check if it's an inline object definition
                            if schema.get('type') == 'object' or 'properties' in schema:
                                models_found.add(response_name)
                                self.debug_info[response_name].append(f"Response {status_code} in {operation_id}")

                        # Get models from pre-analyzed response definitions
                        models_found.update(self.response_models.get(response_name, set()))

                        # Also analyze the response definition directly
                        if 'schema' in response_def:
                            models = self._extract_models_from_schema(response_def['schema'])
                            models_found.update(models)

                # Add all found models as decodable
                self.decodable_models.update(models_found)

                # Track for debugging
                for model in models_found:
                    if model not in self.debug_info or f"Response {status_code} in {operation_id}" not in self.debug_info[model]:
                        self.debug_info[model].append(f"Response {status_code} in {operation_id}")

    def _extract_models_from_schema(self, schema: Any, depth: int = 0) -> Set[str]:
        """Extract all model names referenced in a schema."""
        models = set()

        # Prevent infinite recursion
        if depth > 20:
            return models

        if isinstance(schema, dict):
            # Direct model reference
            if '$ref' in schema:
                ref = schema['$ref']
                if ref.startswith('#/definitions/'):
                    model_name = ref.split('/')[-1]
                    models.add(model_name)

                    # Don't recurse into the referenced model here (handled by propagation)
                    return models

            # Object with properties
            if 'properties' in schema:
                for prop_name, prop_schema in schema.get('properties', {}).items():
                    models.update(self._extract_models_from_schema(prop_schema, depth + 1))

            # Additional properties
            if 'additionalProperties' in schema:
                models.update(self._extract_models_from_schema(schema['additionalProperties'], depth + 1))

            # Array items
            if 'items' in schema:
                models.update(self._extract_models_from_schema(schema['items'], depth + 1))

            # Composition keywords
            for key in ['allOf', 'anyOf', 'oneOf']:
                if key in schema:
                    for sub_schema in schema[key]:
                        models.update(self._extract_models_from_schema(sub_schema, depth + 1))

            # Not keyword
            if 'not' in schema:
                models.update(self._extract_models_from_schema(schema['not'], depth + 1))

        elif isinstance(schema, list):
            for item in schema:
                models.update(self._extract_models_from_schema(item, depth + 1))

        return models

    def _propagate_requirements(self):
        """Propagate encode/decode requirements through model references."""
        # If a model is encodable/decodable, all models it references should be too
        max_iterations = 100  # Prevent infinite loops
        iteration = 0

        while iteration < max_iterations:
            iteration += 1
            changed = False

            # Propagate encodable
            encodable_before = len(self.encodable_models)
            for model in list(self.encodable_models):
                for referenced_model in self.model_references.get(model, set()):
                    if referenced_model not in self.encodable_models:
                        self.encodable_models.add(referenced_model)
                        self.debug_info[referenced_model].append(f"Inherited encoding from {model}")

            # Propagate decodable
            decodable_before = len(self.decodable_models)
            for model in list(self.decodable_models):
                for referenced_model in self.model_references.get(model, set()):
                    if referenced_model not in self.decodable_models:
                        self.decodable_models.add(referenced_model)
                        self.debug_info[referenced_model].append(f"Inherited decoding from {model}")

            changed = (len(self.encodable_models) > encodable_before or
                      len(self.decodable_models) > decodable_before)

            if not changed:
                break

    def _add_vendor_extensions(self):
        """Add vendor extensions to model definitions and response schemas."""
        # Process definitions
        definitions = self.spec.get('definitions', {})

        for model_name, model_def in definitions.items():
            # Add encoding extension
            if model_name in self.encodable_models:
                model_def['x-algorand-msgpack-encodable'] = True

            # Add decoding extension
            if model_name in self.decodable_models:
                model_def['x-algorand-msgpack-decodable'] = True


        # Process response schemas
        responses = self.spec.get('responses', {})

        for response_name, response_def in responses.items():
            # Check if this response should have extensions
            should_encode = response_name in self.encodable_models
            should_decode = response_name in self.decodable_models

            if should_encode or should_decode:
                # Add extensions to the response schema if it exists
                if 'schema' in response_def and isinstance(response_def['schema'], dict):
                    schema = response_def['schema']

                    # Only add to object schemas, not simple types or refs
                    if schema.get('type') == 'object' or 'properties' in schema:
                        if should_encode:
                            schema['x-algorand-msgpack-encodable'] = True
                        if should_decode:
                            schema['x-algorand-msgpack-decodable'] = True


    def get_statistics(self) -> Dict[str, Any]:
        """Get statistics about the analysis."""
        definitions = self.spec.get('definitions', {})
        responses = self.spec.get('responses', {})

        # Count inline response schemas as models too
        total_models = len(definitions)
        response_schemas = 0
        for response_name, response_def in responses.items():
            if 'schema' in response_def and isinstance(response_def['schema'], dict):
                schema = response_def['schema']
                if schema.get('type') == 'object' or 'properties' in schema:
                    response_schemas += 1

        total_models += response_schemas

        # Find models that might be missing extensions
        models_with_refs = set()
        for model_name, model_def in definitions.items():
            if self._find_refs_in_schema(model_def):
                models_with_refs.add(model_name)

        potentially_missing = models_with_refs - self.encodable_models - self.decodable_models

        return {
            'total_models': total_models,
            'total_definitions': len(definitions),
            'total_response_schemas': response_schemas,
            'encodable_models': len(self.encodable_models),
            'decodable_models': len(self.decodable_models),
            'both_encode_decode': len(self.encodable_models.intersection(self.decodable_models)),
            'neither': total_models - len(self.encodable_models.union(self.decodable_models)),
            'models_with_refs': len(models_with_refs),
            'potentially_missing': len(potentially_missing)
        }

    def get_detailed_report(self) -> str:
        """Generate a detailed report of the analysis."""
        report = []
        definitions = self.spec.get('definitions', {})

        # Models that need both encoding and decoding
        both = self.encodable_models.intersection(self.decodable_models)
        if both:
            report.append("\nModels that need BOTH encoding and decoding:")
            for model in sorted(both):
                report.append(f"  - {model}")
                if model in self.debug_info:
                    for usage in self.debug_info[model][:2]:
                        report.append(f"      {usage}")

        # Models that only need encoding
        encode_only = self.encodable_models - self.decodable_models
        if encode_only:
            report.append("\nModels that ONLY need encoding:")
            for model in sorted(encode_only):
                report.append(f"  - {model}")

        # Models that only need decoding
        decode_only = self.decodable_models - self.encodable_models
        if decode_only:
            report.append("\nModels that ONLY need decoding:")
            for model in sorted(decode_only):
                report.append(f"  - {model}")

        # Models with neither
        all_models = set(definitions.keys())
        neither = all_models - self.encodable_models - self.decodable_models
        if len(neither) < 20:  # Only show if not too many
            report.append("\nModels that need NEITHER:")
            for model in sorted(neither):
                report.append(f"  - {model}")

        return "\n".join(report)


def main():
    # Read input file
    if len(sys.argv) > 1:
        input_file = sys.argv[1]
        output_file = sys.argv[2] if len(sys.argv) > 2 else 'output_swagger.json'
    else:
        print("Usage: python add_vendor_extensions.py <input_swagger.json> [output_swagger.json]")
        print("\nThis script analyzes a Swagger/OpenAPI specification and adds vendor extensions")
        print("to indicate which models support msgpack encoding/decoding based on endpoint usage.")
        print("\nOptions:")
        sys.exit(1)

    try:
        # Load the Swagger spec
        with open(input_file, 'r') as f:
            spec = json.load(f)

        print(f"Analyzing Swagger specification: {input_file}")
        print(f"Swagger version: {spec.get('swagger', 'unknown')}")
        print(f"API title: {spec.get('info', {}).get('title', 'unknown')}")

        # Analyze the spec
        analyzer = SwaggerModelAnalyzer(spec)
        analyzer.analyze()

        # Print statistics
        stats = analyzer.get_statistics()
        print(f"\n{'='*60}")
        print("ANALYSIS SUMMARY")
        print(f"{'='*60}")
        print(f"  Total models: {stats['total_models']}")
        print(f"    - In definitions section: {stats['total_definitions']}")
        print(f"    - Inline response schemas: {stats['total_response_schemas']}")
        print(f"  Models that need encoding: {stats['encodable_models']}")
        print(f"  Models that need decoding: {stats['decodable_models']}")
        print(f"  Models that need both: {stats['both_encode_decode']}")
        print(f"  Models that need neither: {stats['neither']}")
        print(f"  Models with references: {stats['models_with_refs']}")

        if stats['potentially_missing'] > 0:
            print(f"\n  ⚠️  Potentially missing extensions: {stats['potentially_missing']}")

        # Print detailed report
        detailed_report = analyzer.get_detailed_report()
        if detailed_report:
            print(f"\n{'='*60}")
            print("DETAILED REPORT")
            print(f"{'='*60}")
            print(detailed_report)

        # Save the modified spec
        with open(output_file, 'w') as f:
            json.dump(spec, f, indent=2)

        print(f"\n{'='*60}")
        print(f"✅ Modified specification saved to: {output_file}")

        # Print example of added extensions
        print("\nExample vendor extensions added:")
        example_count = 0

        # Check definitions first
        for model_name, model_def in spec.get('definitions', {}).items():
            if ('x-algorand-msgpack-encodable' in model_def or
                'x-algorand-msgpack-decodable' in model_def):
                print(f"\n{model_name} (definition):")
                if 'x-algorand-msgpack-encodable' in model_def:
                    print(f"  x-algorand-msgpack-encodable: true")
                if 'x-algorand-msgpack-decodable' in model_def:
                    print(f"  x-algorand-msgpack-decodable: true")
                example_count += 1
                if example_count >= 2:
                    break

        # Then check response schemas
        for response_name, response_def in spec.get('responses', {}).items():
            if example_count >= 3:
                break
            if 'schema' in response_def and isinstance(response_def['schema'], dict):
                schema = response_def['schema']
                if ('x-algorand-msgpack-encodable' in schema or
                    'x-algorand-msgpack-decodable' in schema):
                    print(f"\n{response_name} (response schema):")
                    if 'x-algorand-msgpack-encodable' in schema:
                        print(f"  x-algorand-msgpack-encodable: true")
                    if 'x-algorand-msgpack-decodable' in schema:
                        print(f"  x-algorand-msgpack-decodable: true")
                    example_count += 1

    except FileNotFoundError:
        print(f"❌ Error: Input file '{input_file}' not found.")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"❌ Error: Invalid JSON in input file: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
