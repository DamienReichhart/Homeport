import { ArrowRightLeftIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { PortForwardsPage } from './ui/PortForwardsPage';

export const portForwardsFeature = defineFeature({
  routes: [{ path: 'port-forwards', Component: PortForwardsPage }],
  nav: {
    path: '/port-forwards',
    label: 'portforwards.nav',
    icon: ArrowRightLeftIcon,
    scope: 'cluster',
  },
});
