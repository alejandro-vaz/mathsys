#^
#^  HEAD
#^

#> HEAD -> MODULES
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
    def consume(self) -> None: print(self.content); del self
    def format(self, base: str) -> str: 
        return f"[bold][red]Raised [/]{Item(self.__class__.__name__)} [red]issue:\n>\n>[/] {base.replace("\n", "\n[red]>[/] ")}\n[red]>[/]\n[/]"

#> ISSUES -> UNKNOWN TOKEN
class UnknownToken(Issue):
    def __init__(self, line: int, column: int, code: str) -> None:
        pointer = ' ' * (column - 1) + Place('^')
        message = f"Unknown token at line {line}:\n\n    {Code(code)}\n    {pointer}"
        super().__init__(message)

#> ISSUES -> UNKNOWN TARGET
class UnknownTarget(Issue):
    def __init__(self, unknown: str, available: list) -> None:
        helpers = "\n".join(["- " + value.__name__.replace("_", "-") for value in available])
        message = f"Unknown target {Item(unknown)}.\n\nAvailable targets:\n{helpers}"
        super().__init__(message)

#> ISSUES -> NO FILE PROVIDED
class NoFileProvided(Issue):
    def __init__(self) -> None:
        message = "No input file provided."
        super().__init__(message)

#> ISSUES -> NO TARGET PROVIDED
class NoTargetProvided(Issue):
    def __init__(self, available: list) -> None:
        helpers = "\n".join(["- " + value.__name__.replace("_", "-") for value in available])
        message = f"Available targets:\n{helpers}"
        super().__init__(message)

#> ISSUES -> BROKEN SYNTAX
class BrokenSyntax(Issue):
    def __init__(self) -> None:
        message = "Syntax error"
        super().__init__(message)