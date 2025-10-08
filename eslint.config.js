import js from '@eslint/js';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import svelte from 'eslint-plugin-svelte';
import svelteParser from 'svelte-eslint-parser';
import boundaries from 'eslint-plugin-boundaries';

export default [
  js.configs.recommended,
  {
    files: ['main.js', 'preload.js', 'MinecraftInstallerService.js'],
    languageOptions: {
      ecmaVersion: 2020,
      sourceType: 'module',
      globals: {
        console: 'readonly',
        process: 'readonly',
        __dirname: 'readonly',
        __filename: 'readonly',
        Buffer: 'readonly',
        global: 'readonly',
        require: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly',
      },
    },
  },
  {
    files: ['**/*.cjs'],
    languageOptions: {
      ecmaVersion: 2020,
      sourceType: 'commonjs',
      globals: {
        console: 'readonly',
        process: 'readonly',
        __dirname: 'readonly',
        __filename: 'readonly',
        Buffer: 'readonly',
        global: 'readonly',
        require: 'readonly',
        module: 'readonly',
        exports: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly',
      },
    },
  },
  {
    files: ['**/*.{js,ts,svelte}'],
    ignores: ['main.js', 'preload.js', 'MinecraftInstallerService.js'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 2020,
        sourceType: 'module',
        extraFileExtensions: ['.svelte'],
      },
      globals: {
        document: 'readonly',
        console: 'readonly',
        window: 'readonly',
        process: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly',
      },
    },
    plugins: {
      '@typescript-eslint': ts,
      boundaries,
    },
    rules: {
      ...ts.configs.recommended.rules,
      'boundaries/element-types': [
        'error',
        {
          default: 'disallow',
          rules: [
            {
              from: 'app',
              allow: ['features', 'shared', 'lib'],
            },
            {
              from: 'features',
              allow: [
                ['features', { featureName: '${from.featureName}' }],
                'shared',
                'lib',
              ],
            },
            {
              from: 'shared',
              allow: ['shared', 'lib'],
            },
            {
              from: 'lib',
              allow: ['lib'],
            },
          ],
        },
      ],
    },
    settings: {
      'boundaries/elements': [
        {
          type: 'app',
          pattern: 'src/app',
        },
        {
          type: 'features',
          pattern: 'src/features/*',
          capture: ['featureName'],
        },
        {
          type: 'shared',
          pattern: 'src/shared',
        },
        {
          type: 'lib',
          pattern: 'src/lib',
        },
      ],
    },
  },
  {
    files: ['**/*.svelte'],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: tsParser,
      },
    },
    plugins: {
      svelte,
    },
    rules: {
      ...svelte.configs.recommended.rules,
    },
  },
];
