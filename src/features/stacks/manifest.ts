import { BoxesIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { NewStackPage } from './NewStackPage';
import { StackDetailPage } from './StackDetailPage';
import { StacksPage } from './StacksPage';

export const stacksFeature = defineFeature({
  routes: [
    { path: 'stacks', Component: StacksPage },
    { path: 'stacks/new', Component: NewStackPage },
    { path: 'stacks/:name', Component: StackDetailPage },
  ],
  nav: { path: '/stacks', label: 'Stacks', icon: BoxesIcon },
});
