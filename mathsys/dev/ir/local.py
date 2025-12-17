#^
#^  TYPES
#^

#> TYPES -> U8 CLASS
class u8:
    def __new__(self, value: int) -> bytes:
        if not 1 <= value <= 2**8 - 1: raise ValueError(f"'{value}' is outside range for u8.")
        return bytes([value])

#> TYPES -> NULL8 CLASS
class null8:
    def __new__(self) -> bytes: return bytes([0])

#> TYPES -> U32 CLASS
class u32:
    def __new__(self, value: int) -> bytes:
        if not 1 <= value <= 2**32 - 1: raise ValueError(f"'{value}' is outside range for u32.")
        return bytes([
            (value) & 0xFF,
            (value >> 8) & 0xFF,
            (value >> 16) & 0xFF,
            (value >> 24) & 0xFF
        ])
    
#> TYPES -> NULL32 CLASS
class null32:
    def __new__(self) -> bytes: return bytes([0, 0, 0, 0])


#^
#^  HELPERS
#^

#> HELPERS -> JOIN
def join(binary: list[bytes], delimiter: null8 | null32) -> bytes: 
    return b"".join(binary) + delimiter


#^
#^  MAPPINGS
#^

#> MAPPINGS -> OBJECTTYPE
OBJECTTYPE = {
    None: null8(),
    "@Infinite": u8(1),
    "@Natural": u8(2),
    "@Nexists": u8(3),
    "@Tensor": u8(4),
    "@Undefined": null8(),
    "@Variable": u8(5)
}