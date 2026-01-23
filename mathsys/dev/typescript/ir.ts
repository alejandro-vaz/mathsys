//^
//^ HEAD
//^

//> HEAD -> MODULES
import pako from "pako";


//^ 
//^ HELPERS
//^ 

//> HELPERS -> NODE
export function node(node: Binary, binary: Binary[], nodes: Binary[]): Pointer {
    const index = nodes.indexOf(node);
    if (index !== -1) {return new Pointer(index)}
    nodes.push(node);
    binary.push(node);
    return new Pointer(nodes.length - 1);
}


//^
//^ PROTOCOL
//^

//> PROTOCOL -> DEFINITION
interface BinaryEncodable {
    binary(): Binary;
}


//^
//^ TYPES
//^

//> TYPES -> BINARY
export class Binary implements BinaryEncodable {
    constructor(
        readonly value: bigint,
        readonly width: number
    ) {}
    toBytes(): Uint8Array {
        let value = new Binary(this.value, this.width);
        while (value.width % 8 !== 0) {value = value.add(new Opcode(0x00).binary())}
        const byteLength = value.width / 8;
        const bytes = new Uint8Array(byteLength);
        let val = value.value;
        for (let i = 0; i < byteLength; i++) {
            bytes[i] = Number(val & 0xffn);
            val >>= 8n;
        }
        return pako.deflateRaw(bytes, { level: 9 });
    }
    add(...others: (Binary | BinaryEncodable)[]): Binary {
        return others.reduce((accumulated: Binary, other: Binary | BinaryEncodable) => {
            const transformed = other instanceof Binary ? other : other.binary();
            return new Binary(
                accumulated.value | (transformed.value << BigInt(accumulated.width)),
                accumulated.width + transformed.width
            );
        }, this)
    }
    binary(): Binary {return this}
}

//> TYPES -> OPCODE
export class Opcode implements BinaryEncodable {
    constructor(
        readonly value: number
    ) {}
    binary(): Binary {return new Binary(BigInt(this.value), 5)}
}

//> TYPES -> POINTER
export class Pointer implements BinaryEncodable {
    constructor(
        readonly value: number
    ) {}
    binary(): Binary {return new Binary(BigInt(this.value), 32)}
}

//> TYPES -> SIGN
export class Sign implements BinaryEncodable {
    constructor(
        readonly value: boolean
    ) {}
    binary(): Binary {return new Binary(BigInt(this.value ? 1 : 0), 1)}
}

//> TYPES -> OPTION
export class Option<Type extends BinaryEncodable> implements BinaryEncodable {
    constructor(
        readonly value: Binary | BinaryEncodable | null
    ) {}
    binary(): Binary {return this.value === null ? new Binary(BigInt(0), 1) : new Binary(BigInt(1), 1).add(this.value)}
}

//> TYPES -> BIGUINT
export class BigUint implements BinaryEncodable {
    constructor(
        readonly value: bigint
    ) {}
    binary(): Binary {return new Binary(this.value, 128)}
}

//> TYPES -> STRING
export class String implements BinaryEncodable {
    constructor(
        readonly value: string
    ) {}
    binary(): Binary {
        const data = new TextEncoder().encode(this.value);
        const lengthBinary = new Binary(BigInt(data.length), 16);
        let valueBinary = new Binary(BigInt(0), data.length * 8);
        for (let i = 0; i < data.length; i++) {
            valueBinary = valueBinary.add(new Binary(BigInt(data[i]), 8));
        }
        return lengthBinary.add(valueBinary);
    }
}

//> TYPES -> GROUP
export class Group implements BinaryEncodable {
    constructor(
        readonly value: string
    ) {}
    binary(): Binary {return new Binary(
        BigInt({
            "@Infinite": 1,
            "@Integer": 2,
            "@Natural": 3,
            "@Nexists": 4,
            "@Rational": 5,
            "@Tensor": 6,
            "@Undefined": 0,
            "@Variable": 7,
            "@Whole": 8,
        }[this.value] as number),
        4
    )}
}

//> TYPES -> VEC
export class Vec<Type extends BinaryEncodable> implements BinaryEncodable {
    constructor(
        readonly values: (Binary | BinaryEncodable)[]
    ) {}
    binary(): Binary {
        let value = new Binary(BigInt(this.values.length), 32);
        for (const item of this.values) {
            value = value.add(item instanceof Binary ? item : item.binary());
        }
        return value;
    }
}