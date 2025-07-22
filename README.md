## How to install
Currently we only ship the latest packages over `pip`, although soon a web *API* will be designed. To install, just run:
```sh
pip install mathsys
```

## *Mathsys*
*Mathsys* is a *DSL* (Domain-Specific Language) aimed to make math writing easier on computers, and something machines can actually understand.

To do so, we are building a hybrid syntax between what math currently is right now, but adding features from programming languages (like multicharacter variables).

The project is currently on its early stages, and future improvements will be made over time. Most features Mathsys aims to cover aren't close to being developed yet.

A new version is released every few weeks on an irregular schedule. Regular releases happen every one to three weeks, whilst patches and bug fixes are released soon after detection.

Check it out live on [Abscissa](https://abscissa.eu/playground).

I say *we* but I'm indeed a solo developer for now, so if you want to get involved learn

## How to contribute
1. **Check out our wiki:** [here](https://github.com/abscissa-math/mathsys/wiki).
2. **Join the team:** Contact us to become a member of the organisation and get write role.
3. **Work on a branch:** Create a new branch and add your changes. The branch `main` is protected against direct commits to keep the codebase clean, every commit there is a version deployed into the website.
4. **Submit the pull request:** Create a pull request. Don't forget to update the `changelog.md` and the version numbers accordingly. We will give you feedback.

> [!NOTE]
> If there is already a branch that is developing the next version, branch out instead from that branch instead of branching out from `main`.

## Technical background
Currently only a *Python* transpiler to *LaTeX* is available. Here is how it works:
1. Tokenization and *AST* building are handled completely by the [lark parser](https://github.com/lark-parser/lark) at once.
2. A small *LaTeX* generator runs over the *AST* and prints out *LaTeX* character by character.

## License
All rights reserved.