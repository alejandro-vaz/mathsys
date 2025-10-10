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
    targets = {
        "unix-x86-64": "x86_64-unknown-linux-gnu",
        "web": "wasm32-unknown-unknown"
    }
    # CLASS -> RUN
    def run(self, data: bytes, target: str) -> bytes:
        descriptor, ir = tempfile.mkstemp(dir = "/tmp", suffix = ".ir")
        with os.fdopen(descriptor, "bw") as file: file.write(data)
        descriptor, filename = tempfile.mkstemp(dir = "/tmp")
        os.close(descriptor)
        environment = os.environ.copy()
        environment["Mathsys"] = ir
        subprocess.run(
            self.command(target, filename),
            cwd = os.path.dirname(os.path.abspath(__file__)),
            env = environment,
            capture_output = False,
            text = True,
            check = True
        )
        with open(filename, "rb") as file: binary = file.read()
        os.remove(filename)
        os.remove(ir)
        return binary
    # CLASS -> COMMAND CREATOR HELPER
    def command(self, target: str, filename: str) -> list[str]:
        sysroot = subprocess.check_output(
            ["rustc", "+nightly", "--print", "sysroot"],
            text = True
        ).strip()
        return [
            "rustc",
            "+nightly",
            "../bin/main.rs",
            "--target", self.targets[target],
            "--sysroot", sysroot,
            "-C", f"opt-level=3",
            "-C", "panic=abort",
            *(["-C", "link-arg=-nostartfiles"] if target == "unix-x86-64" else []),
            "-o", filename,
            "-C", f"link-arg=../source/{target}.o"
        ]