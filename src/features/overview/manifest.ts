import { LayoutDashboardIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { OverviewPage } from './OverviewPage';

export const overviewFeature = defineFeature({
  routes: [{ index: true, Component: OverviewPage }],
  nav: { path: '/', label: 'Overview', icon: LayoutDashboardIcon },
});
