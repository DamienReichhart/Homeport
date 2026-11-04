import js from '@eslint/js';
import { createTypeScriptImportResolver } from 'eslint-import-resolver-typescript';
import { defineConfig } from 'eslint/config';
import astro from 'eslint-plugin-astro';
import importX from 'eslint-plugin-import-x';
import globals from 'globals';
import tseslint from 'typescript-eslint';

// Architecture guards (ADR 0006, the app's layers from ADR 0004). Imports point
// app → features → core → shared, and modules are reached through their public
// index only. pages/ and content/ are the routes: they may import any layer.
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

const restrict = (patterns) => ['error', { patterns: [...publicApiOnly, ...patterns] }];

const sources = '**/*.{js,mjs,ts,astro}';

export default defineConfig(
  { ignores: ['dist', '.astro', 'node_modules'] },
  {
    files: [sources],
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    plugins: { 'import-x': importX },
    settings: {
      ...importX.flatConfigs.typescript.settings,
      'import-x/resolver-next': [createTypeScriptImportResolver()],
    },
    rules: {
      '@typescript-eslint/consistent-type-imports': 'error',
      'import-x/no-cycle': 'error',
      'no-restricted-imports': restrict([]),
    },
  },
  {
    // Build-time code (configs, scripts) runs on Node or Bun.
    files: ['*.{js,mjs,ts}', 'scripts/**'],
    languageOptions: { globals: globals.node },
  },
  ...astro.configs.recommended,
  {
    // Follow imports into .astro files from .astro files, or a cycle through
    // components goes unseen; it is reported on the cycle's components. Not
    // through import-x/parsers, which require()s the parser (astro-eslint-parser
    // is ESM only): with no entry there, import-x parses an imported .astro file
    // with the linting file's parser, which only an .astro file has.
    files: ['**/*.astro'],
    settings: {
      'import-x/extensions': [
        ...importX.flatConfigs.typescript.settings['import-x/extensions'],
        '.astro',
      ],
    },
  },
  {
    // Past 300 lines a file usually holds more than one thing: split it.
    files: [`src/${sources}`],
    rules: { 'max-lines': ['warn', { max: 300, skipBlankLines: true, skipComments: true }] },
  },
  {
    files: [`src/features/${sources}`],
    rules: {
      'no-restricted-imports': restrict([
        { group: ['@/app/*'], message: 'features/ must not depend on app/.' },
      ]),
    },
  },
  {
    files: [`src/core/${sources}`],
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
    files: [`src/shared/${sources}`],
    rules: {
      'no-restricted-imports': restrict([
        {
          group: ['@/features/*', '@/core/*', '@/app/*'],
          message: 'shared/ must not depend on features/, core/ or app/.',
        },
      ]),
    },
  },
);
