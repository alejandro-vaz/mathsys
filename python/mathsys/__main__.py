#
#   MAIN
#

# MAIN -> MODULES
import sys

# MAIN -> ENTRY POINT
from . import wrapper

# MAIN -> EXECUTION
if __name__ == "__main__":
    if len(sys.argv) == 3: 
        wrapper(*sys.argv[1:])
    else:
        sys.exit("[ENTRY ISSUE] Usage: python -m mathsys <target> <filename>.math.") 