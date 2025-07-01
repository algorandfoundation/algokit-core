#!/usr/bin/env python3
"""Command-line interface for the Rust OAS Generator."""

import argparse
import contextlib
import json
import shutil
import sys
import tempfile
import traceback
from collections.abc import Generator
from pathlib import Path

from rust_oas_generator.generator.template_engine import RustCodeGenerator
from rust_oas_generator.parser.oas_parser import OASParser
from rust_oas_generator.utils.file_utils import write_files_to_disk

# Exit codes for better error reporting
EXIT_SUCCESS = 0
EXIT_FILE_NOT_FOUND = 1
EXIT_INVALID_JSON = 2
EXIT_GENERATION_ERROR = 3


def parse_command_line_args(args: list[str] | None = None) -> argparse.Namespace:
    """Create and configure the command line argument parser."""
    parser = argparse.ArgumentParser(
        description="Generate Rust client from OpenAPI specification",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s spec.json
  %(prog)s spec.json --output ./client --package-name my_client
  %(prog)s spec.json --verbose
        """,
    )
    parser.add_argument(
        "spec_file",
        type=Path,
        help="Path to OpenAPI specification file (JSON or YAML)",
        metavar="SPEC_FILE",
    )
    parser.add_argument(
        "--output",
        "-o",
        type=Path,
        default=Path("./generated"),
        help="Output directory for generated files (default: %(default)s)",
        dest="output_dir",
    )
    parser.add_argument(
        "--package-name",
        "-p",
        default="api_client",
        help="Name for the generated Rust package (default: %(default)s)",
        dest="package_name",
    )
    parser.add_argument(
        "--template-dir",
        "-t",
        type=Path,
        help="Custom template directory (optional)",
        dest="template_dir",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Enable verbose output",
    )
    parser.add_argument(
        "--description",
        "-d",
        help="Custom description for the generated package (overrides spec description)",
        dest="custom_description",
    )

    parsed_args = parser.parse_args(args)

    # Validate spec file exists
    if not parsed_args.spec_file.exists():
        parser.error(f"Specification file not found: {parsed_args.spec_file}")

    return parsed_args


def print_verbose_info(*, operation_count: int, schema_count: int) -> None:
    """Print verbose information about parsed specification."""
    print(f"Parsed {operation_count} operations")
    print(f"Found {schema_count} schemas")


def print_generation_summary(*, file_count: int, files: dict[Path, str], output_dir: Path) -> None:
    """Print summary of generated files."""
    print(f"Generated {file_count} files:")
    for file_path in sorted(files.keys()):
        print(f"  {file_path}")
    print(f"\nRust client generated successfully in {output_dir}")


@contextlib.contextmanager
def backup_and_clean_output_dir(output_dir: Path) -> Generator[None, None, None]:
    """A context manager to backup and clean the output directory."""
    backup_dir = None
    if output_dir.exists() and any(output_dir.iterdir()):
        backup_dir = Path(tempfile.mkdtemp())
        shutil.copytree(output_dir, backup_dir, dirs_exist_ok=True)

    # Clean output directory before generation
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    try:
        yield
    except Exception:
        if backup_dir:
            print(
                "Error: Generation failed. Restoring original content.",
                file=sys.stderr,
            )
            if output_dir.exists():
                shutil.rmtree(output_dir)
            shutil.copytree(backup_dir, output_dir, dirs_exist_ok=True)
        raise
    finally:
        if backup_dir:
            shutil.rmtree(backup_dir)


def generate_rust_client_from_spec(
    *,
    spec_file: Path,
    output_dir: Path,
    package_name: str,
    verbose: bool,
    custom_description: str | None = None,
) -> dict[Path, str]:
    """Generate Rust client from OpenAPI specification file."""
    # Parse OpenAPI specification
    parser = OASParser()
    parsed_spec = parser.parse_file(spec_file)

    if verbose:
        print_verbose_info(
            operation_count=len(parsed_spec.operations),
            schema_count=len(parsed_spec.schemas),
        )

    # Generate Rust client files
    generator = RustCodeGenerator()
    return generator.generate_client(
        parsed_spec,
        output_dir,
        package_name,
        custom_description=custom_description,
    )


def main(args: list[str] | None = None) -> int:
    """Generate Rust client from OpenAPI specification."""
    parsed_args = parse_command_line_args(args)

    try:
        with backup_and_clean_output_dir(parsed_args.output_dir):
            generated_files = generate_rust_client_from_spec(
                spec_file=parsed_args.spec_file,
                output_dir=parsed_args.output_dir,
                package_name=parsed_args.package_name,
                verbose=parsed_args.verbose,
                custom_description=parsed_args.custom_description,
            )

            # Write files to disk
            write_files_to_disk(generated_files)

            if parsed_args.verbose:
                print_generation_summary(
                    file_count=len(generated_files),
                    files=generated_files,
                    output_dir=parsed_args.output_dir,
                )
            else:
                print(f"Rust client generated successfully in {parsed_args.output_dir}")

        return EXIT_SUCCESS

    except FileNotFoundError:
        print(f"Error: Specification file not found: {parsed_args.spec_file}", file=sys.stderr)
        return EXIT_FILE_NOT_FOUND
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in specification file: {e}", file=sys.stderr)
        return EXIT_INVALID_JSON
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        if parsed_args.verbose:
            traceback.print_exc()
        return EXIT_GENERATION_ERROR


if __name__ == "__main__":
    sys.exit(main())
