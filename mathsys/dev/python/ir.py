#^
#^  HEAD
#^

#> HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass
from typing import Protocol, TypeVar, Generic
from zlib import compress


#^
#^  HELPERS
#^

#> HELPERS -> NODE
def node(node: Binary, binary: list[Binary], nodes: list[Binary]) -> Pointer:
    try: return Pointer(nodes.index(node))
    except ValueError: 
        nodes.append(node)
        binary.append(node)
        return Pointer(len(nodes) - 1)


#^
#^  PROTOCOL
#^

#> PROTOCOL -> DEFINITION
class BinaryEncodable(Protocol):
    def binary(self) -> Binary: ...

#> PROTOCOL -> TYPEVAR
Type = TypeVar("Type", bound = BinaryEncodable)


#^
#^  TYPES
#^

#> TYPES -> BINARY
@dataclass(kw_only = True, frozen = True)
class Binary:
    value: int
    width: int
    def __bytes__(self) -> bytes:
        value = Binary(
            value = self.value,
            width = self.width
        )
        while value.width % 8 != 0: value = value + Opcode(0x00)
        return compress(value.value.to_bytes(value.width // 8, byteorder = "little"), level = 9, wbits = -15)
    def __add__(self, other: Binary | BinaryEncodable) -> "Binary":
        if not isinstance(other, Binary): other = other.binary()
        return Binary(
            value = self.value | (other.value << self.width),
            width = self.width + other.width
        )

#> TYPES -> OPCODE
@dataclass(frozen = True)
class Opcode:
    value: int
    def binary(self) -> Binary:
        return Binary(
            value = self.value,
            width = 5
        )

#> TYPES -> POINTER
@dataclass(frozen = True)
class Pointer:
    value: int
    def binary(self) -> Binary: 
        return Binary(
            value = self.value, 
            width = 32
        )

#> TYPES -> SIGN
@dataclass(frozen = True)
class Sign:
    value: bool
    def binary(self) -> Binary: 
        return Binary(
            value = 1 if self.value else 0,
            width = 1
        )

#> TYPES -> OPTION
@dataclass(frozen = True)
class Option(Generic[Type]):
    value: Binary | BinaryEncodable | None
    def binary(self) -> Binary:
        if self.value is None:
            return Binary(value = 0, width = 1)
        else:
            return Binary(value = 1, width = 1) + self.value

#> TYPES -> BIGUINT
@dataclass(frozen = True)
class BigUint:
    value: int
    def binary(self) -> Binary:
        return Binary(
            value = self.value,
            width = 128
        )

#> TYPES -> STRING
@dataclass(frozen = True)
class String:
    value: str
    def binary(self) -> Binary:
        data = self.value.encode()
        length = len(data)
        return Binary(
            value = length,
            width = 16
        ) + Binary(
            value = int.from_bytes(data, byteorder = "little"),
            width = length * 8
        )

#> TYPES -> GROUP
@dataclass(frozen = True)
class Group:
    value: str
    def binary(self) -> Binary: return Binary(
        value = {
            "@Infinite": 1,
            "@Integer": 2,
            "@Natural": 3,
            "@Nexists": 4,
            "@Rational": 5,
            "@Tensor": 6,
            "@Undefined": 0,
            "@Variable": 7,
            "@Whole": 8
        }[self.value],
        width = 4
    )

#> TYPES -> VEC
@dataclass(frozen = True)
class Vec(Generic[Type]):
    values: list[Binary | BinaryEncodable]
    def binary(self) -> Binary:
        value = Binary(
            value = len(self.values),
            width = 32
        )
        for item in self.values: value += item
        return value