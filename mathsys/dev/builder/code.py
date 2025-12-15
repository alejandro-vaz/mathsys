#^
#^  HEAD
#^

#> HEAD -> MODULES
import os
import subprocess
import tempfile

#> HEAD -> VERSION
from mathsys import __version_info__

#> HEAD -> DATA
from .local import TARGETS, EXTENSIONS


#^
#^  BUILDER
#^

#> BUILDER -> CLASS
class Builder:
    #~ CLASS -> RUN
    def run(self, data: bytes, target: str, optimize: bool) -> bytes:
        descriptor, ir = tempfile.mkstemp(dir = "/tmp", suffix = ".ir")
        with os.fdopen(descriptor, "wb") as file: file.write(data)
        environment = self.config(ir)
        try: 
            self.execute([
                "cargo",
                "+nightly",
                "build",
                *(["--release"] if optimize else []),
                "--target", TARGETS[target]
            ], environment)
            if optimize: 
                for command in self.post(target):
                    try: self.execute(command, environment)
                    except: pass
            with open(self.filename(target), "rb") as file: return file.read()
        except: raise
        finally: os.remove(ir)
    #~ CLASS -> POSTWASM
    def post(self, target: str) -> list[list[str]]:
        match target:
            case "wasm": return [[
                "wasm-opt",
                "-O3",
                "--strip-debug",
                "--enable-simd",
                "--enable-multivalue",
                "--enable-bulk-memory",
                "--dce",
                "--vacuum",
                "-o", self.filename("wasm"),
                self.filename("wasm")
            ]]
            case "unix-x86-64": return [[
                "upx",
                "--best",
                "--ultra-brute",
                "--force-overwrite",
                "-o", self.filename("unix-x86-64"),
                self.filename("unix-x86-64")
            ]]
    #~ CLASS -> FILENAME
    def filename(self, target: str) -> str:
        return os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            "..",
            "target",
            TARGETS[target],
            "release",
            f"mathsys{EXTENSIONS[target]}"
        )
    #~ CLASS -> EXECUTE
    def execute(self, command: list[str], env: dict) -> None: subprocess.run(
        command,
        cwd = os.path.dirname(os.path.join(os.path.dirname(__file__), "..")),
        env = env,
        capture_output = False,
        text = True,
        check = True
    )
    #~ CLASS -> CONFIGURATION
    def config(self, ir: bytes) -> dict:
        env = os.environ.copy()
        env["MathsysSource"] = ir
        env["MathsysMajor"] = str(__version_info__[0])
        env["MathsysMinor"] = str(__version_info__[1])
        env["MathsysPatch"] = str(__version_info__[2])
        return env