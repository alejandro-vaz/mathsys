#^
#^  HEAD
#^

#> HEAD -> MODULES
from os import fdopen, remove, path
from tempfile import mkstemp, gettempdir
from asyncio import create_subprocess_exec, subprocess

#> HEAD -> DATA
from .targets import TARGETS, filename, environment, read, optsizations, original
from .issues import FailedCompilation


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
        descriptor, irfile = mkstemp(dir = gettempdir())
        self.configuration = environment(irfile, target, run)
        with fdopen(descriptor, "wb") as file: file.write(data)
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
                for command in optsizations(target, self.filename): await self.execute(*command)
            return await read(self.filename)
        finally: 
            remove(self.filename)
            remove(irfile)
    #~ CLASS -> EXECUTE
    async def execute(self, *command: str) -> None:
        process = await create_subprocess_exec(
            *command,
            cwd = path.join(path.dirname(__file__), ".."),
            env = self.configuration,
            stdout = subprocess.DEVNULL,
            stderr = None
        )
        await process.wait()
        if process.returncode != 0: raise FailedCompilation()