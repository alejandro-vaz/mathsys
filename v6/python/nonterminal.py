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
    def create(self, items: list) -> None: pass
    @abstractmethod
    def latex(self, types: dict) -> str: pass
    @abstractmethod
    def ir(self, binary: list[Binary], nodes: list[Binary]) -> Pointer: pass
    def __setattr__(self, name: str, value: Any) -> None:
        if self.frozen: raise FrozenInstanceError()
        return super().__setattr__(name, value)
    def freeze(self) -> None: self.frozen = True
    def __init_subclass__(cls) -> None:
        super().__init_subclass__()
        def replacement(self, items: list) -> None: cls.create(self, items); self.freeze()
        cls.__init__ = replacement