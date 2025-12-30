#^
#^  HEAD
#^

#> HEAD -> MODULES
from zlib import compress

#> HEAD -> DATA
from .local import Binary, Opcode, Pointer, Sign, Option, BigUint, String, Group, Vec, BinaryEncodable
from . import dataclasses as ir
from ..parser import dataclasses as parser


#^
#^  IR
#^

#> IR -> GENERATOR
class IR:
    #~ GENERATOR -> VARIABLES
    ir: Binary
    counter: int
    nodes: dict
    #~ GENERATOR -> INIT
    def __init__(self) -> None: pass
    #~ GENERATOR -> RUN
    def run(self, start: parser.Start) -> bytes:
        self.ir = Binary(value = 0, width= 0)
        self.counter = 0
        self.nodes = {}
        self.start(start)
        while self.ir.width % 8 != 0: self.ir += Opcode(0x00)
        return compress(self.ir.__bytes__(), level = 9, wbits = -15)
    #~ GENERATOR -> VARIABLE GENERATOR
    def new(self, element: BinaryEncodable) -> Pointer:
        binary = element.binary()
        if binary in self.nodes: return Pointer(self.nodes[binary])
        else:
            self.ir += binary
            self.nodes[binary] = self.counter
            pointer = Pointer(self.counter)
            self.counter += 1
            return pointer
    #~ GENERATOR -> START GENERATION
    def start(self, start: parser.Start) -> Pointer: 
        return self.new(ir.Start(
            statements = Vec([self.level1(statement) for statement in start.statements])
        ))
    #~ GENERATOR -> 1 LEVEL GENERATION
    def level1(self, level1: parser.Level1) -> Pointer:
        match level1:
            case parser.Declaration(): return self.declaration(level1)
            case parser.Definition(): return self.definition(level1)
            case parser.Annotation(): return self.annotation(level1)
            case parser.Node(): return self.node(level1)
            case parser.Equation(): return self.equation(level1)
            case parser.Comment(): return self.comment(level1)
            case parser.Use(): return self.use(level1)
        return NotImplemented
    #~ GENERATOR -> 1 DECLARATION GENERATION
    def declaration(self, declaration: parser.Declaration) -> Pointer: 
        return self.new(ir.Declaration(
            group = Group(declaration.group),
            variable = self.variable(declaration.variable),
            expression = self.expression(declaration.expression)
        ))
    #~ GENERATOR -> 1 DEFINITION GENERATION
    def definition(self, definition: parser.Definition) -> Pointer:
        return self.new(ir.Definition(
            group = Group(definition.group),
            variable = self.variable(definition.variable),
            expression = self.expression(definition.expression)
        ))
    #~ GENERATOR -> 1 ANNOTATION GENERATION
    def annotation(self, annotation: parser.Annotation) -> Pointer:
        return self.new(ir.Annotation(
            group = Group(annotation.group),
            variables = Vec([self.variable(variable) for variable in annotation.variables])
        ))
    #~ GENERATOR -> 1 NODE GENERATION
    def node(self, node: parser.Node) -> Pointer:
        return self.new(ir.Node(
            expression = self.expression(node.expression)
        ))
    #~ GENERATOR -> 1 EQUATION GENERATION
    def equation(self, equation: parser.Equation) -> Pointer:
        return self.new(ir.Equation(
            leftexpression = self.expression(equation.leftexpression),
            rightexpression = self.expression(equation.rightexpression)
        ))
    #~ GENERATOR -> 1 COMMENT GENERATION
    def comment(self, comment: parser.Comment) -> Pointer:
        return self.new(ir.Comment(
            text = String(comment.text)
        ))
    #~ GENERATOR -> 1 USE GENERATION
    def use(self, use: parser.Use) -> Pointer:
        return self.new(ir.Use(
            name = String(use.name),
            start = Option(self.start(use.start) if use.start is not None else None)
        ))
    #~ GENERATOR -> 2 LEVEL GENERATION
    def level2(self, level2: parser.Level2) -> Pointer:
        match level2:
            case parser.Expression(): return self.expression(level2)
        return NotImplemented
    #~ GENERATOR -> 2 EXPRESSION GENERATION
    def expression(self, expression: parser.Expression) -> Pointer:
        return self.new(ir.Expression(
            signs = Vec([Option(Sign(sign) if isinstance(sign, bool) else None) for sign in expression.signs]),
            terms = Vec([self.level3(item) for item in expression.terms])
        ))
    #~ GENERATOR -> 3 LEVEL GENERATION
    def level3(self, level3: parser.Level3) -> Pointer:
        match level3:
            case parser.Term(): return self.term(level3)
        return NotImplemented
    #~ GENERATOR -> 3 TERM GENERATION
    def term(self, term: parser.Term) -> Pointer:
        return self.new(ir.Term(
            numerator = Vec([self.level4(item) for item in term.numerator]),
            denominator = Vec([self.level4(item) for item in term.denominator])
        ))
    #~ GENERATOR -> 4 LEVEL GENERATION
    def level4(self, level4: parser.Level4) -> Pointer:
        match level4:
            case parser.Factor(): return self.factor(level4)
            case parser.Limit(): return self.limit(level4)
        return NotImplemented
    #~ GENERATOR -> 4 FACTOR GENERATION
    def factor(self, factor: parser.Factor) -> Pointer:
        return self.new(ir.Factor(
            value = self.level5(factor.value),
            exponent = Option(self.expression(factor.exponent) if factor.exponent is not None else None)
        ))
    #~ GENERATOR -> 4 LIMIT GENERATION
    def limit(self, limit: parser.Limit) -> Pointer:
        return self.new(ir.Limit(
            variable = self.variable(limit.variable),
            approach = self.expression(limit.approach),
            direction = Option(Sign(limit.direction) if limit.direction is not None else None),
            nest = self.nest(limit.nest),
            exponent = Option(self.expression(limit.exponent) if limit.exponent is not None else None)
        ))
    #~ GENERATOR -> 5 LEVEL GENERATION
    def level5(self, level5: parser.Level5) -> Pointer:
        match level5:
            case parser.Infinite(): return self.infinite(level5)
            case parser.Variable(): return self.variable(level5)
            case parser.Nest(): return self.nest(level5)
            case parser.Tensor(): return self.tensor(level5)
            case parser.Whole(): return self.whole(level5)
            case parser.Absolute(): return self.absolute(level5)
        return NotImplemented
    #~ GENERATOR -> 5 INFINITE GENERATION
    def infinite(self, infinite: parser.Infinite) -> Pointer:
        return self.new(ir.Infinite())
    #~ GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: parser.Variable) -> Pointer: 
        return self.new(ir.Variable(
            representation = String(variable.representation)
        ))
    #~ GENERATOR -> 5 NEST GENERATION
    def nest(self, nest: parser.Nest) -> Pointer:
        return self.new(ir.Nest(
            expression = Option(self.expression(nest.expression) if nest.expression is not None else None)
        ))
    #~ GENERATOR -> 5 TENSOR GENERATION
    def tensor(self, tensor: parser.Tensor) -> Pointer:
        return self.new(ir.Tensor(
            values = Vec([self.expression(value) for value in tensor.values])
        ))
    #~ GENERATOR -> 5 WHOLE GENERATION
    def whole(self, whole: parser.Whole) -> Pointer:
        return self.new(ir.Whole(
            value = BigUint(whole.value)
        ))
    #~ GENERATOR -> 5 ABSOLUTE GENERATION
    def absolute(self, absolute: parser.Absolute) -> Pointer:
        return self.new(ir.Absolute(
            expression = self.expression(absolute.expression)
        ))