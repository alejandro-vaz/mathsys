#^
#^  EXPORTS
#^

#> EXPORTS -> VERSION
__version__ = "3.9.16"
__version_info__ = tuple([int(number) for number in __version__.split(".")])

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
