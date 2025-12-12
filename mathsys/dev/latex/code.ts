//^
//^ HEAD
//^

//> HEAD -> DATA
import {types} from "./local";
import * as latex from "./dataclasses";
import * as parser from "../parser/dataclasses";


//^
//^ LATEX
//^

//> LATEX -> GENERATOR
class Latex {
    //~ GENERATOR -> CONSTRUCTOR
    constructor(start: parser.Start) {}
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
}