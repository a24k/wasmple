import { describe, test, expect } from '@jest/globals';

import { LogLevel } from './console';

describe('console', () => {
    describe('enum LogLevel', () => {
        test.each([
            { expected: 0, input: LogLevel.Log },
            { expected: 1, input: LogLevel.Debug },
            { expected: 2, input: LogLevel.Info },
            { expected: 3, input: LogLevel.Warn },
            { expected: 4, input: LogLevel.Error },
        ])('definition::case_$#', ({ expected, input }) => {
            expect(input).toBe(expected);
        });
    });
});
