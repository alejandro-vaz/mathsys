//^
//^ START
//^

//> START -> CLASS
export class Start {constructor(
    readonly statements: Level1[]
) {}}


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> NAMESPACE
export abstract class Level1 {constructor() {}}

//> 1ºLEVEL -> DECLARATION
export class Declaration extends Level1 {constructor(
    readonly group: string | null,
    readonly variable: Variable,
    readonly expression: Expression
) {super()}}

//> 1ºLEVEL -> DEFINITION
export class Definition extends Level1 {constructor(
    readonly group: string | null,
    readonly variable: Variable,
    readonly expression: Expression
) {super()}}

//> 1ºLEVEL -> ANNOTATION
export class Annotation extends Level1 {constructor(
    readonly group: string,
    readonly variables: Variable[]
) {super()}}

//> 1ºLEVEL -> NODE
export class Node extends Level1 {constructor(
    readonly expression: Expression
) {super()}}

//> 1ºLEVEL -> EQUATION
export class Equation extends Level1 {constructor(
    readonly leftexpression: Expression,
    readonly rightexpression: Expression
) {super()}}

//> 1ºLEVEL -> COMMENT
export class Comment extends Level1 {constructor(
    readonly text: string
) {super()}}

//> 1ºLEVEL -> USE
export class Use extends Level1 {constructor(
    readonly name: string,
    readonly start: Start | null
) {super()}}


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> NAMESPACE
export abstract class Level2 {constructor() {}}

//> 2ºLEVEL -> EXPRESSION
export class Expression extends Level2 {constructor(
    readonly signs: (boolean | null)[],
    readonly terms: Level3[]
) {super()}}


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> NAMESPACE
export abstract class Level3 {constructor() {}}

//> 3ºLEVEL -> TERM
export class Term extends Level3 {constructor(
    readonly numerator: Level4[],
    readonly denominator: Level4[]
) {super()}}


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> NAMESPACE
export abstract class Level4 {constructor() {}}

//> 4ºLEVEL -> FACTOR
export class Factor extends Level4 {constructor(
    readonly value: Level5,
    readonly exponent: Expression | null
) {super()}}

//> 4ºLEVEL -> LIMIT
export class Limit extends Level4 {constructor(
    readonly variable: Variable,
    readonly approach: Expression,
    readonly direction: boolean | null,
    readonly nest: Nest,
    readonly exponent: Expression | null
) {super()}}


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> NAMESPACE
export abstract class Level5 {constructor() {}}

//> 5ºLEVEL -> INFINITE
export class Infinite extends Level5 {constructor() {super()}}

//> 5ºLEVEL -> VARIABLE
export class Variable extends Level5 {constructor(
    readonly representation: string
) {super()}}

//> 5ºLEVEL -> NEST
export class Nest extends Level5 {constructor(
    readonly expression: Expression | null
) {super()}}

//> 5ºLEVEL -> TENSOR
export class Tensor extends Level5 {constructor(
    readonly values: Expression[]
) {super()}}

//> 5ºLEVEL -> WHOLE
export class Whole extends Level5 {constructor(
    readonly value: bigint
) {super()}}

//> 5ºLEVEL -> ABSOLUTE
export class Absolute extends Level5 {constructor(
    readonly expression: Expression
) {super()}}