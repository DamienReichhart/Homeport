import { ContainerIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { PodDetailPage } from './ui/detail/PodDetailPage';
import { PodsPage } from './ui/PodsPage';

export const podsFeature = defineFeature({
  routes: [
    splitRoutes('pods', PodsPage, [{ path: ':namespace/:name/:tab?', Component: PodDetailPage }]),
  ],
  nav: { path: '/pods', label: 'pods.nav', icon: ContainerIcon, scope: 'cluster' },
  clusterKind: 'kubernetes',
});
