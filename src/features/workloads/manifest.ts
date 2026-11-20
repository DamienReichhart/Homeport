import { LayersIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { WorkloadDetailPage } from './ui/detail/WorkloadDetailPage';
import { WorkloadsPage } from './ui/WorkloadsPage';

export const workloadsFeature = defineFeature({
  routes: [
    splitRoutes('workloads/:kind?', WorkloadsPage, [
      { path: ':namespace/:name/:tab?', Component: WorkloadDetailPage },
    ]),
  ],
  nav: { path: '/workloads', label: 'workloads.nav', icon: LayersIcon, scope: 'cluster' },
  clusterKind: 'kubernetes',
});
