import { NetworkIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { NetworkPage } from './ui/NetworkPage';
import { NetworksPage } from './ui/NetworksPage';

export const incusNetworksFeature = defineFeature({
  requires: 'incus',
  routes: [
    splitRoutes('incus/networks', NetworksPage, [{ path: ':name', Component: NetworkPage }]),
  ],
  nav: { path: '/incus/networks', label: 'incus.networks.nav', icon: NetworkIcon, scope: 'host' },
});
