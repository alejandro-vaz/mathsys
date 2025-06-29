#
#   HEAD
#

# HEAD -> MODULES
from dataclasses import dataclass
import re
import sys


#
#   DATACLASSES
#

# DATACLASSES -> TOKEN
@dataclass
class Token:
    datatype: str
    value: any
    position: list[int]


#
#   TOKENIZER
#

# TOKENIZER -> CLASS
class Tokenizer:
    # CLASS -> VARIABLES
    code: str
    tokens: list[Token | None]
    position: list[int]
    lineStart: int
    index: int
    regex: re.Pattern
    # CLASS -> TOKENS
    spec = {
        "KEYWORD": r"Num(?:&int|&float)?",
        "SPACE": r" +",
        "IDENTIFIER": r"[A-Za-z]+",
        "EQUALITY": r"=",
        "NUMBER": r"[0-9]+(\.[0-9]+)?",
        "SIGNS": r"[+-](\s*[+-])*",
        "LPAREN": r"\(",
        "RPAREN": r"\)",
        "NEWLINE": r"\n+",
        "MISMATCH": r".",
    }
    # CLASS -> NEW ITEM
    def __init__(self: object, code: str) -> None:
        self.code = code
        self.tokens = []
        self.position = [1, 0]
        self.lineStart = 0
        self.index = 0
        self.regex = re.compile("|".join(f"(?P<{name}>{pattern})" for name, pattern in self.spec.items()))
    # CLASS -> TOKENIZER
    def run(self: object) -> list[Token]:
        while self.index < len(self.code):
            self.tokens.append(self.tokenMatch(self.regex.match(self.code, self.index)))
        return [token for token in self.tokens if token is not None]
    # CLASS -> TOKEN MATCHER
    def tokenMatch(self: object, pseudoToken: re.Match) -> Token | None:
        kind = pseudoToken.lastgroup
        value = pseudoToken.group(kind)
        self.index = pseudoToken.end()
        match kind:
            case "MISMATCH":
                sys.exit(f"[TOKENIZER ISSUE] Unexpected character {value!r} on line {self.position[0]}")
            case "SPACE":
                return None
            case "NEWLINE":
                self.position[0] += 1
                self.lineStart = pseudoToken.end()
                return Token(kind, len(value), self.position)
            case "EQUALITY" | "LPAREN" | "RPAREN":
                return Token(kind, None, [self.position[0], pseudoToken.start() - self.lineStart + 1])
            case _:
                return Token(kind, value, [self.position[0], pseudoToken.start() - self.lineStart + 1])