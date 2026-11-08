import { ContainerIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { PodDetailPage } from './ui/detail/PodDetailPage';
import { PodsPage } from './ui/PodsPage';

export const podsFeature = defineFeature({
  routes: [
    { path: 'pods', Component: PodsPage },
    { path: 'pods/:namespace/:name/:tab?', Component: PodDetailPage },
  ],
  nav: { path: '/pods', label: 'pods.nav', icon: ContainerIcon, scope: 'cluster' },
});
