import { HardDriveIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { VolumesPage } from './VolumesPage';

export const volumesFeature = defineFeature({
  routes: [{ path: 'volumes', Component: VolumesPage }],
  nav: { path: '/volumes', label: 'Volumes', icon: HardDriveIcon },
});
