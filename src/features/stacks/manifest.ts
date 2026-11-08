import { BoxesIcon, PlusIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { StackDetailPage } from './ui/detail/StackDetailPage';
import { NewStackPage } from './ui/new/NewStackPage';
import { StacksPage } from './ui/StacksPage';

export const stacksFeature = defineFeature({
  routes: [
    { path: 'stacks', Component: StacksPage },
    { path: 'stacks/new', Component: NewStackPage },
    { path: 'stacks/:name', Component: StackDetailPage },
  ],
  nav: { path: '/stacks', label: 'stacks.nav', icon: BoxesIcon },
  commands: [
    {
      id: 'stacks:new',
      label: 'stacks.palette.new',
      icon: PlusIcon,
      keywords: ['compose', 'create', 'deploy'],
      needsHost: true,
      run: (ctx) => ctx.open('/stacks/new'),
    },
  ],
});
