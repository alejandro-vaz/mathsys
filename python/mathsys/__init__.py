#^
#^  EXPORTS
#^

#> EXPORTS -> VERSION
__version__ = "3.5.4"
__version_info__ = (3, 5, 4)

#> EXPORTS -> LATEST
from .v2 import (
    validate,
    latex,
    web,
    unix_x86_64,
    wrapper
)

#> EXPORTS -> PUBLIC API
__all__ = [
    "validate",
    "latex",
    "web",
    "unix_x86_64",
    "wrapper",
    "__version__",
    "__version_info__"
]
