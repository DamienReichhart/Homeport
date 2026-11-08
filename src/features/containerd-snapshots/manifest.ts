import { HardDriveIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { SnapshotsPage } from './ui/SnapshotsPage';

export const containerdSnapshotsFeature = defineFeature({
  requires: 'containerd',
  routes: [{ path: 'containerd/snapshots', Component: SnapshotsPage }],
  nav: {
    path: '/containerd/snapshots',
    label: 'containerd.snapshots.nav',
    icon: HardDriveIcon,
    scope: 'host',
  },
});
