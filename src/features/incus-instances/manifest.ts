import { PlusIcon, ServerIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { InstancePage } from './ui/InstancePage';
import { InstancesPage } from './ui/InstancesPage';

export const incusInstancesFeature = defineFeature({
  requires: 'incus',
  routes: [
    splitRoutes('incus/instances', InstancesPage, [
      { path: ':name/:tab?', Component: InstancePage },
    ]),
  ],
  nav: {
    path: '/incus/instances',
    label: 'incus.instances.nav',
    icon: ServerIcon,
    scope: 'host',
  },
  commands: [
    {
      id: 'incus-instances:new',
      label: 'incus.new.action',
      icon: PlusIcon,
      keywords: ['incus', 'instance', 'container', 'virtual machine', 'launch', 'create'],
      needsHost: true,
      run: (ctx) => ctx.open('/incus/instances', 'new'),
    },
  ],
});
