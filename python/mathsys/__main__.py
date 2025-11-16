#^
#^   MAIN
#^

#> MAIN -> MODULES
import sys

#> MAIN -> ENTRY POINT
from .v1 import wrapper as wrapper1
from .v2 import wrapper as wrapper2
from .dev import wrapper as wrapperDev

#> MAIN -> EXECUTION
if __name__ == "__main__":
    if len(sys.argv) == 3: 
        match sys.argv[2]:
            case value if value.endswith(".ms1"): wrapper1(*sys.argv[1:])
            case value if value.endswith(".ms2"): wrapper2(*sys.argv[1:])
            case value if value.endswith(".msd"): wrapperDev(*sys.argv[1:])
            case other: sys.exit("[ENTRY ISSUE] Usage: python -m mathsys <target> <filename>.msX.") 
    else: sys.exit("[ENTRY ISSUE] Usage: python -m mathsys <target> <filename>.msX.") 