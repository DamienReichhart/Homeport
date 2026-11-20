import { BoxIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { ContainerPage } from './ui/ContainerPage';
import { ContainersPage } from './ui/ContainersPage';

export const containerdContainersFeature = defineFeature({
  requires: 'containerd',
  routes: [
    splitRoutes('containerd/containers', ContainersPage, [
      { path: ':id/:tab?', Component: ContainerPage },
    ]),
  ],
  nav: {
    path: '/containerd/containers',
    label: 'containerd.containers.nav',
    icon: BoxIcon,
    scope: 'host',
  },
});
