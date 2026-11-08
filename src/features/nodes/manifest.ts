import { ServerIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { NodeDetailPage } from './ui/NodeDetailPage';
import { NodesPage } from './ui/NodesPage';

export const nodesFeature = defineFeature({
  routes: [
    { path: 'nodes', Component: NodesPage },
    { path: 'nodes/:name/:tab?', Component: NodeDetailPage },
  ],
  nav: { path: '/nodes', label: 'nodes.nav', icon: ServerIcon, scope: 'cluster' },
});
