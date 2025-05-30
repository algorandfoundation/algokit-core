from _algokit_msgpack import *
import re


class FFINotImplementedError(NotImplementedError):
    pass


def _camel_to_snake_case(name: str) -> str:
    """
    Convert CamelCase or PascalCase to snake_case.

    Examples:
        TransactionParams200Response -> transaction_params_200_response
        Account -> account
        HTTPResponseCode -> http_response_code
    """
    name = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    name = re.sub("([a-z0-9])([A-Z])", r"\1_\2", name)
    return name.lower()


def _get_ffi_func(pkg_module, base_name, model_name_lower, direction, ffi_type):
    if ffi_type == "json":
        if direction == "to":
            func_name_specific = f"{model_name_lower}_to_json"
        else:
            func_name_specific = f"{model_name_lower}_from_json"
    elif ffi_type == "msgpack":
        if direction == "encode":
            func_name_specific = f"encode_{model_name_lower}"
        else:
            func_name_specific = f"decode_{model_name_lower}"
    else:
        raise ValueError("Invalid ffi_type")

    if hasattr(pkg_module, func_name_specific):
        return getattr(pkg_module, func_name_specific)

    if ffi_type == "msgpack":
        if direction == "encode" and hasattr(pkg_module, "encode_msgpack"):
            return getattr(pkg_module, "encode_msgpack")
        elif direction == "decode" and hasattr(pkg_module, "decode_msgpack"):
            return getattr(pkg_module, "decode_msgpack")

    return None


def model_to_json_str(model_instance) -> str:
    model_name = model_instance.__class__.__name__
    model_name_snake = _camel_to_snake_case(model_name)
    import algokit_msgpack

    ffi_func = _get_ffi_func(
        algokit_msgpack, model_name_snake, model_name_snake, "to", "json"
    )

    if ffi_func:
        return ffi_func(model_instance)
    else:
        raise FFINotImplementedError(
            f"FFI JSON serialization (to_json) not found for {model_name}"
        )


def model_from_json_str(model_class, json_data: str):
    model_name = model_class.__name__
    model_name_snake = _camel_to_snake_case(model_name)
    import algokit_msgpack

    ffi_func = _get_ffi_func(
        algokit_msgpack, model_name_snake, model_name_snake, "from", "json"
    )

    if ffi_func:
        return ffi_func(json_data)
    else:
        raise FFINotImplementedError(
            f"FFI JSON deserialization (from_json) not found for {model_name}"
        )


def model_to_msgpack(model_instance) -> bytes:
    model_name = model_instance.__class__.__name__
    model_name_snake = _camel_to_snake_case(model_name)
    import algokit_msgpack

    ffi_func = _get_ffi_func(
        algokit_msgpack, model_name_snake, model_name_snake, "encode", "msgpack"
    )

    if ffi_func:
        if ffi_func.__name__ == "encode_msgpack":
            pass
        return ffi_func(model_instance)
    else:
        raise FFINotImplementedError(
            f"FFI MessagePack encoding (encode_*) not found for {model_name}"
        )


def model_from_msgpack(model_class, msgpack_data: bytes):
    model_name = model_class.__name__
    model_name_snake = _camel_to_snake_case(model_name)
    import algokit_msgpack

    ffi_func = _get_ffi_func(
        algokit_msgpack, model_name_snake, model_name_snake, "decode", "msgpack"
    )

    if ffi_func:
        if ffi_func.__name__ == "decode_msgpack":
            raise FFINotImplementedError(
                f"Generic FFI MessagePack decoding for {model_name} is complex from Python helper; prefer specific decode functions."
            )
        return ffi_func(msgpack_data)
    else:
        raise FFINotImplementedError(
            f"FFI MessagePack decoding (decode_*) not found for {model_name}"
        )
