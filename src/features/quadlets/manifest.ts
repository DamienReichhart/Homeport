import { PlusIcon, ScrollTextIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { QuadletPage } from './ui/QuadletPage';
import { QuadletsPage } from './ui/QuadletsPage';

export const quadletsFeature = defineFeature({
  requires: 'podman',
  routes: [splitRoutes('quadlets', QuadletsPage, [{ path: ':name', Component: QuadletPage }])],
  nav: { path: '/quadlets', label: 'quadlets.nav', icon: ScrollTextIcon, scope: 'host' },
  commands: [
    {
      id: 'quadlets:new',
      label: 'quadlets.new',
      icon: PlusIcon,
      keywords: ['podman', 'quadlet', 'systemd', 'unit', 'install'],
      needsHost: true,
      run: (ctx) => ctx.open('/quadlets', 'new'),
    },
  ],
});
