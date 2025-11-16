# Mathsys

![Pepy Total Downloads](https://img.shields.io/pepy/dt/mathsys?logo=pypi&label=Pypi%20downloads&link=https%3A%2F%2Fpypi.org%2Fproject%2Fmathsys%2F)
![NPM Downloads](https://img.shields.io/npm/dm/mathsys?logo=npm&label=NPM%20downloads&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2Fmathsys)

*Mathsys* is a *DSL* (*Domain-Specific Language*) aimed to make math writing easier on computers, and something machines can actually understand.

*Mathsys* bridges the gap between traditional mathematical notation and programming languages. It provides a hybrid syntax that maintains mathematical readability while adding programming language features like multi-character variables and structured expressions.

## Installation
Install the latest version via `pip`:

```sh
pip install mathsys
```

### Package
To use the package, simply import it:
```py
import mathsys
```

It is recommended that you import a version specifically, versions available now are `v1`, `v2` and `dev`:
```py
import mathsys.dev as mathsys
```

The latter way ensures you always use that version, no matter the updates that the package receives.

### CLI
Compile a Mathsys file to different targets with:

```sh
python -m mathsys <target> <filename>.msX
```

where `.msX` stands for `.ms1`, `.ms2` ... and `.msd`.

You will need `rustc` installed with the target you are compiling to. If that feels too cumbersome (it really is), try it out first on [Abscissa.](https://app.abscissa.eu/playground)

> [!NOTE]
> Targets available are: `validate`, `latex`, `web` and `unix-x86-64`.

## Project Status
Mathsys is actively developed with regular releases every 1-3 weeks. This project is still in its early stages, so expect major shifts and changes. Most features aren't close to being developed yet.

If you want to get involved in the project, [learn how to contribute](https://docs.abscissa.eu).

## Technical Background
- **Parser:** A [lark parser](https://github.com/lark-parser/lark) based on *Earley* that tokenizes the source and builds the *AST.*
- **LaTeX:** Our custom *LaTeX* generator that traverses the *AST* and outputs easy-to-read *LaTeX.*
- **IR:** A fully binary *Intermediate Representation.*
- **Runtime:** *Rust* based `no_std` runtime which interprets the *IR* embedded into it and implements control-flow for low-level operations.
- **Assembly:** For low-level operations that require speed and don't need memory safety.