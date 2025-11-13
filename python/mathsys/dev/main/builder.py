#^
#^  HEAD
#^

#> HEAD -> MODULES
import subprocess
import os
import tempfile
from functools import lru_cache


#^
#^  STATIC
#^

#> STATIC -> TARGETS
TARGETS = {
    "unix-x86-64": "x86_64-unknown-linux-gnu",
    "web": "wasm32-unknown-unknown"
}


#^
#^  BUILDER
#^

#> BUILDER -> CLASS
class Builder:
    #~ CLASS -> RUN
    @lru_cache(maxsize = None)
    def run(self, data: bytes, target: str) -> bytes:
        try:
            self.checks()
            descriptor, ir = tempfile.mkstemp(dir = "/tmp", suffix = ".ir")
            with os.fdopen(descriptor, "wb") as file: file.write(data)
            descriptor, filename = tempfile.mkstemp(dir = "/tmp")
            os.close(descriptor)
            environment = os.environ.copy()
            environment["Mathsys"] = ir
            try: subprocess.run(
                self.command(target, filename),
                cwd = os.path.dirname(os.path.abspath(__file__)),
                env = environment,
                capture_output = False,
                text = True,
                check = True
            )
            except: pass
            with open(filename, "rb") as file: binary = file.read()
            os.remove(filename)
            os.remove(ir)
            return binary
        except Exception: raise
    #~ CLASS -> COMMAND CREATOR HELPER
    def command(self, target: str, filename: str) -> list[str]:
        return [
            "rustc",
            "+nightly",
            "../bin/main.rs",
            "--target", TARGETS[target],
            "--sysroot", subprocess.check_output(
                ["rustc", "+nightly", "--print", "sysroot"],
                text = True
            ).strip(),
            "-C", f"opt-level=3",
            "-C", "panic=abort",
            *(["-C", "link-arg=-nostartfiles"] if target == "unix-x86-64" else []),
            "-o", filename,
            "-C", f"link-arg=../source/{target}.o"
        ]
    #~ CLASS -> CHECKS
    def checks(self) -> None:
        subprocess.run(
            ["rustc", "--version"],
            capture_output = False,
            text = True,
            check = True
        )