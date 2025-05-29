"""API response object."""

from __future__ import annotations
from typing import Optional, Generic, Mapping, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

@dataclass
class ApiResponse(Generic[T]):
    """
    API response object
    """

    status_code: int
    headers: Optional[Mapping[str, str]] = None
    data: Optional[T] = None
    raw_data: Optional[bytes] = None
