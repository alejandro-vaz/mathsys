#^
#^  HEAD
#^

#> HEAD -> MODULES
from dataclasses import dataclass
from pathlib import Path
import aiofiles
from typing import cast, Any


#^
#^  INPUT
#^

#> INPUT -> FILE
@dataclass(kw_only = True, frozen = True)
class File:
    name: Path
    async def read(self) -> str:
        async with aiofiles.open(self.name) as file: return await file.read()
    async def write(self, extension: str, content: str | bytes) -> None:
        async with aiofiles.open(self.name.with_suffix(extension), mode = "wb" if isinstance(content, bytes) else "w") as file: 
            await file.write(cast(Any, content))

#> INPUT -> FLAG
@dataclass(kw_only = True, frozen = True)
class Flag:
    value: str

#> INPUT -> ALIAS
@dataclass(kw_only = True, frozen = True)
class Alias:
    letters: str

#> INPUT -> TARGET
@dataclass(kw_only = True, frozen = True)
class Target:
    name: str