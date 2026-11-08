import { HardDriveIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { VolumesPage } from './ui/VolumesPage';

export const volumesFeature = defineFeature({
  requires: 'docker',
  routes: [{ path: 'volumes', Component: VolumesPage }],
  nav: { path: '/volumes', label: 'volumes.nav', icon: HardDriveIcon, scope: 'host' },
});
