from typing import Union

__version__: str

def xxh3_64(data: Union[bytes, bytearray], seed: int = 0) -> int:
    """Returns the 64-bit XXH3 hash of `data` (`bytes` or `bytearray`), as an int."""

def xxh3_128(data: Union[bytes, bytearray], seed: int = 0) -> int:
    """Returns the 128-bit XXH3 hash of `data` (`bytes` or `bytearray`), as an int."""
