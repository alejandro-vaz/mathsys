# Mathsys

![Crates.io Total Downloads](https://img.shields.io/crates/d/mathsys)

Mathsys is an educational math language for students and teachers who want to write, read, and experiment with mathematics in a clear, human-readable way.

It looks like math, not code -- but it can still be checked and converted into LaTeX.

## Example

```
x = 2 + 3*7/4

e == lim n->inf of (1 + 1/n)^n^

pi = 3.14159

L = |alpha - 4|
```

compiles to:

$$x=2+\frac{3\cdot 7}{4}\\ e\equiv \lim_{\substack{n\to \infty }}\left( 1+\frac{1}{n}\right) ^{n}\\ \pi =3.14159\\ L=\left| \alpha -4\right| $$

See more examples in [the documentation.](https://docs.abscissa.eu)

## Why Mathsys for Learning?

- Clear, structured expressions instead of dense symbols
- Can be converted directly to LaTeX
- Intuitive to write and read, whilst still being mathematically rigorous
- Can be [used online](https://app.abscissa.eu/playground) without any local installation 
- Allows for greek letters, multicharacter variables and easy syntax

## Try it now

### Local installation

Install the latest version via `pip`:

```sh
pip install mathsys
```

Compile a Mathsys file to different targets with:

```sh
mathsys <target> <filename>.msX
```

where `.msX` is the file extension stands for `.ms1`, `.ms2`, `.ms3` ... (versioned), `.msr` or `.msd` (release and dev).

`mathsys.release` is the last stable version, whilst `mathsys.dev` is the latest version published.

> [!NOTE]
> To view all available targets, run `mathsys help <filename>.msX`.

> [!WARNING]
> You'll need Rust installed in order to compile to `native` target, with the nightly toolchain, and the `wasm32-unknown-unknown` toolchain for compiling to WebAssembly.
> The compiler will also try to optimize the executable so it is recommended that you install `wasm-opt` and `upx`.

## Project Status

Mathsys is actively developed with regular releases every few weeks. This project is still in its early stages, so expect major shifts and changes. Most features aren't close to being developed yet.
