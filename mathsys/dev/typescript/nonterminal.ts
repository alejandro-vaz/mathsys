//^
//^ NONTERMINAL
//^

//> NONTERMINAL -> START
export abstract class NonTerminal {
    private frozen: boolean = false;
    constructor(items: any[]) {
        this.create(items);
        this.frozen = true;
        return new Proxy(this, {
            set: (target, property, value) => {
                if (target.frozen) {throw new Error(`Cannot set property '${String(property)}' on frozen instance`)};
                (target as any)[property] = value;
                return true;
            }
        })
    }
    abstract create(items: any[]): void;
    abstract latex(types: Record<string, string>): string;
    abstract ir(binary: Binary[], nodes: Binary[]): Pointer;
}