import { NetworkIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { NetworksPage } from './NetworksPage';

export const networksFeature = defineFeature({
  routes: [{ path: 'networks', Component: NetworksPage }],
  nav: { path: '/networks', label: 'Networks', icon: NetworkIcon },
});
