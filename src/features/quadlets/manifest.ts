import { PlusIcon, ScrollTextIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { QuadletPage } from './ui/QuadletPage';
import { QuadletsPage } from './ui/QuadletsPage';

export const quadletsFeature = defineFeature({
  requires: 'podman',
  routes: [
    { path: 'quadlets', Component: QuadletsPage },
    { path: 'quadlets/:name', Component: QuadletPage },
  ],
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
