#
#   HEAD
#

# HEAD -> MODULES
import subprocess
import os
import tempfile


#
#   BUILDER
#

# BUILDER -> CLASS
class Builder:
    # CLASS -> VARIABLES
    filename: str
    data: bytes
    targets = {
        "unix/x86/64": "x86_64-unknown-linux-gnu",
        "web": "wasm32-unknown-unknown"
    }
    target: str
    debug: bool
    location: str
    # CLASS -> INIT
    def __init__(self, data: bytes, target: str) -> None:
        self.debug = False
        self.data = data
        if target not in self.targets: raise ValueError(f"'{target}' is not a supported target.")
        self.target = target
        self.location = os.path.dirname(os.path.abspath(__file__))
    # CLASS -> RUN
    def run(self) -> bytes:
        descriptor, ir = tempfile.mkstemp(dir = "/tmp", suffix = ".ir")
        with os.fdopen(descriptor, "bw") as file: file.write(self.data)
        descriptor, self.filename = tempfile.mkstemp(dir = "/tmp")
        os.close(descriptor)
        environment = os.environ.copy()
        environment["Mathsys"] = ir
        subprocess.run(
            self.command(),
            cwd = self.location,
            env = environment,
            capture_output = False,
            text = True,
            check = True
        )
        with open(self.filename, "rb") as file: binary = file.read()
        os.remove(self.filename)
        os.remove(ir)
        return binary
    # CLASS -> COMMAND CREATOR HELPER
    def command(self) -> list[str]:
        sysroot = subprocess.check_output(
            ["rustc", "+nightly", "--print", "sysroot"],
            text = True
        ).strip()
        return [
            "rustc",
            "+nightly",
            "../bin/main.rs",
            "--target", self.targets[self.target],
            "--sysroot", sysroot,
            "-C", f"opt-level={'0' if self.debug else '3'}",
            *(["-g"] if self.debug else []),
            "-C", "panic=abort",
            *(["-C", "link-arg=-nostartfiles"] if self.target == "unix/x86/64" else []),
            "-o", self.filename,
            "-C", f"link-arg=../bin/{self.target}/all.o"
        ]