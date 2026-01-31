#^
#^  HEAD
#^

#> HEAD -> MODULES
from os import environ, path
from aiofiles import open
from string import ascii_lowercase
from random import choices

#> HEAD -> VERSION
from mathsys import __version__


#^
#^  STATIC
#^

#> STATIC -> TARGETS
TARGETS = {
    "unix-x86-64": "x86_64-unknown-linux-gnu",
    "wasm": "wasm32-unknown-unknown",
    "darwin": "aarch64-apple-darwin"
}

#> STATIC -> OPTIMIZATIONS
def optsizations(target: str, filename: str) -> list[list[str]]:
    return {
        "unix-x86-64": [[
            "upx",
            "--best",
            "--8-bit",
            "--8mib-ram",
            "--preserve-build-id",
            "--ultra-brute",
            "--force-overwrite",
            "-o", filename,
            filename
        ]],
        "wasm": [[
            "wasm-opt",
            "-O3",
            "--strip-debug",
            "--enable-simd",
            "--enable-multivalue",
            "--enable-bulk-memory",
            "--dce",
            "--enable-nontrapping-float-to-int",
            "--vacuum",
            "-o", filename,
            filename
        ]],
        "darwin": [[]]
    }[target]


#^
#^  HELPERS
#^

#> HELPERS -> ORIGINAL
def original(target: str, optimize: bool) -> str: 
    extension = {
        "unix-x86-64": "",
        "wasm": ".wasm",
        "darwin": ""
    }[target]
    return path.join(
        path.dirname(__file__),
        "..",
        "target",
        TARGETS[target],
        "release" if optimize else "debug",
        f"wrapper{extension}"
    )

#> HELPERS -> FILENAME
def filename(target: str, optimize: bool) -> str: 
    extension = {
        "unix-x86-64": "",
        "wasm": ".wasm",
        "darwin": ""
    }[target]
    return path.join(
        path.dirname(__file__),
        "..",
        "target",
        TARGETS[target],
        "release" if optimize else "debug",
        f"{''.join(choices(ascii_lowercase, k = 10))}{extension}"
    )

#> HELPERS -> ENVIRONMENT
def environment(irfile: str, target: str, run: frozenset) -> dict:
    environment = environ.copy()
    environment["RUSTFLAGS"] = "-Clink-arg=" + path.join(path.dirname(__file__), "..", "bin", f"{target}.o")
    for key, value in run:
        environment[f"Mathsys{key.capitalize()}"] = value if isinstance(value, str) else str(value).lower()
    environment["MathsysIr"] = irfile
    return environment

#> HELPERS -> READ
async def read(filename: str) -> bytes:
    async with open(filename, "rb") as file: return await file.read()