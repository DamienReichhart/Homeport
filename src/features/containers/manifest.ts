import { BoxIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ContainerDetailPage } from './components/ContainerDetailPage';
import { ContainersPage } from './components/ContainersPage';

export const containersFeature = defineFeature({
  routes: [
    { path: 'containers', Component: ContainersPage },
    { path: 'containers/:id/:tab?', Component: ContainerDetailPage },
  ],
  nav: { path: '/containers', label: 'Containers', icon: BoxIcon },
});
