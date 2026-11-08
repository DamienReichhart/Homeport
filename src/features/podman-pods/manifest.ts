import { GroupIcon, PlusIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { PodPage } from './ui/PodPage';
import { PodsPage } from './ui/PodsPage';

export const podmanPodsFeature = defineFeature({
  requires: 'podman',
  routes: [
    { path: 'podman-pods', Component: PodsPage },
    { path: 'podman-pods/:id', Component: PodPage },
  ],
  nav: { path: '/podman-pods', label: 'podmanpods.nav', icon: GroupIcon, scope: 'host' },
  commands: [
    {
      id: 'podman-pods:new',
      label: 'podmanpods.new',
      icon: PlusIcon,
      keywords: ['podman', 'pod', 'create'],
      needsHost: true,
      run: (ctx) => ctx.open('/podman-pods', 'new'),
    },
  ],
});
