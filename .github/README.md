# Mathsys

![Pepy Total Downloads](https://img.shields.io/pepy/dt/mathsys?logo=pypi&label=Pypi%20downloads&link=https%3A%2F%2Fpypi.org%2Fproject%2Fmathsys%2F)
![NPM Downloads](https://img.shields.io/npm/dm/mathsys?logo=npm&label=NPM%20downloads&link=https%3A%2F%2Fwww.npmjs.com%2Fpackage%2Fmathsys)
![Crates.io Total Downloads](https://img.shields.io/crates/d/mathsys)

Mathsys is an educational math language designed to help students and teachers write, read and experiment with mathematics in a clear, human-readable way.

It looks like math, not code -- but it can still be checked, validated and converted into LaTeX or executables.

## Example

```
x = 2 + 3*7/4

pi == 3.14159

@Number e == lim n->inf of (1 + 1/n)^n^

@Tensor u, v
u*v
```

compiles to:

$$x=2.0+\frac{3.0\cdot 7.0}{4.0}$$
$$\pi \equiv 3.14159$$
$$e\equiv \lim_{\substack{n\to \infty }}\left( 1.0+\frac{1.0}{n}\right) ^{n}$$
$$\overline{u}\cdot \overline{v}$$

## Why Mathsys for Learning?

- Clear, structured expressions instead of dense symbols
- Can be converted directly to LaTeX
- Intuitive to write and read, whilst still being mathematically rigorous
- Can be [used online](https://app.abscissa.eu/playground) without any local installation 
- Allows for greek letters, multicharacter variables and easy syntax

## Try it now

### Online

[Go to playground.](https://app.abscissa.eu/playground)

### Local installation

Install the latest version via `pip`:

```sh
pip install mathsys
```

It is recommended that you import a version specifically:
```py
# named
import mathsys.release as mathsys
import mathsys.dev as mathsys

# specific versions
import mathsys.v1 as mathsys
import mathsys.v2 as mathsys
import mathsys.v3 as mathsys
# ...
```

`mathsys.release` is the last stable version, whilst `mathsys.dev` is the latest version published.

Compile a Mathsys file to different targets with:

```sh
mathsys <target> <filename>.msX
```

where `.msX` is the file extension stands for `.ms1`, `.ms2`, `.ms3` ... (versioned), `.msr` or `.msd` (release and dev).

> [!NOTE]
> To view all available targets, run `mathsys help <filename>.msX`.

> [!WARNING]
> You'll need Rust installed in order to compile to `native` target, with the nightly toolchain, and the `wasm32-unknown-unknown` toolchain for compiling to WebAssembly.
> The compiler will also try to optimize the executable so it is recommended that you install `wasm-opt` and `upx`.

## Project Status

Mathsys is actively developed with regular releases every few weeks. This project is still in its early stages, so expect major shifts and changes. Most features aren't close to being developed yet.
