# coding: utf-8

"""
    Algod REST API.

    API endpoint for algod operations.

    The version of the OpenAPI document: 0.0.1
    Contact: contact@algorand.com
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json
# Optional support for Algorand compact MessagePack via the `algokit_transact`
# Python bindings (built from the Rust crate). We do the import lazily so that
# the generated client can still function even if the optional binary wheel is
# not available on the target platform.
try:
    from algokit_transact import (
        encode_json_to_msgpack as _ak_encode_msgpack,
        decode_msgpack_to_json as _ak_decode_msgpack,
        ModelType as _AkModelType,
    )
except ModuleNotFoundError:  # pragma: no cover – optional dependency
    _ak_encode_msgpack = None  # type: ignore
    _ak_decode_msgpack = None  # type: ignore
    _AkModelType = None  # type: ignore

from pydantic import BaseModel, ConfigDict, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from typing import Optional, Set
from typing_extensions import Self

class ErrorResponse(BaseModel):
    """
    An error response with optional data field.
    """ # noqa: E501
    data: Optional[Dict[str, Any]] = None
    message: StrictStr
    __properties: ClassVar[List[str]] = ["data", "message"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of ErrorResponse from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of ErrorResponse from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "data": obj.get("data"),
            "message": obj.get("message")
        })
        return _obj

    def to_msgpack(self) -> bytes:  # pragma: no cover – thin wrapper
        """Return the compact Algorand MessagePack representation of this model.

        Requires the optional ``algokit_transact`` binary package to be
        installed. If the model is not one of the types supported by that
        package (for example *SimulateRequest* or *SimulateTransaction200Response*)
        a :class:`NotImplementedError` is raised.
        """
        if _ak_encode_msgpack is None or _AkModelType is None:
            raise RuntimeError(
                "algokit_transact is not available — install the algokit_transact package"
                "to use MessagePack helpers"
            )

        try:
            model_type = _AkModelType[self.__class__.__name__]
        except KeyError as exc:  # pragma: no cover
            # Fallback: convert CamelCase -> UPPER_SNAKE_CASE (SimulateRequest -> SIMULATE_REQUEST)
            variant_name = re.sub(r'(?<!^)(?=[A-Z])', '_', self.__class__.__name__).upper()
            try:
                model_type = _AkModelType[variant_name]
            except KeyError as exc:  # pragma: no cover
                raise NotImplementedError(
                    f"Model {self.__class__.__name__} ({variant_name}) is not supported by algokit_transact"
                ) from exc

        return _ak_encode_msgpack(model_type, self.to_json())  # type: ignore[arg-type]

    @classmethod
    def from_msgpack(cls, data: bytes) -> "Self":  # pragma: no cover – thin wrapper
        """Create a new instance from Algorand MessagePack *data*.

        The inverse of :pymeth:`to_msgpack`. Requires
        ``algokit_transact`` to be importable.
        """
        if _ak_decode_msgpack is None or _AkModelType is None:
            raise RuntimeError(
                "algokit_transact is not available — install the algokit_transact package"
                "to use MessagePack helpers"
            )

        try:
            model_type = _AkModelType[cls.__name__]
        except KeyError as exc:  # pragma: no cover
            variant_name = re.sub(r'(?<!^)(?=[A-Z])', '_', cls.__name__).upper()
            try:
                model_type = _AkModelType[variant_name]
            except KeyError as exc:  # pragma: no cover
                raise NotImplementedError(
                    f"Model {cls.__name__} ({variant_name}) is not supported by algokit_transact"
                ) from exc

        json_str = _ak_decode_msgpack(model_type, data)  # type: ignore[arg-type]
        return cls.from_json(json_str)


