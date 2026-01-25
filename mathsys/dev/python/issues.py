#^
#^  HEAD
#^

#> HEAD -> MODULES
from typing import Any
from rich import print


#^
#^  DEFAULTS
#^

#> DEFAULTS -> CODE
def Code(code: str) -> str: return f"[#AAAAAA]{code}[/]"

#> DEFAULTS -> PLACE
def Place(place: str) -> str: return f"[cyan]{place}[/]"

#> DEFAULTS -> ITEM
def Item(item: str) -> str: return f"[purple]{item}[/]"


#^
#^  ISSUES
#^

#> ISSUES -> ISSUE
class Issue(Exception):
    content: str
    def __init__(self, message: str) -> None:
        self.content = self.format(message)
        super().__init__(self.content)
    def consume(self) -> None: print(self.content); exit(1)
    def format(self, base: str) -> str: 
        return f"[bold][red]Raised [/]{Item(self.__class__.__name__)} [red]issue:\n>\n>[/] {base.replace("\n", "\n[red]>[/] ")}\n[red]>[/]\n[/]"

#> ISSUES -> UNKNOWN TOKEN
class UnknownToken(Issue):
    def __init__(self, line: int, column: int, code: str) -> None:
        pointer = ' ' * (column - 1) + Place('^')
        super().__init__(f"Unknown token at line {line}:\n\n    {Code(code)}\n    {pointer}")

#> ISSUES -> UNKNOWN TARGET
class UnknownTarget(Issue):
    def __init__(self, unknown: str, available: list) -> None:
        helpers = "\n".join(["- " + value.__name__.replace("_", "-") for value in available])
        super().__init__(f"Unknown target {Item(unknown)}.\n\nAvailable targets:\n{helpers}")

#> ISSUES -> NO FILE PROVIDED
class NoFileProvided(Issue):
    def __init__(self) -> None: super().__init__("No input file provided.")

#> ISSUES -> NO TARGET PROVIDED
class NoTargetProvided(Issue):
    def __init__(self, available: list) -> None:
        helpers = "\n".join(["- " + value.__name__.replace("_", "-") for value in available])
        super().__init__(f"Available targets:\n{helpers}")

#> ISSUES -> SYNTAX ERROR
class Syntax(Issue):
    def __init__(self) -> None: super().__init__("Syntax error.")

#> ISSUES -> FAILED COMPILATION
class FailedCompilation(Issue):
    def __init__(self) -> None: super().__init__("Compilation failed.")