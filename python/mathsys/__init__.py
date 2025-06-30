#
#   HEAD
#

# HEAD -> MODULES
import sys

# HEAD -> COMPILER
from .main.tokenizer import Tokenizer
from .main.parser import Parser

# HEAD -> DATACLASSES
from .main.parser import Program


#
#   MAIN
#

# MAIN -> TARGET
def target(filename: str) -> Program:
    with open(filename, "r") as file:
        return Parser(Tokenizer(file.read()).run(), filename.endswith(".calc")).parse()

# MAIN -> CONTENT
def compile(content: str, strict: bool) -> Program:
    return Parser(Tokenizer(content).run(), strict).parse()

# MAIN -> ENTRY POINT
if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("[ENTRY ISSUE] Usage: python compiler.py <filename>")
    print(target(sys.argv[1], sys.argv[1].endswith(".calc")))