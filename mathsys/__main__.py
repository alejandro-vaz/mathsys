#^
#^   MAIN
#^

#> MAIN -> MODULES
import sys
import asyncio

#> MAIN -> VERSIONS
from .v1 import wrapper as wrapper1
from .v2 import wrapper as wrapper2
from .v3 import wrapper as wrapper3
from .v4 import wrapper as wrapper4
from .v5 import wrapper as wrapper5
from .v6 import wrapper as wrapper6

#> MAIN -> LATEST
from .release import wrapper as wrapperRelease
from .dev import wrapper as wrapperDev

#> MAIN -> EXECUTION
def main() -> None:
    if len(sys.argv) == 3: 
        match sys.argv[2]:
            case value if value.endswith(".ms1"): wrapper1(*sys.argv[1:])
            case value if value.endswith(".ms2"): wrapper2(*sys.argv[1:])
            case value if value.endswith(".ms3"): wrapper3(*sys.argv[1:])
            case value if value.endswith(".ms4"): wrapper4(*sys.argv[1:])
            case value if value.endswith(".ms5"): asyncio.run(wrapper5(*sys.argv[1:]))
            case value if value.endswith(".ms6"): asyncio.run(wrapper6(*sys.argv[1:]))
            case value if value.endswith(".msr"): asyncio.run(wrapperRelease(*sys.argv[1:]))
            case value if value.endswith(".msd"): asyncio.run(wrapperDev(*sys.argv[1:]))
            case other: sys.exit("[ENTRY ISSUE] Invalid file extension.") 
    else: sys.exit("[ENTRY ISSUE] Usage: mathsys <target> <filename>.msX.") 