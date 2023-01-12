import { describe, test, expect } from '@jest/globals';

import { T } from './buffer';

describe('buffer', () => {
    describe('enum T', () => {
        test.each([
            { expected: 0, input: T.I8 },
            { expected: 1, input: T.U8 },
            { expected: 2, input: T.I16 },
            { expected: 3, input: T.U16 },
            { expected: 4, input: T.I32 },
            { expected: 5, input: T.U32 },
            { expected: 6, input: T.I64 },
            { expected: 7, input: T.U64 },
            { expected: 8, input: T.F32 },
            { expected: 9, input: T.F64 },
        ])('definition::case_$#', ({ expected, input }) => {
            expect(input).toBe(expected);
        });
    });
});
