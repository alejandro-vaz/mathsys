//^
//^ START
//^

//> START -> CLASS
export class Start {constructor(
    public statements: Level1[]
) {}}


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> NAMESPACE
export abstract class Level1 {constructor() {}}

//> 1ºLEVEL -> DECLARATION
export class Declaration extends Level1 {constructor(
    public group: string | null,
    public variable: Variable,
    public expression: Expression
) {super()}}

//> 1ºLEVEL -> DEFINITION
export class Definition extends Level1 {constructor(
    public group: string | null,
    public variable: Variable,
    public expression: Expression
) {super()}}

//> 1ºLEVEL -> ANNOTATION
export class Annotation extends Level1 {constructor(
    public group: string,
    public variables: Variable[]
) {super()}}

//> 1ºLEVEL -> NODE
export class Node extends Level1 {constructor(
    public expression: Expression
) {super()}}

//> 1ºLEVEL -> EQUATION
export class Equation extends Level1 {constructor(
    public leftexpression: Expression,
    public rightexpression: Expression
) {super()}}

//> 1ºLEVEL -> COMMENT
export class Comment extends Level1 {constructor(
    public text: string
) {super()}}

//> 1ºLEVEL -> USE
export class Use extends Level1 {constructor(
    public name: string,
    public start: Start | null
) {super()}}


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> NAMESPACE
export abstract class Level2 {constructor() {}}

//> 2ºLEVEL -> EXPRESSION
export class Expression extends Level2 {constructor(
    public signs: (string | null)[],
    public terms: Level3[]
) {super()}}


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> NAMESPACE
export abstract class Level3 {constructor() {}}

//> 3ºLEVEL -> TERM
export class Term extends Level3 {constructor(
    public numerator: Level4[],
    public denominator: Level4[]
) {super()}}


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> NAMESPACE
export abstract class Level4 {constructor() {}}

//> 4ºLEVEL -> FACTOR
export class Factor extends Level4 {constructor(
    public value: Level5,
    public exponent: Expression | null
) {super()}}

//> 4ºLEVEL -> LIMIT
export class Limit extends Level4 {constructor(
    public variable: Variable,
    public approach: Expression,
    public direction: boolean | null,
    public nest: Nest,
    public exponent: Expression | null
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
    public representation: string
) {super()}}

//> 5ºLEVEL -> NEST
export class Nest extends Level5 {constructor(
    public expression: Expression | null
) {super()}}

//> 5ºLEVEL -> TENSOR
export class Tensor extends Level5 {constructor(
    public values: Expression[]
) {super()}}

//> 5ºLEVEL -> NUMBER
export class Number extends Level5 {constructor(
    public value: number,
    public shift: number
) {super()}}