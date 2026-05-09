import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import solid from 'eslint-plugin-solid';

export default tseslint.config(
    js.configs.recommended,
    ...tseslint.configs.recommended,
    {
        plugins: { solid },
        rules: {
            ...solid.configs.typescript.rules,
            semi: 'warn',
        },
        languageOptions: {
            parserOptions: {
                ecmaVersion: 'latest',
                sourceType: 'module',
            },
            globals: {
                window: 'readonly',
                document: 'readonly',
                navigator: 'readonly',
            },
        },
    },
    {
        ignores: ['node_modules/**', 'dist/**', 'target/**'],
    },
);
