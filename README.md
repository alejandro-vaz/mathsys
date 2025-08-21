# Mathsys

![Pepy Total Downloads](https://img.shields.io/pepy/dt/mathsys?logo=pypi&label=Pypi%20downloads&link=https%3A%2F%2Fpypi.org%2Fproject%2Fmathsys%2F)
![NPM Downloads](https://img.shields.io/npm/dm/mathsys?logo=npm&label=NPM%20downloads&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2Fmathsys)
![GitHub Tag](https://img.shields.io/github/v/tag/alejandro-vaz/mathsys?include_prereleases&logo=github&label=Latest%20version&link=https%3A%2F%2Fgithub.com%2Falejandro-vaz%2Fmathsys%2Freleases%2Flatest)

*Mathsys* is a *DSL* (*Domain-Specific Language*) aimed to make math writing easier on computers, and something machines can actually understand.

*Mathsys* bridges the gap between traditional mathematical notation and programming languages. It provides a hybrid syntax that maintains mathematical readability while adding programming language features like multi-character variables and structured expressions.

## Installation
Install the latest version via pip:

```sh
pip install mathsys
```

### CLI
Compile a Mathsys file to different targets with:

```sh
python -m mathsys <filename> <target>
```

You will need `rustc` installed with the target you are compiling to. If that feels too cumbersome (it really is), try it out first on [Abscissa.](https://abscissa.eu/playground)

> [!NOTE]
> Compiling via command line will also output a `.ltx` file with the *LaTeX.*

## Project Status
Mathsys is actively developed with regular releases every 1-3 weeks. This project is still in its early stages, so expect major shifts and changes. Most features aren't close to being developed yet.

I say *we* but I'm indeed a solo developer for now, so if you want to get involved learn

## How to contribute
1. **Check our docs:** [documentation.](https://github.com/abscissa-math/mathsys/wiki)
2. **Join the team:** Contact us to become a member and get write access.
3. **Work on a branch:** Create a new branch for your changes (`main` is protected).
4. **Submit PR:** Create a pull request with updated changelog and version numbers.

> [!NOTE]
> If there's already a branch developing the next version, branch from that instead of branching from `main`.

## Technical Background
- **Parser:** A [lark parser](https://github.com/lark-parser/lark) based on *Earley* that tokenizes the source and builds the *AST.*
- **LaTeX:** Our custom *LaTeX* generator that traverses the *AST* and outputs easy-to-read *LaTeX.*
- **IR:** A fully binary *Intermediate Representation.*
- **Runtime:** *Rust* based `no_std` runtime which interprets the *IR* embedded into it and implements control-flow for low-level operations.
- **Assembly:** For low-level operations which require speed and don't need memory safety.

## License
All rights reserved. See [LICENSE.md](LICENSE.md) for details.