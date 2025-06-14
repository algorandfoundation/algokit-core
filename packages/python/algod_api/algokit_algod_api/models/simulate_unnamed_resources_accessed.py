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

from pydantic import BaseModel, ConfigDict, Field, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from algokit_algod_api.models.application_local_reference import ApplicationLocalReference
from algokit_algod_api.models.asset_holding_reference import AssetHoldingReference
from algokit_algod_api.models.box_reference import BoxReference
from typing import Optional, Set
from typing_extensions import Self

class SimulateUnnamedResourcesAccessed(BaseModel):
    """
    These are resources that were accessed by this group that would normally have caused failure, but were allowed in simulation. Depending on where this object is in the response, the unnamed resources it contains may or may not qualify for group resource sharing. If this is a field in SimulateTransactionGroupResult, the resources do qualify, but if this is a field in SimulateTransactionResult, they do not qualify. In order to make this group valid for actual submission, resources that qualify for group sharing can be made available by any transaction of the group; otherwise, resources must be placed in the same transaction which accessed them.
    """ # noqa: E501
    accounts: Optional[List[StrictStr]] = Field(default=None, description="The unnamed accounts that were referenced. The order of this array is arbitrary.")
    assets: Optional[List[StrictInt]] = Field(default=None, description="The unnamed assets that were referenced. The order of this array is arbitrary.")
    apps: Optional[List[StrictInt]] = Field(default=None, description="The unnamed applications that were referenced. The order of this array is arbitrary.")
    boxes: Optional[List[BoxReference]] = Field(default=None, description="The unnamed boxes that were referenced. The order of this array is arbitrary.")
    extra_box_refs: Optional[StrictInt] = Field(default=None, description="The number of extra box references used to increase the IO budget. This is in addition to the references defined in the input transaction group and any referenced to unnamed boxes.", alias="extra-box-refs")
    asset_holdings: Optional[List[AssetHoldingReference]] = Field(default=None, description="The unnamed asset holdings that were referenced. The order of this array is arbitrary.", alias="asset-holdings")
    app_locals: Optional[List[ApplicationLocalReference]] = Field(default=None, description="The unnamed application local states that were referenced. The order of this array is arbitrary.", alias="app-locals")
    __properties: ClassVar[List[str]] = ["accounts", "assets", "apps", "boxes", "extra-box-refs", "asset-holdings", "app-locals"]

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
        """Create an instance of SimulateUnnamedResourcesAccessed from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in boxes (list)
        _items = []
        if self.boxes:
            for _item_boxes in self.boxes:
                if _item_boxes:
                    _items.append(_item_boxes.to_dict())
            _dict['boxes'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in asset_holdings (list)
        _items = []
        if self.asset_holdings:
            for _item_asset_holdings in self.asset_holdings:
                if _item_asset_holdings:
                    _items.append(_item_asset_holdings.to_dict())
            _dict['asset-holdings'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in app_locals (list)
        _items = []
        if self.app_locals:
            for _item_app_locals in self.app_locals:
                if _item_app_locals:
                    _items.append(_item_app_locals.to_dict())
            _dict['app-locals'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SimulateUnnamedResourcesAccessed from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "accounts": obj.get("accounts"),
            "assets": obj.get("assets"),
            "apps": obj.get("apps"),
            "boxes": [BoxReference.from_dict(_item) for _item in obj["boxes"]] if obj.get("boxes") is not None else None,
            "extra-box-refs": obj.get("extra-box-refs"),
            "asset-holdings": [AssetHoldingReference.from_dict(_item) for _item in obj["asset-holdings"]] if obj.get("asset-holdings") is not None else None,
            "app-locals": [ApplicationLocalReference.from_dict(_item) for _item in obj["app-locals"]] if obj.get("app-locals") is not None else None
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


