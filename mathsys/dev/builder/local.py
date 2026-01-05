#^
#^  HEAD
#^

#> HEAD -> MODULES
import os
import aiofiles
import string
import random

#> HEAD -> VERSION
from mathsys import __version__


#^
#^  STATIC
#^

#> STATIC -> TARGETS
TARGETS = {
    "unix-x86-64": "x86_64-unknown-linux-gnu",
    "wasm": "wasm32-unknown-unknown"
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
        ]]
    }[target]


#^
#^  HELPERS
#^

#> HELPERS -> ORIGINAL
def original(target: str, optimize: bool) -> str: 
    extension = {
        "unix-x86-64": "",
        "wasm": ".wasm"
    }[target]
    return os.path.join(
        os.path.dirname(__file__),
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
        "wasm": ".wasm"
    }[target]
    return os.path.join(
        os.path.dirname(__file__),
        "..",
        "target",
        TARGETS[target],
        "release" if optimize else "debug",
        f"{''.join(random.choice(string.ascii_lowercase))}{extension}"
    )

#> HELPERS -> ENVIRONMENT
def environment(irfile: str, target: str, run: frozenset) -> dict:
    environment = os.environ.copy()
    environment["RUSTFLAGS"] = "-Clink-arg=" + os.path.join(os.path.dirname(__file__), "..", "bin", f"{target}.o")
    for key, value in run:
        environment[f"Mathsys{key.capitalize()}"] = value if isinstance(value, str) else str(value).lower()
    environment["MathsysIr"] = irfile
    return environment

#> HELPERS -> READ
async def read(filename: str) -> bytes:
    async with aiofiles.open(filename, "rb") as file: return await file.read()