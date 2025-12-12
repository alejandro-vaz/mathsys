#^
#^  EXPORTS
#^

#> EXPORTS -> VERSION
__version__ = "4.2.0"
__version_info__ = tuple([int(number) for number in __version__.split(".")])

#> EXPORTS -> LATEST
from .release import *