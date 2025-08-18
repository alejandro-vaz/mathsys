#
#   HEAD
#

# HEAD -> MODULES
import sys

# HEAD -> COMPILER
from .main.parser import Parser
from .main.latex import LaTeX
from .main.ir import IR
from .main.builder import Builder

# HEAD -> SYNTAX
from .syntax.strict import syntax


#
#   MAIN
#

# MAIN -> VALIDATE
def validate(content: str) -> bool:
    try: 
        view(content)
        return True
    except:
        return False

# MAIN -> VIEW
def view(content: str) -> str:
    return LaTeX().run(Parser(syntax).run(content))

# MAIN -> COMPILE
def compile(content: str, target: str) -> bytes:
    return Builder(IR().run(Parser(syntax).run(content)), target).run()

# MAIN -> TARGET
def wrapper(filename: str, target: str) -> None: 
    components = filename.split(".")
    components[-1] = "ltx"
    with open(filename) as origin:
        content = origin.read()
        with open(".".join(components), "w") as destination:
            destination.write(view(content))
        match target:
            case "unix/x86/64": components.pop()
            case "web": components[-1] = "wasm"
        with open(".".join(components), "wb") as destination:
            destination.write(compile(content, target))