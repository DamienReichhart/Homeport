import { ServerIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { NodeDetailPage } from './ui/NodeDetailPage';
import { NodesPage } from './ui/NodesPage';

export const nodesFeature = defineFeature({
  routes: [splitRoutes('nodes', NodesPage, [{ path: ':name/:tab?', Component: NodeDetailPage }])],
  nav: { path: '/nodes', label: 'nodes.nav', icon: ServerIcon, scope: 'cluster', group: 'cluster' },
  clusterKind: 'kubernetes',
});
