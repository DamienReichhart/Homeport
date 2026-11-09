import { ShipWheelIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ReleasePage } from './ui/release/ReleasePage';
import { ReleasesPage } from './ui/ReleasesPage';
import { RepositoriesPage } from './ui/repositories/RepositoriesPage';

export const helmFeature = defineFeature({
  routes: [
    { path: 'helm', Component: ReleasesPage },
    { path: 'helm/repositories', Component: RepositoriesPage },
    { path: 'helm/:namespace/:name/:tab?', Component: ReleasePage },
  ],
  nav: { path: '/helm', label: 'helm.nav', icon: ShipWheelIcon, scope: 'cluster' },
  clusterKind: 'kubernetes',
});
