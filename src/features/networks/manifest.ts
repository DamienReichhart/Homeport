import { NetworkIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { NetworksPage } from './ui/NetworksPage';

export const networksFeature = defineFeature({
  routes: [{ path: 'networks', Component: NetworksPage }],
  nav: { path: '/networks', label: 'networks.nav', icon: NetworkIcon },
});
