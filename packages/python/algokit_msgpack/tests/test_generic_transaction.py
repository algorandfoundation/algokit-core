
from algokit_msgpack.algokit_msgpack import (
    json_to_msgpack,
    msgpack_to_json,
)


# Polytest Suite: Generic Transaction

# Polytest Group: Generic Transaction Tests


def test_malformed_bytes():
    test_data = '{"test": "test"}'
    msgpack_data = json_to_msgpack(test_data)
    assert msgpack_data is not None
    assert len(msgpack_data) > 0
    assert msgpack_to_json(msgpack_data) == test_data
