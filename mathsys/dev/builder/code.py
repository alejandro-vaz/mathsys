#^
#^  HEAD
#^

#> HEAD -> MODULES
import os
import tempfile
import asyncio

#> HEAD -> DATA
from .local import TARGETS, filename, environment, read, optsizations, original


#^
#^  BUILDER
#^

#> BUILDER -> CLASS
class Builder:
    #~ CLASS -> VARIABLES
    filename: str
    configuration: dict
    #~ CLASS -> RUN
    async def run(self, data: bytes, target: str, optsize: bool, optimize: bool, run: frozenset) -> bytes:
        self.filename = filename(target, optimize)
        descriptor, irfile = tempfile.mkstemp(dir = tempfile.gettempdir())
        self.configuration = environment(irfile, target, run)
        with os.fdopen(descriptor, "wb") as file: file.write(data)
        try: 
            await self.execute(
                "cargo",
                "+nightly",
                "build",
                "--quiet",
                *(["--release"] if optimize else []),
                "--target", TARGETS[target],
                "-Zbuild-std=std,panic_abort"
            )
            await self.execute(
                "mv",
                original(target, optimize),
                self.filename
            )
            if optsize: 
                for command in optsizations(target, self.filename): 
                    try: await self.execute(*command)
                    except: print(f"Failed to run optimization: {command}")
            return await read(self.filename)
        finally: os.remove(irfile)
    #~ CLASS -> EXECUTE
    async def execute(self, *command: str) -> None:
        process = await asyncio.create_subprocess_exec(
            *command,
            cwd = os.path.join(os.path.dirname(__file__), ".."),
            env = self.configuration,
            stdout = asyncio.subprocess.DEVNULL,
            stderr = None
        )
        await process.wait()
        if process.returncode != 0: raise Exception