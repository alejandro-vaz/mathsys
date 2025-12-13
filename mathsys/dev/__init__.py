#^
#^  HEAD
#^

#> HEAD -> MODULES
import sys

#> HEAD -> TIME
from time import time

#> HEAD -> CACHE
from functools import lru_cache, cache, wraps

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
        wasm,
        unix_x86_64
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
async def help() -> str: return "\n".join(["- " + value.__name__.replace("_", "-") for value in await functions()])

#> MAIN -> VALIDATE
async def validate(content: str) -> bool:
    try: _parser.run(content); return True
    except: return False

#> MAIN -> BINARY
async def binary(content: str) -> bytes: return _ir.run(_parser.run(content))

#> MAIN -> TOKENS
async def tokens(content: str) -> int: return len(_ir.run(_parser.run(content)))

#> MAIN -> LATEX
@timeWrapper
async def latex(content: str) -> str: return _latex.run(_parser.run(content))

#> MAIN -> WEB
@timeWrapper
async def wasm(content: str, optimize: bool) -> bytes: return _builder.run(_ir.run(_parser.run(content)), "wasm", optimize)

#> MAIN -> UNIX_X86_X64
@timeWrapper
async def unix_x86_64(content: str, optimize: bool) -> bytes: return _builder.run(_ir.run(_parser.run(content)), "unix-x86-64", optimize)


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
        case "help": print(f"Available targets:\n{await help()}.")
        case "validate": print(await validate(content))
        case "binary": print(await binary(content))
        case "tokens": print(await tokens(content))
        case "latex": 
            components[-1] = "ltx"
            with open(".".join(components), "w") as destination:
                try: destination.write(await latex(content))
                except Exception as error: print(str(error))
        case "wasm": 
            components[-1] = "wasm"
            with open(".".join(components), "wb") as destination:
                try: destination.write(await wasm(content, True))
                except Exception as error: print(str(error))
        case "unix-x86-64": 
            components.pop()
            with open(".".join(components), "wb") as destination:
                try: destination.write(await unix_x86_64(content, True))
                except Exception as error: print(str(error))
        case other: sys.exit(f"[ENTRY ISSUE] Unknown target. Available targets:\n{await help()}.")