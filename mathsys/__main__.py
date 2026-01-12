#^
#^   MAIN
#^

#> MAIN -> MODULES
from sys import argv
from asyncio import run
from pathlib import Path

#> MAIN -> STRUCTURES
from .entry import File, Flag, Alias, Target

#> MAIN -> EXECUTION
def main() -> None:
    arguments = []
    for argument in argv[1:]:
        match argument:
            case file if Path(file).suffix.startswith(".ms"): arguments.append(File(
                name = Path(file)
            ))
            case flag if flag.startswith("--"): arguments.append(Flag(
                value = flag[2:]
            ))
            case alias if alias.startswith("-"): arguments.append(Alias(
                letters = alias[1:]
            ))
            case target: arguments.append(Target(
                name = target
            ))
    for argument in arguments:
        if isinstance(argument, File): call(argument.name.suffix[-1], arguments); exit(0)
    call("r", arguments)

#> MAIN -> CALL
def call(version: str, arguments: list[File | Flag | Alias | Target]) -> None:
    match version:
        case "1": from .v1 import wrapper; wrapper(*argv[1:])
        case "2": from .v2 import wrapper; wrapper(*argv[1:])
        case "3": from .v3 import wrapper; wrapper(*argv[1:])
        case "4": from .v4 import wrapper; wrapper(*argv[1:])
        case "5": from .v5 import wrapper; run(wrapper(*argv[1:]))
        case "6": from .v6 import wrapper; run(wrapper(*arguments))
        case "7": from .v7 import wrapper; run(wrapper(*arguments))
        case "r": from .release import wrapper; run(wrapper(*arguments))
        case "d": from .dev import wrapper; run(wrapper(*arguments))
        case other: pass