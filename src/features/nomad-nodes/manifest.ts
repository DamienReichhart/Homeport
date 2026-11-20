import { ServerIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { NodePage } from './ui/NodePage';
import { NodesPage } from './ui/NodesPage';

export const nomadNodesFeature = defineFeature({
  routes: [splitRoutes('nomad/nodes', NodesPage, [{ path: ':nodeId', Component: NodePage }])],
  nav: { path: '/nomad/nodes', label: 'nomad.nodes.nav', icon: ServerIcon, scope: 'cluster' },
  clusterKind: 'nomad',
});
