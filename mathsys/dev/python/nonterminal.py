#^
#^  HEAD
#^

#> HEAD -> MODULES
from abc import ABC, abstractmethod
from dataclasses import FrozenInstanceError
from typing import Any

#> HEAD -> DATA
from .ir import Binary, Pointer


#^
#^  NONTERMINAL
#^

#> NONTERMINAL -> START
class NonTerminal(ABC):
    frozen: bool = False
    @abstractmethod
    def __init__(self, items: list) -> None: pass
    @abstractmethod
    def latex(self, types: dict[str, str]) -> str: pass
    @abstractmethod
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: pass
    def freeze(self) -> None: self.frozen = True
    @classmethod
    def placeholder(cls) -> bool: return cls.__class__.__name__.startswith("Level")
    def __setattr__(self, name: str, value: Any) -> None:
        if self.frozen: raise FrozenInstanceError()
        return super().__setattr__(name, value)