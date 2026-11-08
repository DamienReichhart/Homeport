import { ActivityIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ClusterEventsPage } from './ui/ClusterEventsPage';

export const clusterEventsFeature = defineFeature({
  routes: [{ path: 'cluster-events', Component: ClusterEventsPage }],
  nav: {
    path: '/cluster-events',
    label: 'clusterevents.nav',
    icon: ActivityIcon,
    scope: 'cluster',
  },
});
