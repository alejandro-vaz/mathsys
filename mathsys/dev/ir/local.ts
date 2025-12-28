//^
//^ TYPES
//^

//> TYPES -> U8
export type u8 = Uint8Array;
export function u8(value: number): u8 {
    if (!Number.isInteger(value) || value < 1 || value > 0xff) {
        throw new RangeError(`'${value}' is outside range for u8.`);
    }
    return new Uint8Array([value]);
}

//> TYPES -> NULL8
export type null8 = Uint8Array;
export function null8(): null8 {return new Uint8Array([0])}

//> TYPES -> U32
export type u32 = Uint8Array;
export function u32(value: number): u32 {
    if (!Number.isInteger(value) || value < 1 || value > 0xffffffff) {
      throw new RangeError(`'${value}' is outside range for u32.`);
    }
    return new Uint8Array([
        value & 0xFF,
        (value >> 8) & 0xFF,
        (value >> 16) & 0xFF,
        (value >> 24) & 0xFF
    ]);
}

//> TYPES -> NULL32
export type null32 = Uint8Array;
export function null32(): null32 {return new Uint8Array([0, 0, 0, 0])}


//^
//^ HELPERS
//^

//> HELPERS -> JOIN
export function join(binary: u8[] | u32[], delimiter: Uint8Array): Uint8Array {
    const nullTerminator = delimiter;
    let totalLength = nullTerminator.length;
    for (const item of binary) {
        totalLength += item.length;
    }
    const combined = new Uint8Array(totalLength);
    let offset = 0;
    for (const item of binary) {
        combined.set(item, offset);
        offset += item.length;
    }
    combined.set(nullTerminator, offset);
    return combined;
}

//> HELPERS -> CLAMP
export function clamp(...arrays: Uint8Array[]): Uint8Array {
    let result = new Uint8Array(arrays.reduce((sum, arr) => sum + arr.length, 0));
    let offset = 0;
    for (const arr of arrays) {
        result.set(arr, offset);
        offset += arr.length;
    }
    return result;
}


//^
//^ MAPPINGS
//^

//> MAPPINGS -> OBJECTTYPE
export const OBJECTTYPE: Record<string, Uint8Array> = {
    null: null8(),
    "@Infinite": u8(1),
    "@Integer": u8(2),
    "@Natural": u8(3),
    "@Nexists": u8(4),
    "@Rational": u8(5),
    "@Tensor": u8(6),
    "@Undefined": null8(),
    "@Variable": u8(7),
    "@Whole": u8(8)
}