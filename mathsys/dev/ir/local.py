#^
#^  HEAD
#^

#> HEAD -> MODULES
from typing import cast


#^
#^  TYPES
#^

#> TYPES -> U8 CLASS
class u8:
    def __new__(cls, value: int) -> bytes:
        if not 1 <= value <= 2**8 - 1: raise ValueError(f"'{value}' is outside range for u8.")
        return bytes([value])
    def __add__(self, other) -> bytes: return cast(bytes, self) + other
    def __radd__(self, other) -> bytes: return cast(bytes, self) + other

#> TYPES -> NULL8 CLASS
class null8:
    def __new__(cls) -> bytes: return bytes([0])
    def __add__(self, other) -> bytes: return cast(bytes, self) + other
    def __radd__(self, other) -> bytes: return cast(bytes, self) + other

#> TYPES -> U32 CLASS
class u32:
    def __new__(cls, value: int) -> bytes:
        if not 1 <= value <= 2**32 - 1: raise ValueError(f"'{value}' is outside range for u32.")
        return bytes([
            (value) & 0xFF,
            (value >> 8) & 0xFF,
            (value >> 16) & 0xFF,
            (value >> 24) & 0xFF
        ])
    def __add__(self, other) -> bytes: return cast(bytes, self) + other
    def __radd__(self, other) -> bytes: return cast(bytes, self) + other
    
#> TYPES -> NULL32 CLASS
class null32:
    def __new__(cls) -> bytes: return bytes([0, 0, 0, 0])
    def __add__(self, other) -> bytes: return cast(bytes, self) + other
    def __radd__(self, other) -> bytes: return cast(bytes, self) + other


#^
#^  HELPERS
#^

#> HELPERS -> JOIN
def join(binary: list[bytes] | list[u32] | list[u8], delimiter: bytes) -> bytes: 
    return b"".join(cast(list[bytes], binary)) + delimiter


#^
#^  MAPPINGS
#^

#> MAPPINGS -> OBJECTTYPE
OBJECTTYPE = {
    None: null8(),
    "@Infinite": u8(1),
    "@Integer": u8(2),
    "@Natural": u8(3),
    "@Nexists": u8(4),
    "@Tensor": u8(5),
    "@Undefined": null8(),
    "@Variable": u8(6),
    "@Whole": u8(7)
}