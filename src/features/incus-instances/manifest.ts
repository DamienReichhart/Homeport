import { ServerIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { InstancePage } from './ui/InstancePage';
import { InstancesPage } from './ui/InstancesPage';

export const incusInstancesFeature = defineFeature({
  requires: 'incus',
  routes: [
    { path: 'incus/instances', Component: InstancesPage },
    { path: 'incus/instances/:name/:tab?', Component: InstancePage },
  ],
  nav: {
    path: '/incus/instances',
    label: 'incus.instances.nav',
    icon: ServerIcon,
    scope: 'host',
  },
});
