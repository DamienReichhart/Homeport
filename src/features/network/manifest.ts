import { WaypointsIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { IngressDetailPage } from './ui/IngressDetailPage';
import { IngressesPage } from './ui/IngressesPage';
import { ServiceDetailPage } from './ui/ServiceDetailPage';
import { ServicesPage } from './ui/ServicesPage';

export const networkFeature = defineFeature({
  routes: [
    splitRoutes('services', ServicesPage, [
      { path: ':namespace/:name/:tab?', Component: ServiceDetailPage },
    ]),
    splitRoutes('ingresses', IngressesPage, [
      { path: ':namespace/:name/:tab?', Component: IngressDetailPage },
    ]),
  ],
  nav: { path: '/services', label: 'network.nav', icon: WaypointsIcon, scope: 'cluster' },
  clusterKind: 'kubernetes',
});
