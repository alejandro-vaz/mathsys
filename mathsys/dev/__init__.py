#^
#^  HEAD
#^

#> HEAD -> MODULES
from functools import wraps
from async_lru import alru_cache
from time import time
from dataclasses import dataclass
from platform import system
import cProfile

#> HEAD -> CLI
from ..entry import File, Flag, Alias, Target

#> HEAD -> VERSION
from .. import __version__

#> HEAD -> COMPILER
from .python.tokenizer import Tokenizer, Token
from .python.parser import Parser
from .python.ir import Binary
from .python.builder import Builder

#> HEAD -> START
from .python.start import Start

#> HEAD -> ISSUES
from .python.issues import Issue, UnknownTarget, NoFileProvided, NoTargetProvided

#> HEAD -> PARSER
_tokenizer = Tokenizer().run
_parser = Parser().run
_builder = Builder().run


#^
#^  PRELUDE
#^

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
async def statistics() -> list: return [function.cache_info() for function in FUNCTIONS]

#> PRELUDE -> CLEAR
async def clear() -> None: 
    for function in FUNCTIONS: function.cache_clear()


#^
#^  MAIN
#^

#> MAIN -> TOKENS
@alru_cache(maxsize = None)
async def tokens(content: str) -> list[Token]: return _tokenizer(content)

#> MAIN -> AST
@alru_cache(maxsize = None)
@timeWrapper
async def ast(content: str) -> Start: return _parser(_tokenizer(content))

##> MAIN -> VALIDATE
@alru_cache(maxsize = None)
async def valid(content: str) -> bool:
    try: await ast(content); return True
    except Issue: return False

##> MAIN -> BINARY
@alru_cache(maxsize = None)
async def binary(content: str) -> bytes: 
    binary = []
    (await ast(content)).ir(binary)
    return bytes(sum(binary, start = Binary(
        value = 0,
        width = 0
    )))

#> MAIN -> SIZE
@alru_cache(maxsize = None)
async def size(content: str) -> int: return len(await binary(content))

##> MAIN -> LATEX
@alru_cache(maxsize = None)
async def latex(content: str) -> str: return (await ast(content)).latex()

##> MAIN -> UNIX-X86-64
@alru_cache(maxsize = 8129)
@timeWrapper
async def unix_x86_64(content: str, optsize: bool, optimize: bool, run: frozenset) -> bytes: return await _builder(
    await binary(content),
    "unix-x86-64", 
    optsize,
    optimize,
    run
)

#> MAIN -> WASM
@alru_cache(maxsize = 8192)
@timeWrapper
async def wasm(content: str, optsize: bool, optimize: bool, run: frozenset) -> bytes: return await _builder(
    await binary(content), 
    "wasm", 
    optsize,
    optimize,
    run
)

#> MAIN -> DARWIN
@alru_cache(maxsize = 8192)
@timeWrapper
async def darwin(content: str, optsize: bool, optimize: bool, run: frozenset) -> bytes: return await _builder(
    await binary(content),
    "darwin",
    optsize,
    optimize,
    run
)

#> MAIN -> NATIVE
@alru_cache(maxsize = 8192)
@timeWrapper
async def native(content: str, optsize: bool, optimize: bool, run: frozenset) -> bytes: return await _builder(
    await binary(content),
    "darwin" if system().lower() == "darwin" else "unix-x86-64",
    optsize,
    optimize,
    run
)


#^
#^  TARGETS
#^

#> TARGETS -> SETTINGS
@dataclass(kw_only = True)
class Settings:
    file: File
    target: Target
    optsize = False
    optimize = False
    run = {
        "ir": "",
        "version": __version__,
        "debug": False,
        "class": False,
        "chore": True,
        "trace": True,
        "alert": False,
        "point": True
    }
    def apply(self, argument: File | Flag | Alias | Target) -> None:
        match argument:
            case File(): pass
            case Flag():
                match argument.value:
                    case "optsize": self.optsize = True
                    case "optimize": self.optimize = True
                    case "debug": self.run["debug"] = True
                    case "class": self.run["class"] = True
                    case "no-chore": self.run["chore"] = False
                    case "no-trace": self.run["trace"] = False
                    case "alert": self.run["alert"] = True
                    case "no-point": self.run["point"] = False
            case Alias():
                for letter in argument.letters:
                    match letter.lower():
                        case "z": self.apply(Flag(value = "optsize"))
                        case "o": self.apply(Flag(value = "optimize"))
                        case "d": self.apply(Flag(value = "debug"))
                        case "c": self.apply(Flag(value = "class"))
                        case "h": self.apply(Flag(value = "no-chore"))
                        case "t": self.apply(Flag(value = "no-trace"))
                        case "a": self.apply(Flag(value = "alert"))
                        case "p": self.apply(Flag(value = "no-point"))
            case Target(): pass
    async def content(self) -> str: return await self.file.read()

#> TARGETS -> WRAPPER
async def wrapper(*arguments: File | Flag | Alias | Target) -> None: 
    try:
        file = None
        target = None
        for argument in arguments:
            if file is None and isinstance(argument, File): file = argument
            if target is None and isinstance(argument, Target): target = argument
        if file is None: raise NoFileProvided()
        if target is None: raise NoTargetProvided(FUNCTIONS)
        settings = Settings(
            file = file,
            target = target
        )
        for argument in arguments: settings.apply(argument)
        match settings.target.name:
            case "tokens": print(await tokens(
                await settings.content()
            ))
            case "ast": print(await ast(
                await settings.content()
            ))
            case "validate": print(await valid(
                await settings.content()
            ))
            case "binary": print(await binary(
                await settings.content()
            ))
            case "size": print(await size(
                await settings.content()
            ))
            case "latex": print(await latex(
                await settings.content()
            ))
            case "unix-x86-64": await settings.file.write("", await unix_x86_64(
                await settings.content(),
                settings.optsize,
                settings.optimize,
                frozenset(settings.run.items())
            ))
            case "wasm": await settings.file.write("wasm", await wasm(
                await settings.content(),
                settings.optsize,
                settings.optimize,
                frozenset(settings.run.items())
            ))
            case "darwin": await settings.file.write("", await darwin(
                await settings.content(),
                settings.optsize,
                settings.optimize,
                frozenset(settings.run.items())
            ))
            case "native": await settings.file.write("", await native(
                await settings.content(),
                settings.optsize,
                settings.optimize,
                frozenset(settings.run.items())
            ))
            case other: raise UnknownTarget(other, FUNCTIONS)
    #except Issue as issue: issue.consume()
    except: raise

#> TARGETS -> FUNCTIONS
FUNCTIONS = [
    tokens,
    ast,
    valid,
    binary,
    size,
    latex,
    unix_x86_64,
    wasm,
    darwin,
    native
]