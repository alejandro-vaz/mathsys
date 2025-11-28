#^
#^  EXPORTS
#^

#> EXPORTS -> VERSION
__version__ = "4.2.0"
__version_info__ = tuple([int(number) for number in __version__.split(".")])

#> EXPORTS -> LATEST
from .v3 import (
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
