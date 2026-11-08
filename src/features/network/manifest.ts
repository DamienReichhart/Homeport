import { WaypointsIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { IngressDetailPage } from './ui/IngressDetailPage';
import { IngressesPage } from './ui/IngressesPage';
import { ServiceDetailPage } from './ui/ServiceDetailPage';
import { ServicesPage } from './ui/ServicesPage';

export const networkFeature = defineFeature({
  routes: [
    { path: 'services', Component: ServicesPage },
    { path: 'services/:namespace/:name/:tab?', Component: ServiceDetailPage },
    { path: 'ingresses', Component: IngressesPage },
    { path: 'ingresses/:namespace/:name/:tab?', Component: IngressDetailPage },
  ],
  nav: { path: '/services', label: 'network.nav', icon: WaypointsIcon, scope: 'cluster' },
});
