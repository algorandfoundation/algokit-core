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

from pydantic import BaseModel, ConfigDict, Field
from typing import Any, ClassVar, Dict, List
from algokit_algod_api.models.ledger_state_delta_for_transaction_group import LedgerStateDeltaForTransactionGroup
from typing import Optional, Set
from typing_extensions import Self

class GetTransactionGroupLedgerStateDeltasForRound200Response(BaseModel):
    """
    GetTransactionGroupLedgerStateDeltasForRound200Response
    """ # noqa: E501
    deltas: List[LedgerStateDeltaForTransactionGroup] = Field(alias="Deltas")
    __properties: ClassVar[List[str]] = ["Deltas"]

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
        """Create an instance of GetTransactionGroupLedgerStateDeltasForRound200Response from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in deltas (list)
        _items = []
        if self.deltas:
            for _item_deltas in self.deltas:
                if _item_deltas:
                    _items.append(_item_deltas.to_dict())
            _dict['Deltas'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of GetTransactionGroupLedgerStateDeltasForRound200Response from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "Deltas": [LedgerStateDeltaForTransactionGroup.from_dict(_item) for _item in obj["Deltas"]] if obj.get("Deltas") is not None else None
        })
        return _obj


