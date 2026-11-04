import js from '@eslint/js';
import globals from 'globals';
import { createTypeScriptImportResolver } from 'eslint-import-resolver-typescript';
import importX from 'eslint-plugin-import-x';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import tseslint from 'typescript-eslint';

// Architecture guards (docs/ARCHITECTURE.md, ADR 0004). Imports point
// app → features → core → shared, and modules are reached through their
// public index only.
const publicApiOnly = [
  {
    group: ['@/features/*/*'],
    message: 'Import a feature through its public API (@/features/<name>) only.',
  },
  {
    group: ['@/core/*/*'],
    message: 'Import a core module through its public API (@/core/<name>) only.',
  },
  {
    group: ['@/shared/ui/*'],
    message: 'Import UI components from the kit (@/shared/ui) only.',
  },
];

const restrict = (patterns, paths = []) => [
  'error',
  { patterns: [...publicApiOnly, ...patterns], paths },
];

export default tseslint.config(
  // website/ is its own Astro project, with its own eslint.config.js.
  {
    ignores: [
      'dist',
      'target',
      'src-tauri',
      'node_modules',
      'src/shared/api/bindings.ts',
      'website',
    ],
  },
  {
    files: ['**/*.{ts,tsx}'],
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    languageOptions: { globals: globals.browser },
    plugins: { 'react-hooks': reactHooks, 'react-refresh': reactRefresh, 'import-x': importX },
    settings: {
      ...importX.flatConfigs.typescript.settings,
      'import-x/resolver-next': [createTypeScriptImportResolver()],
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],
      '@typescript-eslint/consistent-type-imports': 'error',
      'import-x/no-cycle': 'error',
      'no-restricted-imports': restrict([]),
    },
  },
  {
    // Past 300 lines a file usually holds more than one thing: split it.
    files: ['src/**/*.{ts,tsx}'],
    ignores: ['**/*.test.{ts,tsx}'],
    rules: { 'max-lines': ['warn', { max: 300, skipBlankLines: true, skipComments: true }] },
  },
  {
    files: ['src/core/**/*.{ts,tsx}'],
    rules: {
      'no-restricted-imports': restrict([
        {
          group: ['@/features/*', '@/app/*'],
          message: 'core/ must not depend on features/ or app/.',
        },
      ]),
    },
  },
  {
    files: ['src/shared/**/*.{ts,tsx}'],
    rules: {
      'no-restricted-imports': restrict([
        {
          group: ['@/features/*', '@/core/*', '@/app/*'],
          message: 'shared/ must not depend on features/, core/ or app/.',
        },
      ]),
    },
  },
  {
    // Components render; talking to the backend belongs in the feature's api/.
    files: ['src/features/*/ui/**/*.{ts,tsx}'],
    ignores: ['**/*.test.{ts,tsx}'],
    rules: {
      'no-restricted-imports': restrict(
        [],
        [
          {
            name: '@/shared/api',
            importNames: ['commands', 'subscribe'],
            message: 'Call the backend from the feature’s api/ folder, not from components.',
          },
        ],
      ),
    },
  },
  {
    // Vendored shadcn/ui components co-export their cva variants.
    files: ['src/shared/ui/**/*.{ts,tsx}'],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
  {
    // Test helpers are never hot-reloaded.
    files: ['src/test/**/*.{ts,tsx}'],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
);
