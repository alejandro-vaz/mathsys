//^
//^ INTERFACE
//^

//> INTERFACE -> DEFINITION
export interface BinaryEncodable {
    binary(): Binary;
}


//^
//^ TYPES
//^

//> TYPES -> BINARY
export class Binary {
    constructor(
        readonly value: bigint,
        readonly width: number
    ) {}
    __bytes__(): Uint8Array {
        if (this.width % 8 !== 0) {throw new RangeError(`Cannot convert Binary to Uint8Array: width ${this.width} is not a multiple of 8`)};
        const width = this.width / 8;
        const bytes = new Uint8Array(width);
        let temp = this.value;
        for (let index = 0; index < width; index++) {
            bytes[index] = Number(temp & 0xFFn);
            temp >>= 8n;
        }
        return bytes;
    }
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
    binary(): Binary {if (this.value === null) {
        return new Binary(BigInt(0), 1)
    } else {
        return join(new Binary(BigInt(1), 1), this.value)
    }}
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
        const length = data.length;
        let databig = 0n;
        for (let index = 0; index < length; index++) {databig |= BigInt(data[index]) << BigInt(8 * index)}
        return join(
            new Binary(BigInt(length), 16),
            new Binary(databig, length * 8)
        )
    }
}

//> TYPES -> GROUP
export class Group implements BinaryEncodable {
    constructor(
        readonly value: string | null
    ) {}
    binary(): Binary {return new Binary(BigInt({
        null: 0,
        "@Infinite": 1,
        "@Integer": 2,
        "@Natural": 3,
        "@Nexists": 4,
        "@Rational": 5,
        "@Tensor": 6,
        "@Undefined": 0,
        "@Variable": 7,
        "@Whole": 8
    }[this.value ?? "null"] as number), 4)}
}

//> TYPES -> VEC
export class Vec<Type extends BinaryEncodable> implements BinaryEncodable {
    constructor(
        readonly values: (Binary | BinaryEncodable)[]
    ) {}
    binary(): Binary {return join(new Binary(BigInt(this.values.length), 32), ...this.values)}
}


//^
//^ HELPERS
//^

//> HELPERS -> JOIN
export function join(...binaries: (Binary | BinaryEncodable)[]): Binary {
    let value = 0n;
    let width = 0;
    for (let binary of binaries) {
        binary = binary instanceof Binary ? binary : binary.binary();
        value |= binary.value << BigInt(width);
        width += binary.width;
    }
    return new Binary(value, width);
}