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
    def run(self, data: bytes, target: str) -> bytes:
        self.checks()
        descriptor, ir = tempfile.mkstemp(dir = "/tmp", suffix = ".ir")
        with os.fdopen(descriptor, "wb") as file: file.write(data)
        environment = os.environ.copy()
        environment["MathsysSource"] = ir
        environment["MathsysPrecision"] = "standard"
        environment["MathsysMajor"] = str(__version_info__[0])
        environment["MathsysMinor"] = str(__version_info__[1])
        environment["MathsysPatch"] = str(__version_info__[2])
        try: 
            subprocess.run(
                self.command(target),
                cwd = os.path.dirname(os.path.abspath(__file__)),
                env = environment,
                capture_output = False,
                text = True,
                check = True
            )
            if target == "wasm": subprocess.run(
                self.postwasm(),
                cwd = os.path.dirname(os.path.abspath(__file__)),
                env = environment,
                capture_output = False,
                text = True,
                check = True
            )
            with open(self.filename(target), "rb") as file: return file.read()
        except: raise
        finally: os.remove(ir)
    #~ CLASS -> COMMAND CREATOR HELPER
    def command(self, target: str) -> list[str]:
        return [
            "cargo",
            "+nightly",
            "build",
            "--release",
            "--target", TARGETS[target]
        ]
    #~ CLASS -> POSTWASM
    def postwasm(self) -> str:
        return [
            "wasm-opt",
            "-O3",
            "--strip-debug",
            "--enable-bulk-memory",
            "--dce",
            "--vacuum",
            "-o", self.filename("wasm"),
            self.filename("wasm")
        ]
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
    #~ CLASS -> CHECKS
    def checks(self) -> None:
        subprocess.run(
            ["rustc", "--version"],
            capture_output = False,
            text = True,
            check = True
        )