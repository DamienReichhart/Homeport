import { FileCogIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { ConfigMapDetailPage, SecretDetailPage } from './ui/ConfigDetailPages';
import { ConfigMapsPage } from './ui/ConfigMapsPage';
import { SecretsPage } from './ui/SecretsPage';

export const configFeature = defineFeature({
  routes: [
    splitRoutes('configmaps', ConfigMapsPage, [
      { path: ':namespace/:name/:tab?', Component: ConfigMapDetailPage },
    ]),
    splitRoutes('secrets', SecretsPage, [
      { path: ':namespace/:name/:tab?', Component: SecretDetailPage },
    ]),
  ],
  nav: { path: '/configmaps', label: 'config.nav', icon: FileCogIcon, scope: 'cluster' },
  clusterKind: 'kubernetes',
});
