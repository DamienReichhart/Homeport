import type { StackTemplate } from './template';

export const empty: StackTemplate = {
  id: 'empty',
  title: 'stacks.templates.empty.title',
  description: 'stacks.templates.empty.description',
  name: '',
  compose: `services:
`,
  env: '',
};
