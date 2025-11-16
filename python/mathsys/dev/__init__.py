#^
#^  HEAD
#^

#> HEAD -> MODULES
import sys
import time

#> HEAD -> CACHE
from functools import lru_cache

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
        print(f"[INFO] Compiled to {function.__name__} in {(time.time() - start):.3f}s.")
        return state
    return wrapper

#> MAIN -> STATISTICS
def statistics() -> list:
    return [
        validate.cache_info(),
        latex.cache_info(),
        web.cache_info(),
        unix_x86_64.cache_info()
    ]

#> MAIN -> CLEAR
def clear() -> None:
    validate.cache_clear()
    latex.cache_clear()
    web.cache_clear()
    unix_x86_64.cache_clear()

#> MAIN -> VALIDATE
@lru_cache(maxsize = None)
@timeWrapper
def validate(content: str) -> bool:
    try: _parser.run(content); return True
    except: return False

#> MAIN -> LATEX
@lru_cache(maxsize = None)
@timeWrapper
def latex(content: str) -> str: return _latex.run(_parser.run(content))

#> MAIN -> WEB
@lru_cache(maxsize = None)
@timeWrapper
def web(content: str) -> bytes: return _builder.run(_ir.run(_parser.run(content)), "web")

#> MAIN -> UNIX_X86_X64
@lru_cache(maxsize = None)
@timeWrapper
def unix_x86_64(content: str) -> bytes: return _builder.run(_ir.run(_parser.run(content)), "unix-x86-64")

#> MAIN -> TARGET
def wrapper(*arguments: str) -> None: 
    #~ TARGET -> PREPROCESSING
    components = arguments[1].split(".")
    with open(arguments[1]) as origin: content = origin.read()
    #~ TARGET -> MATCHING
    match arguments[0]:
        case "validate": print(validate(content))
        case "latex": 
            components[-1] = "ltx"
            with open(".".join(components), "w") as destination:
                try: destination.write(latex(content))
                except Exception as error: 
                    message = str(error)
                    print(message)
                    destination.write(message)
                    exit(1)
        case "web": 
            components[-1] = "wasm"
            with open(".".join(components), "wb") as destination:
                try: destination.write(web(content))
                except Exception as error: print(str(error)); exit(1)
        case "unix-x86-64": 
            components.pop()
            with open(".".join(components), "wb") as destination:
                try: destination.write(unix_x86_64(content))
                except Exception as error: print(str(error)); exit(1)
        case other: sys.exit("[ENTRY ISSUE] Unknown command. Available commands: validate, latex, web, unix-x86-64.")