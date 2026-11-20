import { BriefcaseIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { AllocationPage } from './ui/allocation/AllocationPage';
import { JobPage } from './ui/JobPage';
import { JobsPage } from './ui/JobsPage';

export const nomadJobsFeature = defineFeature({
  routes: [
    splitRoutes('nomad/jobs', JobsPage, [{ path: ':namespace/:jobId', Component: JobPage }]),
    { path: 'nomad/allocations/:allocId/:tab?', Component: AllocationPage },
  ],
  nav: { path: '/nomad/jobs', label: 'nomad.jobs.nav', icon: BriefcaseIcon, scope: 'cluster' },
  clusterKind: 'nomad',
});
