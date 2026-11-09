import { FileCogIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ConfigMapDetailPage, SecretDetailPage } from './ui/ConfigDetailPages';
import { ConfigMapsPage } from './ui/ConfigMapsPage';
import { SecretsPage } from './ui/SecretsPage';

export const configFeature = defineFeature({
  routes: [
    { path: 'configmaps', Component: ConfigMapsPage },
    { path: 'configmaps/:namespace/:name/:tab?', Component: ConfigMapDetailPage },
    { path: 'secrets', Component: SecretsPage },
    { path: 'secrets/:namespace/:name/:tab?', Component: SecretDetailPage },
  ],
  nav: { path: '/configmaps', label: 'config.nav', icon: FileCogIcon, scope: 'cluster' },
  clusterKind: 'kubernetes',
});
