import { BoxIcon, PlayIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { ContainerDetailPage } from './ui/detail/ContainerDetailPage';
import { ContainersPage } from './ui/list/ContainersPage';

export const containersFeature = defineFeature({
  requires: 'docker',
  routes: [
    splitRoutes('containers', ContainersPage, [
      { path: ':id/:tab?', Component: ContainerDetailPage },
    ]),
  ],
  nav: { path: '/containers', label: 'containers.nav', icon: BoxIcon, scope: 'host' },
  commands: [
    {
      id: 'containers:run',
      label: 'containers.palette.run',
      icon: PlayIcon,
      keywords: ['new', 'start', 'create'],
      needsHost: true,
      run: (ctx) => ctx.open('/containers', 'run'),
    },
  ],
});
