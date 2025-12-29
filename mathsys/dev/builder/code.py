#^
#^  HEAD
#^

#> HEAD -> MODULES
import os
import subprocess
import tempfile

#> HEAD -> VERSION
from mathsys import __version__

#> HEAD -> DATA
from .local import TARGETS, EXTENSIONS


#^
#^  BUILDER
#^

#> BUILDER -> CLASS
class Builder:
    #~ CLASS -> RUN
    def run(self, data: bytes, target: str, optimize: bool) -> bytes:
        descriptor, ir = tempfile.mkstemp(dir = tempfile.gettempdir(), suffix = ".ir")
        with os.fdopen(descriptor, "wb") as file: file.write(data)
        environment = self.config(ir, target)
        try: 
            self.execute([
                "cargo",
                "+nightly",
                "build",
                *(["--release"] if optimize else []),
                "--target", TARGETS[target],
                "-Zbuild-std=std,panic_abort"
            ], environment)
            if optimize: 
                for command in self.post(target, optimize):
                    try: self.execute(command, environment)
                    except: print(f"Failed to run optimization: {command}")
            with open(self.filename(target, optimize), "rb") as file: return file.read()
        finally: os.remove(ir)
    #~ CLASS -> POSTWASM
    def post(self, target: str, optimize: bool) -> list[list[str]]:
        match target:
            case "wasm": return [[
                "wasm-opt",
                "-O3",
                "--strip-debug",
                "--enable-simd",
                "--enable-multivalue",
                "--enable-bulk-memory",
                "--dce",
                "--enable-nontrapping-float-to-int",
                "--vacuum",
                "-o", self.filename("wasm", optimize),
                self.filename("wasm", optimize)
            ]]
            case "unix-x86-64": return [[
                "upx",
                "--best",
                "--ultra-brute",
                "--force-overwrite",
                "-o", self.filename("unix-x86-64", optimize),
                self.filename("unix-x86-64", optimize)
            ]]
        return NotImplemented
    #~ CLASS -> FILENAME
    def filename(self, target: str, optimize: bool) -> str:
        return os.path.join(
            os.path.dirname(__file__),
            "..",
            "target",
            TARGETS[target],
            "release" if optimize else "debug",
            f"wrapper{EXTENSIONS[target]}"
        )
    #~ CLASS -> EXECUTE
    def execute(self, command: list[str], env: dict) -> None: subprocess.run(
        command,
        cwd = os.path.join(os.path.dirname(__file__), ".."),
        env = env,
        capture_output = False,
        text = True,
        check = True
    )
    #~ CLASS -> CONFIGURATION
    def config(self, ir: str, target: str) -> dict:
        env = os.environ.copy()
        env["MathsysSource"] = ir
        env["MathsysVersion"] = __version__
        env["RUSTFLAGS"] = "-Clink-arg=" + os.path.join(os.path.dirname(__file__), "..", "bin", f"{target}.o")
        return env