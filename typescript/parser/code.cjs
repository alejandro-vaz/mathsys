const nearley = require("nearley");
const grammar = require("./syntax.cjs");
const parser = new nearley.Parser(nearley.Grammar.fromCompiled(grammar));
parser.feed(`# this is a simple file
@Tensor u = []
@Tensor v = [1, 0]

# here is another comment
@Number w == u*v

# it's time to end here, but first
use standard

    `);
console.log(JSON.stringify(parser.results));