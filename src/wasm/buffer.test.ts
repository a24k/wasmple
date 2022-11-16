import { describe, test, expect } from '@jest/globals';

import { Type } from './buffer';

describe('buffer', () => {
    describe('enum Type', () => {
        test.each([
            { expected: 0, input: Type.I8 },
            { expected: 1, input: Type.U8 },
            { expected: 2, input: Type.I16 },
            { expected: 3, input: Type.U16 },
            { expected: 4, input: Type.I32 },
            { expected: 5, input: Type.U32 },
            { expected: 6, input: Type.I64 },
            { expected: 7, input: Type.U64 },
            { expected: 8, input: Type.F32 },
            { expected: 9, input: Type.F64 },
        ])('definition::case_$#', ({ expected, input }) => {
            expect(input).toBe(expected);
        });
    });
});
