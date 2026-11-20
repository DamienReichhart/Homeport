import { BrushCleaningIcon, GaugeIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { OverviewPage } from './ui/OverviewPage';

export const overviewFeature = defineFeature({
  routes: [{ path: 'overview', Component: OverviewPage }],
  nav: { path: '/overview', label: 'overview.nav', icon: GaugeIcon, scope: 'app' },
  commands: [
    {
      id: 'overview:prune',
      label: 'overview.palette.prune',
      icon: BrushCleaningIcon,
      keywords: ['clean', 'disk', 'space', 'remove unused'],
      needsHost: true,
      run: (ctx) => ctx.open('/overview', 'prune'),
    },
  ],
});
