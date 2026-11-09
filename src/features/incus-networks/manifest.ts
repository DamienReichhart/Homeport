import { NetworkIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { NetworkPage } from './ui/NetworkPage';
import { NetworksPage } from './ui/NetworksPage';

export const incusNetworksFeature = defineFeature({
  requires: 'incus',
  routes: [
    { path: 'incus/networks', Component: NetworksPage },
    { path: 'incus/networks/:name', Component: NetworkPage },
  ],
  nav: { path: '/incus/networks', label: 'incus.networks.nav', icon: NetworkIcon, scope: 'host' },
});
