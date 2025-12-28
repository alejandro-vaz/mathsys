//^
//^ HEAD
//^

//> HEAD -> MODULES
import {deflateRawSync} from "zlib";

//> HEAD -> DATA
import {u32, u8, OBJECTTYPE, null32, null8, clamp} from "./local.js";
import * as ir from "./dataclasses.js";
import * as parser from "../parser/dataclasses.js";


//^
//^ IR
//^

//> IR -> GENERATOR
export class IR {
    //~ GENERATOR -> VARIABLES
    private ir: Uint8Array;
    private counter: number;
    private nodes: Map<string, number>;
    //~ GENERATOR -> CONSTRUCTOR
    constructor() {
        this.ir = new Uint8Array()
        this.counter = 0;
        this.nodes = new Map()
    }
    //~ GENERATOR -> RUN
    run(start: parser.Start): Uint8Array {
        this.ir = new Uint8Array();
        this.counter = 0;
        this.nodes.clear();
        this.start(start);
        return deflateRawSync(this.ir, {level: 9});
    }
    //~ GENERATOR -> VARIABLE GENERATOR
    new(element: any): u32 {
        const binary = element.bytes();
        const key = Array.from(binary).join(",");

        if (this.nodes.has(key)) {
            return u32(this.nodes.get(key)!);
        } else {
            this.counter += 1;
            this.ir = clamp(this.ir, binary);
            this.nodes.set(key, this.counter);
            return u32(this.counter);
        }
    }
    //~ GENERATOR -> START GENERATION
    start(start: parser.Start): u32 {return this.new(new ir.Start(
        start.statements.map(statement => this.level1(statement))
    ))}
    //~ GENERATOR -> 1 LEVEL GENERATION
    level1(level1: parser.Level1): u32 {
        if (level1 instanceof parser.Declaration) return this.declaration(level1);
        if (level1 instanceof parser.Definition) return this.definition(level1);
        if (level1 instanceof parser.Annotation) return this.annotation(level1);
        if (level1 instanceof parser.Node) return this.node(level1);
        if (level1 instanceof parser.Equation) return this.equation(level1);
        if (level1 instanceof parser.Comment) return this.comment(level1);
        if (level1 instanceof parser.Use) return this.use(level1);
        throw new Error();
    }
    //~ GENERATOR -> 1 DECLARATION GENERATION
    declaration(declaration: parser.Declaration): u32 {return this.new(new ir.Declaration(
        OBJECTTYPE[String(declaration.group)],
        this.variable(declaration.variable),
        this.expression(declaration.expression)
    ))}
    //~ GENERATOR -> 1 DEFINITION GENERATION
    definition(definition: parser.Definition): u32 {return this.new(new ir.Definition(
        OBJECTTYPE[String(definition.group)],
        this.variable(definition.variable),
        this.expression(definition.expression)
    ))}
    //~ GENERATOR -> 1 ANNOTATION GENERATION
    annotation(annotation: parser.Annotation): u32 {return this.new(new ir.Annotation(
        OBJECTTYPE[String(annotation.group)],
        annotation.variables.map(variable => this.variable(variable))
    ))}
    //~ GENERATOR -> 1 NODE GENERATION
    node(node: parser.Node): u32 {return this.new(new ir.Node(
        this.expression(node.expression)
    ))}
    //~ GENERATOR -> 1 EQUATION GENERATION
    equation(equation: parser.Equation): u32 {return this.new(new ir.Equation(
        this.expression(equation.leftexpression),
        this.expression(equation.rightexpression)
    ))}
    //~ GENERATOR -> 1 COMMENT GENERATION
    comment(comment: parser.Comment): u32 {return this.new(new ir.Comment(
        [new TextEncoder().encode(comment.text)]
    ))}
    //~ GENERATOR -> 1 USE GENERATION
    use(use: parser.Use): u32 {return this.new(new ir.Use(
        [new TextEncoder().encode(use.name)],
        use.start !== null ? this.start(use.start) : null32()
    ))}
    //~ GENERATOR -> 2 LEVEL GENERATION
    level2(level2: parser.Level2): u32 {
        if (level2 instanceof parser.Expression) return this.expression(level2);
        throw new Error();
    }
    //~ GENERATOR -> 2 EXPRESSION GENERATION
    expression(expression: parser.Expression): u32 {return this.new(new ir.Expression(
        expression.signs.map(sign => sign === null || sign ? u8(1) : u8(2)),
        expression.terms.map(term => this.level3(term))
    ))}
    //~ GENERATOR -> 3 LEVEL GENERATION
    level3(level3: parser.Level3): u32 {
        if (level3 instanceof parser.Term) return this.term(level3);
        throw new Error();
    }
    //~ GENERATOR -> 3 TERM GENERATION
    term(term: parser.Term): u32 {return this.new(new ir.Term(
        term.numerator.map(term => this.level4(term)),
        term.denominator.map(term => this.level4(term))
    ))}
    //~ GENERATOR -> 4 LEVEL GENERATION
    level4(level4: parser.Level4): u32 {
        if (level4 instanceof parser.Factor) return this.factor(level4);
        if (level4 instanceof parser.Limit) return this.limit(level4);
        throw new Error();
    }
    //~ GENERATOR -> 4 FACTOR GENERATION
    factor(factor: parser.Factor): u32 {return this.new(new ir.Factor(
        this.level5(factor.value),
        factor.exponent !== null ? this.expression(factor.exponent) : null32()
    ))}
    //~ GENERATOR -> 4 LIMIT GENERATION
    limit(limit: parser.Limit): u32 {return this.new(new ir.Limit(
        this.variable(limit.variable),
        this.expression(limit.approach),
        limit.direction !== null ? u8(+limit.direction + 1) : null8(),
        this.nest(limit.nest),
        limit.exponent !== null ? this.expression(limit.exponent) : null32()
    ))}
    //~ GENERATOR -> 5 LEVEL GENERATION
    level5(level5: parser.Level5): u32 {
        if (level5 instanceof parser.Infinite) return this.infinite(level5);
        if (level5 instanceof parser.Variable) return this.variable(level5);
        if (level5 instanceof parser.Nest) return this.nest(level5);
        if (level5 instanceof parser.Tensor) return this.tensor(level5);
        if (level5 instanceof parser.Whole) return this.whole(level5);
        if (level5 instanceof parser.Absolute) return this.absolute(level5);
        throw new Error();
    }
    //~ GENERATOR -> 5 INFINITE GENERATION
    infinite(infinite: parser.Infinite): u32 {return this.new(new ir.Infinite())}
    //~ GENERATOR -> 5 VARIABLE GENERATION
    variable(variable: parser.Variable): u32 {return this.new(new ir.Variable(
        [new TextEncoder().encode(variable.representation)]
    ))}
    //~ GENERATOR -> 5 NEST GENERATION
    nest(nest: parser.Nest): u32 {return this.new(new ir.Nest(
        nest.expression !== null ? this.expression(nest.expression) : null32()
    ))}
    //~ GENERATOR -> 5 TENSOR GENERATION
    tensor(tensor: parser.Tensor): u32 {return this.new(new ir.Tensor(
        tensor.values.map(value => this.expression(value))
    ))}
    //~ GENERATOR -> 5 WHOLE GENERATION
    whole(whole: parser.Whole): u32 {return this.new(new ir.Whole(
        whole.value !== 0 ? u32(whole.value) : null32()
    ))}
    //~ GENERATOR -> 5 ABSOLUTE GENERATION
    absolute(absolute: parser.Absolute): u32 {return this.new(new ir.Absolute(
        this.expression(absolute.expression)
    ))}
}