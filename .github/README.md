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

$$x=2+\frac{3\cdot 7}{4}$$ 
$$e\equiv \lim_{\substack{n\to \infty }}\left( 1+\frac{1}{n}\right) ^{n}$$
$$\pi =3.14159$$ 
$$L=\left| \alpha -4\right| $$

See more examples in [the documentation for Mathsys.](https://docs.abscissa.eu/)

## Why Mathsys for Learning?

- Clear, structured expressions instead of dense symbols
- Can be converted directly to LaTeX
- Intuitive to write and read, whilst still being mathematically rigorous
- Can be [used online](https://app.abscissa.eu/playground) without any local installation 
- Allows for greek letters, multicharacter variables and easy syntax

## Try it now

### Online playground

Mathsys can be tried out at the [online playground](https://abscissa.eu/playground).

> [NOTE!] The online playground is stuck at Mathsys v4, so it might differ slightly from the current release.

### Local installation

Install the latest version via `cargo`:

```sh
cargo install mathsys
```

## Project Status

Mathsys is actively developed with regular releases every few weeks. This project is still in its early stages, so expect major shifts and changes. Most features aren't close to being developed yet.

## Implementation

This DSL is designed to read and write mathematical expressions in a form that feels like natural mathematical language. To achieve this, the implementation focuses on flexibility, clarity, and lazy evaluation, rather than performance or computation (for now).

* Parsing: The core parser is a hand-written Earley parser with backpointers. This choice allows the language to handle highly ambiguous, human-like math syntax, while deferring interpretation of ambiguous constructs until semantic disambiguation can resolve them. The parser constructs the abstract syntax tree lazily, generating nodes only as needed, which reduces unnecessary work and keeps the structure closely aligned with the input text.

* Evaluation: While the DSL does not yet perform computations, all expressions are represented lazily. This design allows future backends—whether for numeric evaluation, symbolic manipulation, or other purposes—to operate efficiently without re-parsing or eagerly constructing intermediate representations. Currently, the only backend outputs LaTeX.

* Implementation Language: The DSL is implemented entirely in Rust. All core parsing and AST logic is handwritten, giving precise control over parsing behavior and memory management.

This combination of a flexible, Earley-based parser, lazy AST construction, and semantic disambiguation allows the DSL to capture the nuance of human-written math while remaining open to future extensions and backends.