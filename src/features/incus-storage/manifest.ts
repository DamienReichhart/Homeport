import { HardDriveIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { StoragePage } from './ui/StoragePage';

export const incusStorageFeature = defineFeature({
  requires: 'incus',
  routes: [{ path: 'incus/storage/:pool?', Component: StoragePage }],
  nav: { path: '/incus/storage', label: 'incus.storage.nav', icon: HardDriveIcon, scope: 'host' },
});
