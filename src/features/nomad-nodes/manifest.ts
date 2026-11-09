import { ServerIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { NodePage } from './ui/NodePage';
import { NodesPage } from './ui/NodesPage';

export const nomadNodesFeature = defineFeature({
  routes: [
    { path: 'nomad/nodes', Component: NodesPage },
    { path: 'nomad/nodes/:nodeId', Component: NodePage },
  ],
  nav: { path: '/nomad/nodes', label: 'nomad.nodes.nav', icon: ServerIcon, scope: 'cluster' },
  clusterKind: 'nomad',
});
