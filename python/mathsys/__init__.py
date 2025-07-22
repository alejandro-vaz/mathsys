#
#   HEAD
#

# HEAD -> MODULES
import sys

# HEAD -> COMPILER
from .main.parser import Parser
from .main.generator import LaTeX

# HEAD -> SYNTAX
from .syntax.strict import syntax


#
#   MAIN
#

# MAIN -> COMPILE
def compile(content: str) -> str:
    return LaTeX().run(Parser(syntax).run(content))

# MAIN -> TARGET
def target(filename: str) -> str: 
    components = filename.split(".")
    components[-1] = "ltx"
    with open(".".join(components), "w") as destination:
        with open(filename, "r") as origin: 
            destination.write(compile(origin.read()))

# MAIN -> ENTRY POINT
if __name__ == "__main__":
    if len(sys.argv) == 2:
        target(sys.argv[1])
    else:
        sys.exit("[ENTRY ISSUE] Usage: python -m mathsys <filename>")