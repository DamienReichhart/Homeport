import { LayoutDashboardIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { OverviewPage } from './ui/OverviewPage';

export const overviewFeature = defineFeature({
  routes: [{ index: true, Component: OverviewPage }],
  nav: { path: '/', label: 'overview.nav', icon: LayoutDashboardIcon },
});
