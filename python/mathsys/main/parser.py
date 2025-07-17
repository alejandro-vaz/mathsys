#
#   HEAD
#

# HEAD -> MODULES
from __future__ import annotations
from dataclasses import dataclass
from lark import Lark, Transformer, Token


#
#   1ºLEVEL
#

# 1ºLEVEL -> NAMESPACE
class Level1: pass

# 1ºLEVEL -> SHEET
@dataclass
class Sheet(Level1):
    statements: list[Level2]


#
#   2ºLEVEL
#

# 2ºLEVEL -> NAMESPACE
class Level2: pass

# 2ºLEVEL -> DECLARATION
@dataclass
class Declaration(Level2):
    keyword: str
    identifier: str
    expression: Expression


#
#   3ºLEVEL
#

# 3ºLEVEL -> NAMESPACE
class Level3: pass

# 3ºLEVEL -> EXPRESSION
@dataclass
class Expression(Level3):
    terms: list[Term | Brackets | Variable]


#
#   4ºLEVEL
#

# 4ºLEVEL -> NAMESPACE
class Level4: pass

# 4ºLEVEL -> TERM
@dataclass
class Term(Level4):
    signs: str
    number: str

# 4ºLEVEL -> VARIABLE
@dataclass
class Variable(Level4):
    signs: str
    identifier: str

# 4ºLEVEL -> BRACKETS
@dataclass
class Brackets(Level4):
    signs: str
    expression: Expression


#
#   PARSER
#

# PARSER -> CLASS
class Parser(Transformer):
    syntax: str
    # CLASS -> INIT
    def __init__(self, syntax: str) -> None:
        self.syntax = syntax
        super()
    # CLASS -> RUN
    def run(self, content: str) -> Sheet:
        return self.transform(Lark(self.syntax, parser="earley", start="sheet").parse(content))
    # CLASS -> SHEET CONSTRUCT
    def sheet(self, items: list[Level2]) -> Sheet: 
        return Sheet(items)
    # CLASS -> DECLARATION CONSTRUCT
    def declaration(self, items: list[str | Expression]): 
        items.pop(2)
        return Declaration(*items)
    # CLASS -> EXPRESSION CONSTRUCT
    def expression(self, items: list[Term | Brackets | Variable]): 
        return Expression(items)
    # CLASS -> TERM CONSTRUCT
    def term(self, items: list[str]):
        return Term("", items[0]) if (len(items) == 1) else Term(*items)
    # CLASS -> VARIABLE CONSTRUCT
    def variable(self, items: list[str]):
        return Variable("", items[0]) if (len(items) == 1) else Variable(*items)
    # CLASS -> BRACKETS CONSTRUCT
    def brackets(self, items: list[str | Expression]):
        items.pop(len(items) - 3)
        items.pop(len(items) - 1)
        return Brackets(*items) if len(items) == 2 else Brackets("", *items)
    # CLASS -> TOKENS
    def KEYWORD(self, token: Token) -> str: return str(token)
    def IDENTIFIER(self, token: Token) -> str: return str(token)
    def NUMBER(self, token: Token) -> str: return str(token)
    def NEWLINE(self, token: Token) -> str: return str(token)
    def EQUALITY(self, token: Token) -> str: return str(token)
    def SIGNS(self, token: Token) -> str: return str(token)
    def OPEN(self, token: Token) -> str: return str(token)
    def CLOSE(self, token: Token) -> str: return str(token)
    def SPACE(self, token: Token) -> str: return str(token)