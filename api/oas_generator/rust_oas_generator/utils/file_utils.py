"""
File utilities for the OAS generator.

This module provides file and directory operations for the Rust OAS generator
with proper type annotations and documentation.
"""

import shutil
from pathlib import Path


def write_files_to_disk(files: dict[Path, str]) -> None:
    """Write generated files to disk.

    Args:
        files: Dictionary mapping file paths to their content.
    """
    for path, content in files.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")


def clean_output_directory(output_dir: Path) -> None:
    """Clean the output directory by removing all files and subdirectories.

    Args:
        output_dir: Path to the output directory to clean.
    """
    shutil.rmtree(output_dir, ignore_errors=True)
    output_dir.mkdir(parents=True, exist_ok=True)


def copy_file(src: Path, dest: Path) -> None:
    """Copy a file from source to destination.

    Args:
        src: Source file path.
        dest: Destination file path.
    """
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dest)


def ensure_directory(directory: Path) -> None:
    """Ensure that a directory exists.

    Args:
        directory: Path to the directory to create.
    """
    directory.mkdir(parents=True, exist_ok=True)


def get_relative_path(file_path: Path, base_path: Path) -> Path:
    """Get relative path from base_path to file_path.

    Args:
        file_path: Target file path.
        base_path: Base path to calculate relative path from.

    Returns:
        Relative path from base_path to file_path, or the original
        path if it cannot be made relative.
    """
    try:
        return file_path.relative_to(base_path)
    except ValueError:
        return file_path


def list_rust_files(directory: Path) -> list[Path]:
    """List all .rs files in a directory recursively.

    Args:
        directory: Directory to search for Rust files.

    Returns:
        Sorted list of paths to .rs files.
    """
    if not directory.is_dir():
        return []
    return sorted(directory.rglob("*.rs"))
