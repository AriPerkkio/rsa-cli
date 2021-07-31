module.exports = {
    root: true,
    env: {
        node: true,
        es6: true,
    },
    globals: {
        BigInt: true,
    },
    parserOptions: {
        ecmaVersion: 2020,
        sourceType: 'module',
    },
    parser: '@typescript-eslint/parser',
    plugins: ['prettier', '@typescript-eslint/eslint-plugin'],
    extends: [
        'eslint:recommended',
        'plugin:prettier/recommended',
        'plugin:@typescript-eslint/eslint-plugin/recommended',
    ],
    rules: {
        'prettier/prettier': 'error',
    },
};
