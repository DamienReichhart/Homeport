import js from '@eslint/js';
import globals from 'globals';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  // website/ is its own Astro project, checked by `astro check` in its build.
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
    plugins: { 'react-hooks': reactHooks, 'react-refresh': reactRefresh },
    rules: {
      ...reactHooks.configs.recommended.rules,
      'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],
      '@typescript-eslint/consistent-type-imports': 'error',
      // Architecture guard: features must not reach into each other's internals,
      // and shared code must not depend on features.
      'no-restricted-imports': [
        'error',
        {
          patterns: [
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
          ],
        },
      ],
    },
  },
  {
    files: ['src/shared/**/*.{ts,tsx}'],
    rules: {
      'no-restricted-imports': [
        'error',
        {
          patterns: [
            {
              group: ['@/features/*', '@/core/*', '@/app/*'],
              message: 'shared/ must not depend on features/, core/ or app/.',
            },
          ],
        },
      ],
    },
  },
  {
    files: ['src/core/**/*.{ts,tsx}'],
    rules: {
      'no-restricted-imports': [
        'error',
        {
          patterns: [
            {
              group: ['@/features/*', '@/app/*'],
              message: 'core/ must not depend on features/ or app/.',
            },
            {
              group: ['@/core/*/*'],
              message: 'Import a core module through its public API (@/core/<name>) only.',
            },
            {
              group: ['@/shared/ui/*'],
              message: 'Import UI components from the kit (@/shared/ui) only.',
            },
          ],
        },
      ],
    },
  },
  {
    // Vendored shadcn/ui components co-export their cva variants.
    files: ['src/shared/ui/**/*.{ts,tsx}'],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
);
