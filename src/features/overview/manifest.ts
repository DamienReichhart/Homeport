import { BrushCleaningIcon, LayoutDashboardIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { OverviewPage } from './ui/OverviewPage';

export const overviewFeature = defineFeature({
  routes: [{ index: true, Component: OverviewPage }],
  nav: { path: '/', label: 'overview.nav', icon: LayoutDashboardIcon },
  commands: [
    {
      id: 'overview:prune',
      label: 'overview.palette.prune',
      icon: BrushCleaningIcon,
      keywords: ['clean', 'disk', 'space', 'remove unused'],
      needsHost: true,
      run: (ctx) => ctx.open('/', 'prune'),
    },
  ],
});
