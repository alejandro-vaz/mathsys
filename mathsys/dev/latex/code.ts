//^
//^ HEAD
//^

//> HEAD -> DATA
import {types} from "./local.js";
import * as latex from "./dataclasses.js";
import * as parser from "../parser/dataclasses.js";


//^
//^ LATEX
//^

//> LATEX -> GENERATOR
export class LaTeX {
    //~ GENERATOR -> CONSTRUCTOR
    constructor() {}
    //~ GENERATOR -> RUN
    run(start: parser.Start): string {
        for (const key in types) {
            delete types[key];
        }
        return this.start(start);
    }
    //~ GENERATOR -> START GENERATION
    start(start: parser.Start): string {
        return String(new latex.Start(
            start.statements.map(statement => this.level1(statement)).filter(statement => Boolean(statement))
        ));
    }
    //~ GENERATOR -> 1 LEVEL GENERATION
    level1(level1: parser.Level1): string {
        if (level1 instanceof parser.Declaration) return this.declaration(level1);
        if (level1 instanceof parser.Definition) return this.definition(level1);
        if (level1 instanceof parser.Annotation) return this.annotation(level1);
        if (level1 instanceof parser.Node) return this.node(level1);
        if (level1 instanceof parser.Equation) return this.equation(level1);
        if (level1 instanceof parser.Comment) return this.comment(level1);
        if (level1 instanceof parser.Use) return this.use(level1);
        return "";
    }
    //~ GENERATOR -> 1 DECLARATION GENERATION
    declaration(declaration: parser.Declaration): string {
        if (declaration.group !== null) {
            types[declaration.variable.representation] = declaration.variable.representation in types ? types[declaration.variable.representation] : declaration.group
        }
        return String(new latex.Declaration(
            declaration.group !== null ? declaration.group : "",
            this.variable(declaration.variable),
            this.expression(declaration.expression)
        ));
    }
    //~ GENERATOR -> 1 DEFINITION GENERATION
    definition(definition: parser.Definition): string {
        if (definition.group !== null) {
            types[definition.variable.representation] = definition.variable.representation in types ? types[definition.variable.representation] : definition.group
        }
        return String(new latex.Definition(
            definition.group !== null ? definition.group : "",
            this.variable(definition.variable),
            this.expression(definition.expression)
        ));
    }
    //~ GENERATOR -> 1 ANNOTATION GENERATION
    annotation(annotation: parser.Annotation): string {
        for (const variable of annotation.variables) {
            types[variable.representation] = variable.representation in types ? types[variable.representation] : annotation.group
        }
        return String(new latex.Annotation(
            annotation.group,
            annotation.variables.map(variable => this.variable(variable))
        ));
    }
    //~ GENERATOR -> 1 NODE GENERATION
    node(node: parser.Node): string {
        return String(new latex.Node(
            this.expression(node.expression)
        ));
    }
    //~ GENERATOR -> 1 EQUATION GENERATION
    equation(equation: parser.Equation): string {
        return String(new latex.Equation(
            this.expression(equation.leftexpression),
            this.expression(equation.rightexpression)
        ));
    }
    //~ GENERATOR -> 1 COMMENT GENERATION
    comment(comment: parser.Comment): string {
        return String(new latex.Comment(
            comment.text
        ));
    }
    //~ GENERATOR -> 1 USE GENERATION
    use(use: parser.Use): string {
        if (use.start !== null) {this.start(use.start)}
        return String(new latex.Use(
            use.name,
            use.start !== null
        ));
    }
    //~ GENERATOR -> 2 LEVEL GENERATION
    level2(level2: parser.Level2): string {
        if (level2 instanceof parser.Expression) return this.expression(level2);
        return "";
    }
    //~ GENERATOR -> 2 EXPRESSION GENERATION
    expression(expression: parser.Expression): string {
    if (!expression) {
        console.error("LaTeX.expression called with:", expression);
        throw new Error("Invalid Expression passed to LaTeX.expression");
    }
        return String(new latex.Expression(
            expression.signs.map(sign => sign !== null ? sign : ""),
            expression.terms.map(term => this.level3(term))
        ));
    }
    //~ GENERATOR -> 3 LEVEL GENERATION
    level3(level3: parser.Level3): string {
        if (level3 instanceof parser.Term) return this.term(level3);
        return "";
    }
    //~ GENERATOR -> 3 TERM GENERATION
    term(term: parser.Term): string {
        const numerator: string[] = [];
        term.numerator.forEach((item: parser.Level4, index: number) => {
            let value = this.level4(item);
            if (index !== 0) {
                if (term.numerator[index - 1] instanceof parser.Factor) {
                    if ((term.numerator[index - 1] as any).value instanceof parser.Number) {
                        if (item instanceof parser.Factor) {
                            if (item.value instanceof parser.Number || item.value instanceof parser.Infinite) {
                                value = String.raw`\cdot ` + value;
                            }
                        }
                    } else {value = String.raw`\cdot ` + value}
                } else {value = String.raw`\cdot ` + value}
            }
            numerator.push(value);
        });
        const denominator: string[] = [];
        term.denominator.forEach((item: parser.Level4, index: number) => {
            let value = this.level4(item);
            if (index !== 0) {
                if (term.denominator[index - 1] instanceof parser.Factor) {
                    if ((term.denominator[index - 1] as any).value instanceof parser.Number) {
                        if (item instanceof parser.Factor) {
                            if (item.value instanceof parser.Number || item.value instanceof parser.Infinite) {
                                value = String.raw`\cdot ` + value;
                            }
                        }
                    } else {value = String.raw`\cdot ` + value}
                } else {value = String.raw`\cdot ` + value}
            }
            denominator.push(value);
        });
        return String(new latex.Term(
            numerator,
            denominator
        ));
    }
    //~ GENERATOR -> 4 LEVEL GENERATION
    level4(level4: parser.Level4): string {
        if (level4 instanceof parser.Factor) return this.factor(level4);
        if (level4 instanceof parser.Limit) return this.limit(level4);
        return "";
    }
    //~ GENERATOR -> 4 FACTOR GENERATION
    factor(factor: parser.Factor): string {
        return String(new latex.Factor(
            this.level5(factor.value),
            factor.exponent !== null ? this.expression(factor.exponent) : null
        ));
    }
    //~ GENERATOR -> 4 LIMIT GENERATION
    limit(limit: parser.Limit): string {
        return String(new latex.Limit(
            this.variable(limit.variable),
            this.expression(limit.approach),
            limit.direction,
            this.nest(limit.nest),
            limit.exponent !== null ? this.expression(limit.exponent) : null
        ));
    }
    //~ GENERATOR -> 5 LEVEL GENERATION
    level5(level5: parser.Level5): string {
        if (level5 instanceof parser.Infinite) return this.infinite(level5);
        if (level5 instanceof parser.Variable) return this.variable(level5);
        if (level5 instanceof parser.Nest) return this.nest(level5);
        if (level5 instanceof parser.Tensor) return this.tensor(level5);
        if (level5 instanceof parser.Number) return this.number(level5);
        return "";
    }
    //~ GENERATOR -> 5 INFINITE GENERATION
    infinite(infinite: parser.Infinite): string {
        return String(new latex.Infinite());
    }
    //~ GENERATOR -> 5 VARIABLE GENERATION
    variable(variable: parser.Variable): string {
        return String(new latex.Variable(
            variable.representation
        ));
    }
    //~ GENERATOR -> 5 NEST GENERATION
    nest(nest: parser.Nest): string {
        return String(new latex.Nest(
            nest.expression !== null ? this.expression(nest.expression) : ""
        ));
    }
    //~ GENERATOR -> 5 TENSOR GENERATION
    tensor(tensor: parser.Tensor): string {
        return String(new latex.Tensor(
            tensor.values.map(value => this.expression(value))
        ));
    }
    //~ GENERATOR -> 5 NUMBER GENERATION
    number(number: parser.Number): string {
        return String(new latex.Number(
            number.value,
            number.shift
        ));
    }
}