import { BoxIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ContainerPage } from './ui/ContainerPage';
import { ContainersPage } from './ui/ContainersPage';

export const containerdContainersFeature = defineFeature({
  requires: 'containerd',
  routes: [
    { path: 'containerd/containers', Component: ContainersPage },
    { path: 'containerd/containers/:id/:tab?', Component: ContainerPage },
  ],
  nav: {
    path: '/containerd/containers',
    label: 'containerd.containers.nav',
    icon: BoxIcon,
    scope: 'host',
  },
});
