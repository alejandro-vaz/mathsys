#^
#^  HEAD
#^

#> HEAD -> MODULES
import sys

#> HEAD -> TIME
from time import time

#> HEAD -> CACHE
from functools import wraps
from async_lru import alru_cache

#> HEAD -> COMPILER
from .parser.code import Parser
from .latex.code import LaTeX
from .ir.code import IR
from .builder.code import Builder


#^
#^  PRELUDE
#^

#> PRELUDE -> CLASSES
_parser = Parser()
_latex = LaTeX()
_ir = IR()
_builder = Builder()

#> PRELUDE -> FUNCTIONS
async def functions() -> list:
    return [
        help,
        validate,
        binary,
        tokens,
        latex,
        native
    ]

#> PRELUDE -> TIME WRAPPER
def timeWrapper(function):
    @wraps(function)
    async def wrapper(*args, **kwargs):
        start = time()
        state = await function(*args, **kwargs)
        print(f"[INFO] Compiled to {function.__name__} in {(time() - start):.3f}s.")
        return state
    return wrapper

#> PRELUDE -> STATISTICS
async def statistics() -> list: return [function.cache_info() for function in await functions()]

#> PRELUDE -> CLEAR
async def clear() -> None: 
    for function in await functions(): function.cache_clear()


#^
#^  MAIN
#^

#> MAIN -> HELP
@alru_cache(maxsize = None)
async def help() -> str: return "\n".join(["- " + value.__name__.replace("_", "-") for value in await functions()])

#> MAIN -> VALIDATE
@alru_cache(maxsize = None)
async def validate(content: str) -> bool:
    try: _parser.run(content); return True
    except: return False

#> MAIN -> BINARY
@alru_cache(maxsize = None)
@timeWrapper
async def binary(content: str) -> bytes: return _ir.run(_parser.run(content))

#> MAIN -> TOKENS
@alru_cache(maxsize = None)
async def tokens(content: str) -> int: return len(_ir.run(_parser.run(content)))

#> MAIN -> LATEX
@alru_cache(maxsize = None)
@timeWrapper
async def latex(content: str) -> str: return _latex.run(_parser.run(content))

#> MAIN -> NATIVE
@alru_cache(maxsize = 8192)
@timeWrapper
async def native(content: str, optimize: bool, target: str): return _builder.run(_ir.run(_parser.run(content)), target, optimize)


#^
#^  TARGETS
#^

#> TARGETS -> WRAPPER
async def wrapper(*arguments: str) -> None: 
    #~ TARGET -> PREPROCESSING
    components = arguments[1].split(".")
    with open(arguments[1]) as origin: content = origin.read()
    #~ TARGET -> MATCHING
    match arguments[0]:
        case "help": print(f"Usage: mathsys <target> <filename>.msX\nAvailable targets:\n{await help()}")
        case "validate": print(await validate(content))
        case "binary": print(await binary(content))
        case "tokens": print(await tokens(content))
        case "latex": 
            components[-1] = "ltx"
            with open(".".join(components), "w") as destination:
                try: destination.write(await latex(content))
                except Exception as error: print(str(error))
        case "native": 
            components.pop()
            with open(".".join(components), "wb") as destination:
                try: destination.write(await native(content, True, "unix-x86-64"))
                except Exception as error: print(str(error))
        case other: sys.exit(f"[ENTRY ISSUE] Unknown target. Available targets:\n{await help()}")