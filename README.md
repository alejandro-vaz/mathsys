# Mathsys

## Diagram

```mermaid
stateDiagram-v2
    [*] --> tokens/tokenize/tokenizer
    tokens/tokenize/tokenizer --> filtered/filter/filter
    filtered/filter/filter --> forest/parse/parser
    grammar/extend/extensor --> forest/parse/parser
    forest/parse/parser --> &start/solve/solver
    &start/solve/solver --> tokens/tokenize/tokenizer
    &start/solve/solver --> latex/render/@
    latex/render/@ --> [*]
```