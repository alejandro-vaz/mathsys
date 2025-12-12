"use strict";

var _nearley = _interopRequireDefault(require("nearley"));
var _syntax = _interopRequireDefault(require("./syntax.cjs"));
var _util = _interopRequireDefault(require("util"));
function _interopRequireDefault(e) { return e && e.__esModule ? e : { default: e }; }
const parser = new _nearley.default.Parser(_nearley.default.Grammar.fromCompiled(_syntax.default));
parser.feed(`+2*inf - 3/4`);
console.log(_util.default.inspect(parser.results[0], {
  depth: null,
  colors: true
}));