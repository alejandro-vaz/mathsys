#^
#^  HEAD
#^

#> HEAD -> DATA
from .local import u32, u8, OBJECTTYPE, null32, null8
from . import dataclasses as ir
from ..parser import dataclasses as parser


#^
#^  IR
#^

#> IR -> GENERATOR
class IR:
    #~ GENERATOR -> VARIABLES
    ir: bytes
    counter: int
    nodes: dict
    #~ GENERATOR -> INIT
    def __init__(self) -> None: pass
    #~ GENERATOR -> VARIABLE GENERATOR
    def new(self, element) -> u32:
        binary = bytes(element)
        if binary in self.nodes:
            return u32(self.nodes[binary])
        else:
            self.counter += 1
            self.ir += binary
            self.nodes[binary] = self.counter
            return u32(self.counter)
    #~ GENERATOR -> RUN
    def run(self, start: parser.Start) -> bytes:
        self.ir = b""
        self.counter = 0
        self.nodes = {}
        self.start(start)
        return self.ir
    #~ GENERATOR -> START GENERATION
    def start(self, start: parser.Start) -> u32: 
        return self.new(ir.Start(
            statements = [self.level1(statement) for statement in start.statements]
        ))
    #~ GENERATOR -> 1 LEVEL GENERATION
    def level1(self, level1: parser.Level1) -> u32:
        match level1:
            case parser.Declaration(): return self.declaration(level1)
            case parser.Definition(): return self.definition(level1)
            case parser.Annotation(): return self.annotation(level1)
            case parser.Node(): return self.node(level1)
            case parser.Equation(): return self.equation(level1)
            case parser.Comment(): return self.comment(level1)
            case parser.Use(): return self.use(level1)
    #~ GENERATOR -> 1 DECLARATION GENERATION
    def declaration(self, declaration: parser.Declaration) -> u32: 
        return self.new(ir.Declaration(
            group = OBJECTTYPE[declaration.group],
            variable = self.variable(declaration.variable),
            expression = self.expression(declaration.expression)
        ))
    #~ GENERATOR -> 1 DEFINITION GENERATION
    def definition(self, definition: parser.Definition) -> u32:
        return self.new(ir.Definition(
            group = OBJECTTYPE[definition.group],
            variable = self.variable(definition.variable),
            expression = self.expression(definition.expression)
        ))
    #~ GENERATOR -> 1 ANNOTATION GENERATION
    def annotation(self, annotation: parser.Annotation) -> u32:
        return self.new(ir.Annotation(
            group = OBJECTTYPE[annotation.group],
            variables = [self.variable(variable) for variable in annotation.variables]
        ))
    #~ GENERATOR -> 1 NODE GENERATION
    def node(self, node: parser.Node) -> u32:
        return self.new(ir.Node(
            expression = self.expression(node.expression)
        ))
    #~ GENERATOR -> 1 EQUATION GENERATION
    def equation(self, equation: parser.Equation) -> u32:
        return self.new(ir.Equation(
            leftexpression = self.expression(equation.leftexpression),
            rightexpression = self.expression(equation.rightexpression)
        ))
    #~ GENERATOR -> 1 COMMENT GENERATION
    def comment(self, comment: parser.Comment) -> u32:
        return self.new(ir.Comment(
            text = [comment.text.encode()]
        ))
    #~ GENERATOR -> 1 USE GENERATION
    def use(self, use: parser.Use) -> u32:
        return self.new(ir.Use(
            name = [use.name.encode()],
            start = self.start(use.start) if use.start is not None else null32()
        ))
    #~ GENERATOR -> 2 LEVEL GENERATION
    def level2(self, level2: parser.Level2) -> u32:
        match level2:
            case parser.Expression(): return self.expression(level2)
    #~ GENERATOR -> 2 EXPRESSION GENERATION
    def expression(self, expression: parser.Expression) -> u32:
        return self.new(ir.Expression(
            signs = [u8(1) if sign is None or sign else u8(2) for sign in expression.signs],
            terms = [self.level3(item) for item in expression.terms]
        ))
    #~ GENERATOR -> 3 LEVEL GENERATION
    def level3(self, level3: parser.Level3) -> u32:
        match level3:
            case parser.Term(): return self.term(level3)
    #~ GENERATOR -> 3 TERM GENERATION
    def term(self, term: parser.Term) -> u32:
        return self.new(ir.Term(
            numerator = [self.level4(item) for item in term.numerator],
            denominator = [self.level4(item) for item in term.denominator]
        ))
    #~ GENERATOR -> 4 LEVEL GENERATION
    def level4(self, level4: parser.Level4) -> u32:
        match level4:
            case parser.Factor(): return self.factor(level4)
            case parser.Limit(): return self.limit(level4)
    #~ GENERATOR -> 4 FACTOR GENERATION
    def factor(self, factor: parser.Factor) -> u32:
        return self.new(ir.Factor(
            value = self.level5(factor.value),
            exponent = self.expression(factor.exponent) if factor.exponent is not None else null32()
        ))
    #~ GENERATOR -> 4 LIMIT GENERATION
    def limit(self, limit: parser.Limit) -> u32:
        return self.new(ir.Limit(
            variable = self.variable(limit.variable),
            approach = self.expression(limit.approach),
            direction = u8(int(limit.direction) + 1) if limit.direction is not None else null8(),
            nest = self.nest(limit.nest),
            exponent = self.expression(limit.exponent) if limit.exponent is not None else null32()
        ))
    #~ GENERATOR -> 5 LEVEL GENERATION
    def level5(self, level5: parser.Level5) -> u32:
        match level5:
            case parser.Infinite(): return self.infinite(level5)
            case parser.Variable(): return self.variable(level5)
            case parser.Nest(): return self.nest(level5)
            case parser.Tensor(): return self.tensor(level5)
            case parser.Natural(): return self.natural(level5)
    #~ GENERATOR -> 5 INFINITE GENERATION
    def infinite(self, infinite: parser.Infinite) -> u32:
        return self.new(ir.Infinite())
    #~ GENERATOR -> 5 VARIABLE GENERATION
    def variable(self, variable: parser.Variable) -> u32: 
        return self.new(ir.Variable(
            representation = [variable.representation.encode()]
        ))
    #~ GENERATOR -> 5 NEST GENERATION
    def nest(self, nest: parser.Nest) -> u32:
        return self.new(ir.Nest(
            expression = self.expression(nest.expression) if nest.expression is not None else null32()
        ))
    #~ GENERATOR -> 5 TENSOR GENERATION
    def tensor(self, tensor: parser.Tensor) -> u32:
        return self.new(ir.Tensor(
            values = [self.expression(value) for value in tensor.values]
        ))
    #~ GENERATOR -> 5 NATURAL GENERATION
    def natural(self, natural: parser.Natural) -> u32:
        return self.new(ir.Natural(
            value = u32(natural.value) if natural.value != 0 else null32()
        ))