#^
#^  HEAD
#^

#> HEAD -> MODULES
import sys
import time

#> HEAD -> COMPILER
from .main.parser import Parser
from .main.latex import LaTeX
from .main.ir import IR
from .main.builder import Builder

#> HEAD -> SYNTAX
from .main.syntax import syntax


#^
#^  MAIN
#^

#> MAIN -> CLASSES
_parser = Parser(syntax)
_latex = LaTeX()
_ir = IR()
_builder = Builder()

#> MAIN -> TIME WRAPPER
def timeWrapper(function):
    def wrapper(*args, **kwargs):
        start = time.time()
        state = function(*args, **kwargs)
        print(f"[SUCCESS] Compiled in {(time.time() - start):.3f}s.")
        return state
    return wrapper

#> MAIN -> VALIDATE
@timeWrapper
def validate(content: str) -> bool:
    try: _parser.run(content); return True
    except: return False

#> MAIN -> LATEX
@timeWrapper
def latex(content: str) -> str: 
    return _latex.run(_parser.run(content))

#> MAIN -> WEB
@timeWrapper
def web(content: str) -> bytes: 
    return _builder.run(_ir.run(_parser.run(content)), "web")

#> MAIN -> UNIX_X86_X64
@timeWrapper
def unix_x86_64(content: str) -> bytes: 
    return _builder.run(_ir.run(_parser.run(content)), "unix-x86-64")

#> MAIN -> TARGET
def wrapper(*arguments: str) -> None: 
    #~ TARGET -> PREPROCESSING
    components = arguments[1].split(".")
    with open(arguments[1]) as origin: content = origin.read()
    #~ TARGET -> MATCHING
    match arguments[0]:
        case "watch":
            components[-1] = "ltx"
            try:
                while True:
                    with open(arguments[1]) as origin: content = origin.read()
                    with open(".".join(components), "w") as destination:
                        try: destination.write(latex(content))
                        except KeyboardInterrupt: raise
                        except: pass
                    time.sleep(0.2)
            except KeyboardInterrupt: pass
        case "validate": print(validate(content))
        case "latex": 
            components[-1] = "ltx"
            with open(".".join(components), "w") as destination:
                destination.write(latex(content))
        case "web": 
            components[-1] = "wasm"
            with open(".".join(components), "wb") as destination:
                destination.write(web(content))
        case "unix-x86-64": 
            components.pop()
            with open(".".join(components), "wb") as destination:
                destination.write(unix_x86_64(content))
        case _: sys.exit(f"[ENTRY ISSUE] Unknown command. Available commands: watch, validate, latex, web, unix-x86-64.")