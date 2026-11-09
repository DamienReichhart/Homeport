import { FileCode2Icon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ApplyPage } from './ui/ApplyPage';

export const applyFeature = defineFeature({
  routes: [{ path: 'apply', Component: ApplyPage }],
  nav: { path: '/apply', label: 'apply.nav', icon: FileCode2Icon, scope: 'cluster' },
  clusterKind: 'kubernetes',
});
