import { BlocksIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ApiObjectPage } from './ui/ApiObjectPage';
import { ApiResourcesPage } from './ui/ApiResourcesPage';

export const apiResourcesFeature = defineFeature({
  routes: [
    { path: 'api-resources', Component: ApiResourcesPage },
    { path: 'api-resources/:group/:version/:plural', Component: ApiResourcesPage },
    {
      path: 'api-resources/:group/:version/:plural/:namespace/:name/:tab?',
      Component: ApiObjectPage,
    },
  ],
  nav: {
    path: '/api-resources',
    label: 'apiresources.nav',
    icon: BlocksIcon,
    scope: 'cluster',
  },
  clusterKind: 'kubernetes',
});
