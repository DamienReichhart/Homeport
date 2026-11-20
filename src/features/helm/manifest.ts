import { ShipWheelIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { ReleasePage } from './ui/release/ReleasePage';
import { ReleasesPage } from './ui/ReleasesPage';
import { RepositoriesPage } from './ui/repositories/RepositoriesPage';

export const helmFeature = defineFeature({
  routes: [
    splitRoutes('helm', ReleasesPage, [{ path: ':namespace/:name/:tab?', Component: ReleasePage }]),
    { path: 'helm/repositories', Component: RepositoriesPage },
  ],
  nav: { path: '/helm', label: 'helm.nav', icon: ShipWheelIcon, scope: 'cluster', group: 'tools' },
  clusterKind: 'kubernetes',
});
