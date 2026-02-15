---
title: "Changelog"
description: "Mathsys update changelog."
icon: "code-pull-request"
iconType: "solid"
mode: "wide"
---

# Changelog

## v0
1. Initial release.
1. Added *TypeScript* compiler.
1. Completed [#1](https://github.com/abscissa-math/mathsys/issues/1)
1. Completed [#2](https://github.com/abscissa-math/mathsys/issues/2)
1. Completed [#3](https://github.com/abscissa-math/mathsys/issues/3)
1. Added type annotations in *Python* compiler for some classes.
1. Added support for `+` and `-` operations in the tokenizer and parser.
1. Redefined syntax.
1. Modularized compiler.
1. Published `pip` and `npm` packages.
1. Deleted semantic analyzer.
1. Improved entry points.
1. Completed [#5](https://github.com/abscissa-math/mathsys/issues/5)
1. Completed [#6](https://github.com/abscissa-math/mathsys/issues/6)
1. Completed [#7](https://github.com/abscissa-math/mathsys/issues/7)
1. Built *IR* generator.
1. Switched to *Lark* parser.
1. Removed *TypeScript* compiler temporarily.
1. Defined a simple *IR* instruction set.
1. Modularized the parser so multiple syntaxes are allowed in the future.
1. Made entry point fixed imports to `main/` dynamic.
1. Added the `syntax/` directory with its files to package-shipped files.
1. Replaced *IR* generator for *LaTeX* generator.
1. Added support for multiplication.
1. Redesigned internal syntax and removed comments in it.
1. Added tests.
1. Improved `.gitignore` structure.
1. Create `node` project again with commands.
1. Improved `pip` package structure.
1. Added division.
1. Added vectors.
1. Added multiple-levels syntax hierarchy system.
1. Added comments.
1. Added bare expressions and equations.
1. Improved and cleaned syntax.
1. Added `lark` dependency to *Python* package.
1. Added `·` as a character for multiplication.
1. Added more test cases.
1. Added exponentiation.
1. Removed `sheet` types.
1. Made comments always uppercase.
1. Refactored `Factor` *LaTeX* generation.
1. Added `º()` function on parser which accesses a list safely.

## v1
1. Added *Rust* runtime.
1. Added *IR* generator.
1. Added executable builder.
1. Added streamlined workflows for developing on assembly and assembly support.
1. Updated syntax to make parsing possible.
1. Updated `README.md`.
1. Improved entry point.
1. Added limits.
1. Added immutable definitions.
1. Added Greek letter mapping.
1. Added `inf` keyword for infinity.
1. Added tags to `README.md`.
1. Trimmed down `babel.config.json`.
1. Improved public *API*.
1. Added thread safety to runtime allocator.
1. Optimized runtime `bcmp()` function.
1. Removed mod `lib::string`.
1. Added inlining for stack functions imported from assembly.
1. Improved compile commands with subshells and heredocs.
1. Renamed environment variable for the *IR* to `Mathsys`.
1. Streamlined compilation process with `all.o` binaries.
1. Refactored complex functions in the compilation steps to keep cyclomatic complexity under 10.
1. Optimized package structure to preserve previous versions.
1. Moved from a bump allocator to a stack allocator.
1. Removed implicit addition for implicit multiplication continuation. 
1. Syntax overhaul to keep functionality but increase ease-of-parse.
1. Updated `README.md` to account for changes and the [new documentation](https://docs.abscissa.eu).
1. Unified compile scripts for assembly.
1. Moved from beta to stable in `pyproject.toml`.
1. Made *Rust* functions safe, with small unsafe blocks for wrappers and the allocator.
1. Renamed `bcalls` and `ncalls` to detail and lookup.
1. Renamed `sheet` to `start` and made it the entry point, delaying `level1`.
1. Moved changelog to the [new documentation](https://docs.abscissa.eu).
1. Removed testing until there's an integration with *Abscissa* for testing data.

## v2
1. Implemented *IR* parsing in the *Rust* runtime.
1. Added data implementations for all objects in *Rust.*
1. Removed data race issues from the stack allocator.
1. Improved `.gitignore` organisation.
1. Fixed typos in `README.md`.
1. Corrected email in `pyproject.toml`.
1. Total heap size message before runtime initialization now uses scientific notation.
1. Improved error handling in `builder.py`.
1. Moved limit to level four.
1. Refactored assembly build scripts.

## v3
1. Added runtime values with their own methods.
1. Created runtime context.
1. Added evaluation methods to classes.
1. Started processing the `Start` node.
1. Improved *IR* parsing in *Rust.*
1. Added `Value` downcasting in the *Rust* runtime.
1. Removed previous versions from `.gitignore`.
1. Fixed typos on `README.md`.
1. Added `memcmp()` function on `memory.rs`.
1. Simplified assembly folders, assembly will only be used for communicating with the system.
1. Changed *LaTeX* transpiler to be dataclass-based instead of having a string buffer.
1. Improved comment syntax.
1. Added file extension based compiling.
1. Optimized runtime and added variable assignments.
1. Fixed grammar and syntax in `README.md`.
1. Added version and version info to exports.
1. Merged `rustc.rs` and `memory.rs`.
1. Implemented caching with `lru_cache` and hashing inputs.
1. Added `_Nexists` runtime value.
1. Added compile-time indicator for *Python* targets.
1. Removed `#debug`.
1. Added exit codes enum for runtime.
1. Enhanced runtime verbose.
1. Removed anti-verbosing `settings.detail` and `.lookup`.
1. Updated error handling in builder.
1. Fixed separation of variables with `inf`.
1. Fixed extra newlines at document start.
1. Added keywords to class constructors in `parser.py`.
1. Fixed `lru_cache` and added cache managing utils.
1. Improved context `id` and added `equiv` function.
1. Added runtime code for `Equation`.
1. Improved error handling.
1. Removed watch command.
1. Added negative property to `_Number`.
1. Removed mutable downcast function.
1. Changed context runtime so that `Start` must be the last item.
1. Alert stdout print is now orange instead of yellowish.
1. Added support for multicharacter greek words.
1. Added `º`, `$` and `%` characters as valid for variables.
1. Added summation method to runtime values.
1. Switched to bitmap allocator.
1. Streamlined locales and genlocales in runtime values.
1. Developed Expression data structure.
1. Added new environment variables to control runtime.
1. Changed catch-all from match clauses from `_` to `other`.
1. Fixed exception handling.
1. Updated `README.md` wording.
1. Added mutable downcasting.
1. Added empty parenthesis for a `_Nexists` value.
1. Added size limit for `lru_cache`.
1. Added info method to context values in runtime.
1. Fixed addition methods.
1. Trimmed down allocator.
1. Added strong typing in syntax, parser, *LaTeX,* *IR,* and throughout the runtime.
1. Added tensor runtime value.
1. Added annotation `level1` structure.
1. Renamed vector to tensor.
1. Renamed `_Infinity` to `Infinite` in context.
1. Renamed all context classes to have no underscore but data ones to have.
1. Removed all downcasting in favor of mutcasting.
1. Fixed Greek letters collision.
1. Removed `(frozen = True)` dataclasses.
1. Added type system in runtime.
2. Added standard library and `use` keyword.
1. Restructured `data/*.rs` files to improve logs.
1. Fixed `equiv` and `summation` methods in context values.
2. Added modulus method for tensor.
3. Moved memory usage tracking to left.

## v4
1. Optimized *Python* parsing workflow.
1. Moved to a *Rust* standard runtime with the usual global allocator, so marks were removed.
1. Added `tokens()` target.
1. Expanded runtime values adding `unequivalency()`.
1. Expanded runtime values adding `multiplication()`.
1. Expanded runtime values adding `negate()` and `invert()`.
1. Streamlined runtime debugging with display and debug implementations.
1. Added `partial()` and `result()` methods with `chore()` and `class()` outputs.
1. Refactored `converter.rs`.
1. Added *IR* deduplication.
1. Standardized variable names.
1. Expanded `stdout` functions to allow for any printable input.

## v5
1. Remade versioning system
1. Enhanced runtime
1. Enhanced syntax
1. Added TypeScript package
1. Migrated to Cargo
1. Divided `@Number` type into individual types

## v6
No changelog available.

## v7
### Added
1. Full migration to Rust.
### Changed
1. Migrated from Python to Rust.
1. Simplified syntax without types.
### Removed
1. Executable building.
1. IR target.
1. Runtime flags.
### Fixed
1. Full issue coverage now.

## v8 -- Semantic solver
### Added
1. Local module importing from filesystem.
1. Added `README.md` in `Cargo.toml`.
### Changed
1. Contextual semantic solver replaced old heuristic solver.
1. Improved filetree and directories.
### Removed
1. Old numerical solver.
### Fixed
1. Imported modules now must have `.msm` file extension.