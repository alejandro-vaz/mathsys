#^
#^  HEAD
#^

#> HEAD -> MODULES
from abc import ABC
from dataclasses import dataclass
from re import compile

#> HEAD -> DATA
from .issues import UnknownToken


#^
#^  TOKENS
#^

#> TOKENS -> PROTOTYPE
@dataclass(kw_only = True, frozen = True)
class Token(ABC):
    column: int
    line: int
    value: str
    def important(self) -> bool: return not self.__class__.__name__.startswith("_")
    def compilable(self) -> bool: return self.__class__.__name__ != "QUOTE"

#> TOKENS -> IMPORTANT
class IDENTIFIER(Token): pattern = r"[A-Za-zº$%]+"
class MODULE(Token): pattern = r"\"[a-z]+\""
class NUMBER(Token): pattern = r"[0-9]+"
class OPERATOR(Token): pattern = r"[\*\/]"
class QUOTE(Token): pattern = r"\#[^\n]*"
class RATIONAL(Token): pattern = r"[0-9]*\.[0-9]+"
class SIGN(Token): pattern = r"[+-]"
class TYPE(Token): pattern = r"\@(Infinite|Integer|Natural|Nexists|Rational|Tensor|Undefined|Variable|Whole)"

#> TOKENS -> UNIMPORTANT
class _BINDING(Token): pattern = r"=="
class _CLOSE(Token): pattern = r"\)"
class _COMMA(Token): pattern = r","
class _ENTER(Token): pattern = r"\["
class _EQUALITY(Token): pattern = r"="
class _EXIT(Token): pattern = r"\]"
class _EXPONENTIATION(Token): pattern = r"\^"
class _INF(Token): pattern = r"inf"
class _LIM(Token): pattern = r"lim"
class _NEWLINES(Token): pattern = r"\n+"
class _OF(Token): pattern = r"of"
class _OPEN(Token): pattern = r"\("
class _PIPE(Token): pattern = r"\|"
class _SPACES(Token): pattern = r" +"
class _TO(Token): pattern = r"->"
class _UNDEFINED(Token): pattern = r"\?"
class _USE(Token): pattern = r"use"

#> TOKENS -> EOF
class _EOF(Token): pattern = r"(?!)"

#> TOKENS -> ORDER
ORDER = [
    _UNDEFINED,
    _LIM,
    _PIPE,
    _TO,
    _OF,
    _INF,
    _USE,
    TYPE,
    IDENTIFIER,
    _EXPONENTIATION,
    RATIONAL,
    NUMBER,
    _BINDING,
    _EQUALITY,
    OPERATOR,
    SIGN,
    _OPEN,
    _CLOSE,
    _ENTER,
    _COMMA,
    _EXIT,
    _SPACES,
    _NEWLINES,
    MODULE,
    QUOTE,
    _EOF
]


#^
#^  TOKENIZER
#^

#> TOKENIZER -> CLASS
class Tokenizer:
    content: str
    column: int
    line: int
    left: str
    tokens: list[Token]
    def run(self, content: str) -> list[Token]:
        self.content = content
        self.column = 1
        self.line = 1
        self.left = content
        self.tokens = []
        while self.left: 
            token = self.next()
            self.tokens.append(token)
            if not isinstance(token, _NEWLINES): self.column += len(token.value)
            else: self.line += len(token.value); self.line = 0
            self.left = self.left[len(token.value):]
        self.tokens.append(_EOF(
            column = self.column,
            line = self.line,
            value = ""
        ))
        return self.tokens.copy()
    def next(self) -> Token:
        status = {}
        for token, pattern in {item: compile(item.pattern) for item in ORDER}.items():
            match = pattern.match(self.left)
            if match is None: continue
            status[token] = match.group(0)
        try: 
            at = max(len(string) for string in status.values())
            for token, match in status.items():
                if len(match) == at: return token(
                    column = self.column,
                    line = self.line,
                    value = match
                )
            assert False
        except ValueError: raise UnknownToken(self.line, self.column, self.content.split("\n")[self.line - 1])