#
#   EXPORTS
#

# EXPORTS -> VERSION
__version__ = "2.1.5"
__version_info__ = (2, 1, 5)

# EXPORTS -> LATEST
from .v1 import (
    validate,
    latex,
    web,
    unix_x86_64,
    wrapper
)

# EXPORTS -> PUBLIC API
__all__ = [
    "validate",
    "latex",
    "web",
    "unix_x86_64",
    "wrapper"
]