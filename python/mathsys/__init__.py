#^
#^  EXPORTS
#^

#> EXPORTS -> VERSION
__version__ = "3.7.40"
__version_info__ = (3, 7, 0)

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
    "__version__",
    "__version_info__",
    "validate",
    "latex",
    "web",
    "unix_x86_64",
    "wrapper"
]
