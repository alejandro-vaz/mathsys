//^
//^ HEAD
//^

//> HEAD -> MODULES
import {deflateRawSync} from "zlib";

//> HEAD -> DATA
import {Binary, Opcode, Pointer, Sign, Option, BigUint, String, Group, Vec, BinaryEncodable, join} from "./local.js";
import * as ir from "./dataclasses.js";
import * as parser from "../parser/dataclasses.js";


//^
//^ IR
//^

//> IR -> GENERATOR
export class IR {
    //~ GENERATOR -> VARIABLES
    private ir: Binary;
    private counter: number;
    private nodes: Map<Binary, number>;
    //~ GENERATOR -> CONSTRUCTOR
    constructor() {
        this.ir = new Binary(0n, 0);
        this.counter = 0;
        this.nodes = new Map();
    }
    //~ GENERATOR -> RUN
    run(start: parser.Start): Uint8Array {
        this.ir = new Binary(0n, 0);
        this.counter = 0;
        this.nodes.clear();
        this.start(start);
        while (this.ir.width % 8 !== 0) {this.ir = join(this.ir, new Opcode(0x00))};
        return deflateRawSync(this.ir.__bytes__(), {level: 9});
    }
    //~ GENERATOR -> VARIABLE GENERATOR
    new(element: BinaryEncodable): Pointer {
        const binary = element.binary();
        if (this.nodes.has(binary)) {return new Pointer(this.nodes.get(binary) as number)} else {
            this.ir = join(this.ir, binary);
            this.nodes.set(binary, this.counter);
            this.counter += 1;
            return new Pointer(this.counter);
        }
    }
    //~ GENERATOR -> START GENERATION
    start(start: parser.Start): Pointer {return this.new(new ir.Start(
        new Vec(start.statements.map(statement => this.level1(statement)))
    ))}
    //~ GENERATOR -> 1 LEVEL GENERATION
    level1(level1: parser.Level1): Pointer {
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
    declaration(declaration: parser.Declaration): Pointer {return this.new(new ir.Declaration(
        new Group(declaration.group),
        this.variable(declaration.variable),
        this.expression(declaration.expression)
    ))}
    //~ GENERATOR -> 1 DEFINITION GENERATION
    definition(definition: parser.Definition): Pointer {return this.new(new ir.Definition(
        new Group(definition.group),
        this.variable(definition.variable),
        this.expression(definition.expression)
    ))}
    //~ GENERATOR -> 1 ANNOTATION GENERATION
    annotation(annotation: parser.Annotation): Pointer {return this.new(new ir.Annotation(
        new Group(annotation.group),
        new Vec(annotation.variables.map(variable => this.variable(variable)))
    ))}
    //~ GENERATOR -> 1 NODE GENERATION
    node(node: parser.Node): Pointer {return this.new(new ir.Node(
        this.expression(node.expression)
    ))}
    //~ GENERATOR -> 1 EQUATION GENERATION
    equation(equation: parser.Equation): Pointer {return this.new(new ir.Equation(
        this.expression(equation.leftexpression),
        this.expression(equation.rightexpression)
    ))}
    //~ GENERATOR -> 1 COMMENT GENERATION
    comment(comment: parser.Comment): Pointer {return this.new(new ir.Comment(
        new String(comment.text)
    ))}
    //~ GENERATOR -> 1 USE GENERATION
    use(use: parser.Use): Pointer {return this.new(new ir.Use(
        new String(use.name),
        new Option(use.start !== null ? this.start(use.start) : null)
    ))}
    //~ GENERATOR -> 2 LEVEL GENERATION
    level2(level2: parser.Level2): Pointer {
        if (level2 instanceof parser.Expression) return this.expression(level2);
        throw new Error();
    }
    //~ GENERATOR -> 2 EXPRESSION GENERATION
    expression(expression: parser.Expression): Pointer {return this.new(new ir.Expression(
        new Vec(expression.signs.map(sign => new Option(sign === null ? null : new Sign(sign)))),
        new Vec(expression.terms.map(term => this.level3(term)))
    ))}
    //~ GENERATOR -> 3 LEVEL GENERATION
    level3(level3: parser.Level3): Pointer {
        if (level3 instanceof parser.Term) return this.term(level3);
        throw new Error();
    }
    //~ GENERATOR -> 3 TERM GENERATION
    term(term: parser.Term): Pointer {return this.new(new ir.Term(
        new Vec(term.numerator.map(term => this.level4(term))),
        new Vec(term.denominator.map(term => this.level4(term)))
    ))}
    //~ GENERATOR -> 4 LEVEL GENERATION
    level4(level4: parser.Level4): Pointer {
        if (level4 instanceof parser.Factor) return this.factor(level4);
        if (level4 instanceof parser.Limit) return this.limit(level4);
        throw new Error();
    }
    //~ GENERATOR -> 4 FACTOR GENERATION
    factor(factor: parser.Factor): Pointer {return this.new(new ir.Factor(
        this.level5(factor.value),
        new Option(factor.exponent !== null ? this.expression(factor.exponent) : null)
    ))}
    //~ GENERATOR -> 4 LIMIT GENERATION
    limit(limit: parser.Limit): Pointer {return this.new(new ir.Limit(
        this.variable(limit.variable),
        this.expression(limit.approach),
        new Option(limit.direction !== null ? new Sign(limit.direction) : null),
        this.nest(limit.nest),
        new Option(limit.exponent !== null ? this.expression(limit.exponent) : null)
    ))}
    //~ GENERATOR -> 5 LEVEL GENERATION
    level5(level5: parser.Level5): Pointer {
        if (level5 instanceof parser.Infinite) return this.infinite(level5);
        if (level5 instanceof parser.Variable) return this.variable(level5);
        if (level5 instanceof parser.Nest) return this.nest(level5);
        if (level5 instanceof parser.Tensor) return this.tensor(level5);
        if (level5 instanceof parser.Whole) return this.whole(level5);
        if (level5 instanceof parser.Absolute) return this.absolute(level5);
        throw new Error();
    }
    //~ GENERATOR -> 5 INFINITE GENERATION
    infinite(infinite: parser.Infinite): Pointer {return this.new(new ir.Infinite())}
    //~ GENERATOR -> 5 VARIABLE GENERATION
    variable(variable: parser.Variable): Pointer {return this.new(new ir.Variable(
        new String(variable.representation)
    ))}
    //~ GENERATOR -> 5 NEST GENERATION
    nest(nest: parser.Nest): Pointer {return this.new(new ir.Nest(
        new Option(nest.expression !== null ? this.expression(nest.expression) : null)
    ))}
    //~ GENERATOR -> 5 TENSOR GENERATION
    tensor(tensor: parser.Tensor): Pointer {return this.new(new ir.Tensor(
        new Vec(tensor.values.map(value => this.expression(value)))
    ))}
    //~ GENERATOR -> 5 WHOLE GENERATION
    whole(whole: parser.Whole): Pointer {return this.new(new ir.Whole(
        new BigUint(whole.value)
    ))}
    //~ GENERATOR -> 5 ABSOLUTE GENERATION
    absolute(absolute: parser.Absolute): Pointer {return this.new(new ir.Absolute(
        this.expression(absolute.expression)
    ))}
}