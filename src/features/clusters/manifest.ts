import { PlusIcon, ShipWheelIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { AddClusterPage } from './ui/add/AddClusterPage';
import { ClustersPage } from './ui/ClustersPage';
import { ClusterDetailPage } from './ui/detail/ClusterDetailPage';

// No sidebar entry: the switcher leads here, as for Hosts.
export const clustersFeature = defineFeature({
  routes: [
    splitRoutes('clusters', ClustersPage, [{ path: ':clusterId', Component: ClusterDetailPage }]),
    { path: 'clusters/new', Component: AddClusterPage },
  ],
  commands: [
    {
      id: 'clusters:add',
      label: 'clusters.palette.add',
      icon: PlusIcon,
      keywords: ['kubernetes', 'k8s', 'kubeconfig', 'context', 'new'],
      run: (ctx) => ctx.open('/clusters/new'),
    },
    {
      id: 'clusters:manage',
      label: 'clusters.palette.manage',
      icon: ShipWheelIcon,
      keywords: ['kubernetes', 'k8s', 'clusters'],
      run: (ctx) => ctx.open('/clusters'),
    },
  ],
});
